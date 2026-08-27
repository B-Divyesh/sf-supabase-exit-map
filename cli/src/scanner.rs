use crate::guidance::finding;
use crate::model::{ChecklistItem, Effort, Evidence, Finding, Report, ScanScope, Summary};
use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum ScanError {
    Missing(PathBuf),
    NotDirectory(PathBuf),
    Io(io::Error),
}

impl std::fmt::Display for ScanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Missing(p) => write!(f, "project path does not exist: {}", p.display()),
            Self::NotDirectory(p) => write!(f, "project path is not a directory: {}", p.display()),
            Self::Io(e) => write!(f, "could not scan project: {e}"),
        }
    }
}

impl std::error::Error for ScanError {}

pub fn scan_project(input: &Path) -> Result<Report, ScanError> {
    if !input.exists() {
        return Err(ScanError::Missing(input.to_path_buf()));
    }
    if !input.is_dir() {
        return Err(ScanError::NotDirectory(input.to_path_buf()));
    }
    let root = locate_supabase_root(input);
    let mut state = State::default();

    let config = root.join("config.toml");
    if config.is_file() {
        state.read(&root, &config, FileKind::Config);
    }
    let migrations = root.join("migrations");
    if migrations.is_dir() {
        walk(&migrations, &mut |p| state.read(&root, p, FileKind::Sql)).map_err(ScanError::Io)?;
    }
    for name in ["seed.sql", "roles.sql"] {
        let path = root.join(name);
        if path.is_file() {
            state.read(&root, &path, FileKind::Sql);
        }
    }
    let functions = root.join("functions");
    if functions.is_dir() {
        walk(&functions, &mut |p| {
            state.read(&root, p, FileKind::Function)
        })
        .map_err(ScanError::Io)?;
    }

    if !config.is_file() {
        state.warnings.push(
            "No config.toml found; dashboard-only Auth and API settings require manual inventory."
                .into(),
        );
    }
    if state.scope.sql_files == 0 {
        state.warnings.push("No SQL migrations or seed files found; schema dependencies may be missing from this report.".into());
    }
    if !functions.is_dir() {
        state.warnings.push("No functions directory found; confirm whether Edge Functions exist only in a remote project.".into());
    }

    let mut report = Report {
        schema_version: "1.0".into(),
        project_root: root.display().to_string(),
        summary: Summary { findings: 0, low_effort: 0, medium_effort: 0, high_effort: 0, overall_effort: Effort::Low },
        scope: state.scope,
        findings: state.findings.into_values().collect(),
        checklist: checklist(),
        warnings: state.warnings,
        limitations: vec![
            "Static analysis cannot see dashboard-only settings, live row counts, object bytes, active sessions, or dynamic SQL.".into(),
            "No live Supabase project was queried. Use read-only platform exports to verify this inventory before scheduling a cutover.".into(),
            "Open-source component compatibility does not include a hosted provider's backups, scaling, uptime, security response, or support.".into(),
        ],
    };
    if report.findings.is_empty() {
        report.warnings.push(
            "No known Supabase dependencies were detected in the scanned local files.".into(),
        );
    }
    report.finalize();
    Ok(report)
}

fn locate_supabase_root(input: &Path) -> PathBuf {
    let nested = input.join("supabase");
    if nested.join("config.toml").is_file() || nested.join("migrations").is_dir() {
        nested
    } else {
        input.to_path_buf()
    }
}

#[derive(Clone, Copy)]
enum FileKind {
    Config,
    Sql,
    Function,
}

#[derive(Default)]
struct State {
    findings: BTreeMap<String, Finding>,
    warnings: Vec<String>,
    scope: ScanScope,
}

impl Default for ScanScope {
    fn default() -> Self {
        Self {
            config_files: 0,
            sql_files: 0,
            function_files: 0,
            bytes_read: 0,
            live_project_queried: false,
        }
    }
}

