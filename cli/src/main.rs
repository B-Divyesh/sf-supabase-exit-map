use clap::{Parser, ValueEnum};
use std::fs;
use std::path::PathBuf;
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
    #[arg(default_value = ".", value_name = "PATH")]
    path: PathBuf,

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
    let scanned = match scan_project(&cli.path) {
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
    if let Some(path) = cli.output {
        if let Err(error) = fs::write(&path, format!("{rendered}\n")) {
            eprintln!("error: could not write {}: {error}", path.display());
            return ExitCode::from(2);
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
