//! Local, read-only analysis for a Supabase project directory.
//!
//! ```no_run
//! use std::path::Path;
//! let report = supabase_exit_map::scan_project(Path::new("./supabase"))?;
//! println!("{} dependencies", report.summary.findings);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

mod guidance;
pub mod model;
pub mod report;
mod scanner;

pub use scanner::{scan_project, ScanError};