impl State {
    fn read(&mut self, root: &Path, path: &Path, kind: FileKind) {
        if !matches_extension(path, kind) {
            return;
        }
        let bytes = match fs::read(path) {
            Ok(value) => value,
            Err(error) => {
                self.warnings
                    .push(format!("Could not read {}: {error}", relative(root, path)));
                return;
            }
        };
        self.scope.bytes_read += bytes.len() as u64;
        match kind {
            FileKind::Config => self.scope.config_files += 1,
            FileKind::Sql => self.scope.sql_files += 1,
            FileKind::Function => self.scope.function_files += 1,
        }
        let content = match String::from_utf8(bytes) {
            Ok(value) => value,
            Err(_) => {
                self.warnings
                    .push(format!("Skipped non-UTF-8 file {}", relative(root, path)));
                return;
            }
        };
        let rel = relative(root, path);
        match kind {
            FileKind::Config => self.scan_config(&rel, &content),
            FileKind::Sql => self.scan_sql(&rel, &content),
            FileKind::Function => self.scan_function(&rel, &content),
        }
    }

    fn add(&mut self, id: &str, name: &str, file: &str, line: usize, excerpt: &str) {
        let item = self
            .findings
            .entry(id.into())
            .or_insert_with(|| finding(id, name));
        item.evidence.push(Evidence {
            file: file.into(),
            line,
            excerpt: redact(excerpt),
        });
    }

    fn scan_config(&mut self, file: &str, content: &str) {
        let mut section = String::new();
        for (idx, raw) in content.lines().enumerate() {
            let line = raw.trim();
            if line.starts_with('[') && line.ends_with(']') {
                section = line.trim_matches(&['[', ']'][..]).to_ascii_lowercase();
                if section == "auth" || section.starts_with("auth.") {
                    self.add(
                        "auth.config",
                        "Authentication configuration",
                        file,
                        idx + 1,
                        line,
                    );
                }
                if section == "storage" || section.starts_with("storage.") {
                    self.add("storage", "Object storage", file, idx + 1, line);
                }
                if section == "realtime" || section.starts_with("realtime.") {
                    self.add(
                        "realtime",
                        "Realtime channels and replication",
                        file,
                        idx + 1,
                        line,
                    );
                }
                if section == "functions"
                    || section.starts_with("functions.")
                    || section == "edge_runtime"
                {
                    self.add("edge.functions", "Edge Functions", file, idx + 1, line);
                }
            }
            let lower = line.to_ascii_lowercase();
            if section.starts_with("auth")
                && (lower.contains("enabled")
                    || lower.contains("redirect")
                    || lower.contains("site_url"))
            {
                self.add(
                    "auth.config",
                    "Authentication configuration",
                    file,
                    idx + 1,
                    line,
                );
            }
        }
    }

    fn scan_sql(&mut self, file: &str, content: &str) {
        for (idx, raw) in content.lines().enumerate() {
            let lower = raw.to_ascii_lowercase();
            let line = idx + 1;
            if lower.contains("create extension") {
                self.add(
                    "database.extensions",
                    "Postgres extensions",
                    file,
                    line,
                    raw,
                );
            }
            if lower.contains("create policy")
                || lower.contains("enable row level security")
                || lower.contains("auth.uid()")
                || lower.contains("auth.jwt()")
            {
                self.add(
                    "database.rls",
                    "Row-level security and claim helpers",
                    file,
                    line,
                    raw,
                );
            }
            if lower.contains("create function")
                || lower.contains("create or replace function")
                || lower.contains("create trigger")
                || lower.contains("security definer")
            {
                self.add(
                    "database.routines",
                    "Database routines and triggers",
                    file,
                    line,
                    raw,
                );
            }
            if lower.contains("auth.users")
                || lower.contains("auth.identities")
                || lower.contains("auth.sessions")
            {
                self.add("auth.schema", "Auth schema dependencies", file, line, raw);
            }
            if lower.contains("storage.objects")
                || lower.contains("storage.buckets")
                || lower.contains("storage.foldername")
            {
                self.add("storage", "Object storage", file, line, raw);
            }
            if lower.contains("supabase_realtime")
                || lower.contains("realtime.")
                || lower.contains("alter publication")
            {
                self.add(
                    "realtime",
                    "Realtime channels and replication",
                    file,
                    line,
                    raw,
                );
            }
            if lower.contains("cron.schedule") || lower.contains("pg_cron") {
                self.add("platform.cron", "Scheduled database jobs", file, line, raw);
            }
            if lower.contains("vault.") || lower.contains("supabase_vault") {
                self.add(
                    "platform.vault",
                    "Database-managed secrets",
                    file,
                    line,
                    raw,
                );
            }
            if lower.contains("net.http_")
                || lower.contains("pg_net")
                || lower.contains("http_post(")
            {
                self.add(
                    "platform.network",
                    "Database outbound network calls",
                    file,
                    line,
                    raw,
                );
            }
        }
    }

