use serde::Serialize;
use std::fmt::{self, Display};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Category {
    Database,
    Auth,
    Storage,
    Realtime,
    EdgeFunction,
    Platform,
}

impl Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Database => "Database",
                Self::Auth => "Auth",
                Self::Storage => "Storage",
                Self::Realtime => "Realtime",
                Self::EdgeFunction => "Edge Functions",
                Self::Platform => "Platform",
            }
        )
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum Effort {
    Low,
    Medium,
    High,
}

impl Display for Effort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Low => "low",
                Self::Medium => "medium",
                Self::High => "high",
            }
        )
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Portability {
    Portable,
    Adapt,
    Replace,
    Verify,
}

impl Display for Portability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Portable => "portable",
                Self::Adapt => "adapt",
                Self::Replace => "replace",
                Self::Verify => "verify",
            }
        )
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Evidence {
    pub file: String,
    pub line: usize,
    pub excerpt: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Finding {
    pub id: String,
    pub category: Category,
    pub name: String,
    pub portability: Portability,
    pub effort: Effort,
    pub evidence: Vec<Evidence>,
    pub compatibility: String,
    pub hosted_guarantee: String,
    pub replacements: Vec<String>,
    pub next_steps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ChecklistItem {
    pub phase: u8,
    pub title: String,
    pub detail: String,
    pub required: bool,
}

#[derive(Debug, Clone, Default, Serialize, PartialEq, Eq)]
pub struct ScanScope {
    pub config_files: usize,
    pub sql_files: usize,
    pub function_files: usize,
    pub bytes_read: u64,
    pub live_project_queried: bool,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Summary {
    pub findings: usize,
    pub low_effort: usize,
    pub medium_effort: usize,
    pub high_effort: usize,
    pub overall_effort: Effort,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Report {
    pub schema_version: String,
    pub project_root: String,
    pub summary: Summary,
    pub scope: ScanScope,
    pub findings: Vec<Finding>,
    pub checklist: Vec<ChecklistItem>,
    pub warnings: Vec<String>,
    pub limitations: Vec<String>,
}

impl Report {
    pub fn finalize(&mut self) {
        self.findings.sort_by(|a, b| a.id.cmp(&b.id));
        for finding in &mut self.findings {
            finding
                .evidence
                .sort_by(|a, b| a.file.cmp(&b.file).then(a.line.cmp(&b.line)));
            finding.evidence.dedup();
        }
        self.warnings.sort();
        self.warnings.dedup();
        let low = self
            .findings
            .iter()
            .filter(|f| f.effort == Effort::Low)
            .count();
        let medium = self
            .findings
            .iter()
            .filter(|f| f.effort == Effort::Medium)
            .count();
        let high = self
            .findings
            .iter()
            .filter(|f| f.effort == Effort::High)
            .count();
        self.summary = Summary {
            findings: self.findings.len(),
            low_effort: low,
            medium_effort: medium,
            high_effort: high,
            overall_effort: if high > 0 {
                Effort::High
            } else if medium > 0 {
                Effort::Medium
            } else {
                Effort::Low
            },
        };
    }
}
