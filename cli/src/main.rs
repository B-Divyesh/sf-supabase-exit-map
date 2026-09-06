use clap::{Parser, ValueEnum};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use supabase_exit_map::{report, scan_project};

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Format {
    Terminal,
    Json,
    Markdown,
}

/// Inventory local Supabase dependencies and plan a lower-risk exit.
#[derive(Debug, Parser)]
#[command(
    name = "supabase-exit-map",
    version,
    about,
    long_about = "Scan a local Supabase project without credentials or network access. Produces evidence-backed dependency findings, portable SQL checks, replacement choices, and effort flags. This does not perform a migration or inspect a live project."
)]
struct Cli {
    /// Project root or Supabase directory to scan
    #[arg(value_name = "PATH")]
    path: Option<PathBuf>,

    /// Run the bundled sample in a new temporary directory
    #[arg(long, conflicts_with = "path")]
    demo: bool,

    /// Report format written to stdout (or --output)
    #[arg(long, value_enum, default_value_t = Format::Terminal)]
    format: Format,

    /// Shorthand for --format json
    #[arg(long, conflicts_with = "format")]
    json: bool,

    /// Write the report to a file instead of stdout
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,

    /// Exit 3 when the scan has warnings (useful in CI)
    #[arg(long)]
    strict: bool,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let demo = if cli.demo {
        match materialize_demo() {
            Ok(value) => Some(value),
            Err(error) => {
                eprintln!("error: could not create bundled demo: {error}");
                return ExitCode::from(2);
            }
        }
    } else {
        None
    };
    let input = demo
        .as_deref()
        .or(cli.path.as_deref())
        .unwrap_or_else(|| Path::new("."));
    let scanned = match scan_project(input) {
        Ok(value) => value,
        Err(error) => {
            eprintln!("error: {error}");
            return ExitCode::from(2);
        }
    };
    let format = if cli.json { Format::Json } else { cli.format };
    let rendered = match format {
        Format::Terminal => report::terminal(&scanned),
        Format::Json => match report::json(&scanned) {
            Ok(value) => value,
            Err(error) => {
                eprintln!("error: could not serialize report: {error}");
                return ExitCode::from(2);
            }
        },
        Format::Markdown => report::markdown(&scanned),
    };
    let demo_report = demo
        .as_ref()
        .map(|root| root.join(format!("exit-map-demo.{}", extension(format))));
    let output = cli.output.as_ref().or(demo_report.as_ref());
    if let Some(path) = output {
        if let Err(error) = fs::write(path, format!("{rendered}\n")) {
            eprintln!("error: could not write {}: {error}", path.display());
            return ExitCode::from(2);
        }
        if cli.demo {
            eprintln!(
                "Bundled sample copied to {}",
                demo.as_ref().expect("demo root").display()
            );
            eprintln!("Demo report written to {}", path.display());
            if cli.output.is_none() {
                println!("{rendered}");
            }
        }
    } else {
        println!("{rendered}");
    }
    if cli.strict && !scanned.warnings.is_empty() {
        ExitCode::from(3)
    } else {
        ExitCode::SUCCESS
    }
}

fn extension(format: Format) -> &'static str {
    match format {
        Format::Terminal => "txt",
        Format::Json => "json",
        Format::Markdown => "md",
    }
}

fn materialize_demo() -> std::io::Result<PathBuf> {
    let unique = format!(
        "supabase-exit-map-demo-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    );
    let root = std::env::temp_dir().join(unique);
    let supabase = root.join("supabase");
    fs::create_dir_all(supabase.join("migrations"))?;
    fs::create_dir_all(supabase.join("functions/send-mail"))?;
    for (relative, contents) in DEMO_FILES {
        let path = root.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, contents)?;
    }
    Ok(root)
}

const DEMO_FILES: &[(&str, &str)] = &[
    (
        "supabase/config.toml",
        include_str!("../examples/demo-project/supabase/config.toml"),
    ),
    (
        "supabase/migrations/20260906000000_workspace.sql",
        include_str!("../examples/demo-project/supabase/migrations/20260906000000_workspace.sql"),
    ),
    (
        "supabase/functions/send-mail/index.ts",
        include_str!("../examples/demo-project/supabase/functions/send-mail/index.ts"),
    ),
];
