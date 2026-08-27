use std::fs;
use std::path::Path;
use std::process::Command;
use supabase_exit_map::model::{Effort, Portability};
use supabase_exit_map::{report, scan_project, ScanError};
use tempfile::TempDir;

fn project() -> TempDir {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("supabase");
    fs::create_dir_all(root.join("migrations")).unwrap();
    fs::create_dir_all(root.join("functions/send-mail")).unwrap();
    fs::write(
        root.join("config.toml"),
        r#"project_id = "local"
[auth]
enabled = true
[auth.external.github]
enabled = true
[storage]
enabled = true
[realtime]
enabled = true
[functions.send-mail]
verify_jwt = true
"#,
    )
    .unwrap();
    fs::write(
        root.join("migrations/20260101000000_app.sql"),
        r#"create extension if not exists pg_cron;
alter table public.notes enable row level security;
create policy "read own" on public.notes using (auth.uid() = owner_id);
create or replace function public.on_user() returns trigger security definer as $$ begin return new; end; $$ language plpgsql;
create trigger user_created after insert on auth.users execute function public.on_user();
insert into storage.buckets (id, name) values ('files', 'files');
alter publication supabase_realtime add table public.notes;
select cron.schedule('nightly', '0 1 * * *', 'select 1');
select vault.create_secret('do-not-print', 'secret_key');
select net.http_post(url := 'https://example.invalid');
"#,
    )
    .unwrap();
    fs::write(
        root.join("functions/send-mail/index.ts"),
        r#"import { createClient } from "npm:@supabase/supabase-js";
const secret = Deno.env.get("MAIL_SECRET");
await fetch("https://mail.example.invalid", { method: "POST" });
"#,
    )
    .unwrap();
    temp
}

#[test]
fn inventories_managed_dependencies_and_evidence() {
    let temp = project();
    let result = scan_project(temp.path()).unwrap();
    let ids: Vec<_> = result.findings.iter().map(|f| f.id.as_str()).collect();

    for expected in [
        "auth.config",
        "auth.schema",
        "database.extensions",
        "database.rls",
        "database.routines",
        "edge.functions",
        "platform.cron",
        "platform.network",
        "platform.vault",
        "realtime",
        "storage",
    ] {
        assert!(ids.contains(&expected), "missing {expected}: {ids:?}");
    }
    assert_eq!(result.scope.config_files, 1);
    assert_eq!(result.scope.sql_files, 1);
    assert_eq!(result.scope.function_files, 1);
    assert!(!result.scope.live_project_queried);
    assert_eq!(result.summary.overall_effort, Effort::High);
    assert!(result.findings.iter().any(|f| {
        f.id == "storage" && f.portability == Portability::Replace && !f.next_steps.is_empty()
    }));
    assert!(result
        .findings
        .iter()
        .flat_map(|f| &f.evidence)
        .all(|e| !e.excerpt.contains("do-not-print")));
}

#[test]
fn documented_json_and_markdown_are_stable_and_useful() {
    let temp = project();
    let result = scan_project(temp.path()).unwrap();
    let json = report::json(&result).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["schema_version"], "1.0");
    assert_eq!(parsed["scope"]["live_project_queried"], false);

    let markdown = report::markdown(&result);
    assert!(markdown.starts_with("# Supabase exit map"));
    assert!(markdown.contains("Open-source compatibility"));
    assert!(markdown.contains("Hosted-service boundary"));
    assert!(markdown.contains("- [ ] **Phase 1"));
}

#[test]
fn empty_project_returns_actionable_warnings() {
    let temp = TempDir::new().unwrap();
    let result = scan_project(temp.path()).unwrap();
    assert!(result.findings.is_empty());
    assert!(result.warnings.len() >= 4);
    assert!(report::terminal(&result).contains("No dependencies detected"));
}

#[test]
fn invalid_input_is_distinct() {
    let error = scan_project(Path::new("this-path-does-not-exist")).unwrap_err();
    assert!(matches!(error, ScanError::Missing(_)));
}

#[test]
fn cli_supports_json_output_files_and_strict_exit_code() {
    let temp = project();
    let output = temp.path().join("map.json");
    let status = Command::new(env!("CARGO_BIN_EXE_supabase-exit-map"))
        .arg(temp.path())
        .arg("--json")
        .arg("--output")
        .arg(&output)
        .status()
        .unwrap();
    assert!(status.success());
    let parsed: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(output).unwrap()).unwrap();
    assert!(parsed["findings"].as_array().unwrap().len() >= 10);

    let empty = TempDir::new().unwrap();
    let strict = Command::new(env!("CARGO_BIN_EXE_supabase-exit-map"))
        .arg(empty.path())
        .arg("--strict")
        .arg("--json")
        .status()
        .unwrap();
    assert_eq!(strict.code(), Some(3));
}