    fn scan_function(&mut self, file: &str, content: &str) {
        let code_ext = matches!(
            Path::new(file).extension().and_then(|s| s.to_str()),
            Some("ts" | "tsx" | "js" | "jsx" | "mjs" | "cjs")
        );
        if !code_ext {
            return;
        }
        let name = Path::new(file)
            .components()
            .nth(1)
            .and_then(|c| c.as_os_str().to_str())
            .unwrap_or("function");
        let mut signals = Vec::new();
        for (idx, raw) in content.lines().enumerate() {
            let lower = raw.to_ascii_lowercase();
            if lower.contains("deno.env.get") {
                signals.push(format!("secret/environment access at line {}", idx + 1));
            }
            if lower.contains("createclient") || lower.contains("@supabase/supabase-js") {
                signals.push(format!("Supabase client at line {}", idx + 1));
            }
            if lower.contains("fetch(") {
                signals.push(format!("outbound HTTP at line {}", idx + 1));
            }
            if lower.contains(".channel(") || lower.contains(".storage") || lower.contains(".auth")
            {
                signals.push(format!("managed API call at line {}", idx + 1));
            }
        }
        let evidence = if signals.is_empty() {
            format!("Edge Function source: {name}")
        } else {
            format!("{}: {}", name, signals.join(", "))
        };
        self.add("edge.functions", "Edge Functions", file, 1, &evidence);
    }
}

fn matches_extension(path: &Path, kind: FileKind) -> bool {
    if !path.is_file() {
        return false;
    }
    match kind {
        FileKind::Config => path.file_name().and_then(|s| s.to_str()) == Some("config.toml"),
        FileKind::Sql => path
            .extension()
            .and_then(|s| s.to_str())
            .is_some_and(|s| s.eq_ignore_ascii_case("sql")),
        FileKind::Function => {
            !matches!(path.file_name().and_then(|s| s.to_str()), Some(name) if name.starts_with('.'))
        }
    }
}

fn walk(dir: &Path, visitor: &mut dyn FnMut(&Path)) -> io::Result<()> {
    let mut entries: Vec<_> = fs::read_dir(dir)?.collect::<Result<_, _>>()?;
    entries.sort_by_key(|e| e.path());
    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            walk(&path, visitor)?;
        } else {
            visitor(&path);
        }
    }
    Ok(())
}

fn relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn redact(input: &str) -> String {
    let trimmed = input.trim();
    let lower = trimmed.to_ascii_lowercase();
    if lower.contains("vault.") || lower.contains("create_secret") {
        let call = trimmed.split('(').next().unwrap_or("secret operation");
        format!("{call}([arguments redacted])")
    } else if lower.contains("secret") && trimmed.contains('=') {
        format!(
            "{} = [value redacted]",
            trimmed.split('=').next().unwrap_or("secret").trim()
        )
    } else {
        trimmed.chars().take(180).collect()
    }
}

fn checklist() -> Vec<ChecklistItem> {
    vec![
        ChecklistItem { phase: 1, title: "Freeze the inventory".into(), detail: "Commit migrations and function source; export dashboard settings and list storage objects with read-only access.".into(), required: true },
        ChecklistItem { phase: 1, title: "Measure the live system".into(), detail: "Record database size, write rate, active sessions, object bytes, realtime connections, and recovery objectives.".into(), required: true },
        ChecklistItem { phase: 2, title: "Prove portable SQL".into(), detail: "Restore a fresh logical dump into the target Postgres version and resolve extensions, owners, roles, and search paths.".into(), required: true },
        ChecklistItem { phase: 2, title: "Replace managed edges".into(), detail: "Stand up auth, object storage, realtime, functions, mail, secrets, and scheduled jobs that the findings identify.".into(), required: true },
        ChecklistItem { phase: 3, title: "Rehearse with production-shaped data".into(), detail: "Test policy allow/deny cases, login/session behavior, object hashes, websocket load, background retries, backups, and restore.".into(), required: true },
        ChecklistItem { phase: 3, title: "Define rollback".into(), detail: "Set cutover ownership, data freeze or replication method, acceptance checks, rollback trigger, DNS/session behavior, and observation window.".into(), required: true },
    ]
}
