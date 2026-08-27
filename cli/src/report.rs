use crate::model::{Effort, Report};

pub fn json(report: &Report) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(report)
}

pub fn terminal(report: &Report) -> String {
    let mut out = String::new();
    out.push_str("SUPABASE EXIT MAP\n");
    out.push_str(&format!("Project  {}\n", report.project_root));
    out.push_str(&format!(
        "Scanned  {} config · {} SQL · {} function files · {}\n",
        report.scope.config_files,
        report.scope.sql_files,
        report.scope.function_files,
        human_bytes(report.scope.bytes_read)
    ));
    out.push_str(&format!(
        "Estimate {} overall · {} findings ({} low / {} medium / {} high)\n\n",
        report.summary.overall_effort,
        report.summary.findings,
        report.summary.low_effort,
        report.summary.medium_effort,
        report.summary.high_effort
    ));
    if report.findings.is_empty() {
        out.push_str("No dependencies detected in local files. Review the warnings and manual checklist below.\n\n");
    }
    for (index, item) in report.findings.iter().enumerate() {
        out.push_str(&format!(
            "{:02}  {} · {} · {}\n",
            index + 1,
            item.category,
            item.portability,
            item.effort
        ));
        out.push_str(&format!("    {}\n", item.name));
        if let Some(e) = item.evidence.first() {
            out.push_str(&format!(
                "    Evidence: {}:{} — {}\n",
                e.file, e.line, e.excerpt
            ));
        }
        out.push_str(&format!("    Route: {}\n", item.replacements.join(" / ")));
        out.push_str(&format!("    Next: {}\n\n", item.next_steps.join("; ")));
    }
    out.push_str("PORTABLE SQL CHECKLIST\n");
    for item in &report.checklist {
        out.push_str(&format!(
            "[ ] Phase {} — {}: {}\n",
            item.phase, item.title, item.detail
        ));
    }
    if !report.warnings.is_empty() {
        out.push_str("\nWARNINGS\n");
        for warning in &report.warnings {
            out.push_str(&format!("! {warning}\n"));
        }
    }
    out.push_str("\nBOUNDARY\n");
    for note in &report.limitations {
        out.push_str(&format!("- {note}\n"));
    }
    out
}

pub fn markdown(report: &Report) -> String {
    let mut out = format!("# Supabase exit map\n\n**Project:** `{}`  \n**Overall effort:** {}  \n**Inventory:** {} findings ({} low, {} medium, {} high)\n\n", report.project_root, report.summary.overall_effort, report.summary.findings, report.summary.low_effort, report.summary.medium_effort, report.summary.high_effort);
    out.push_str("## Dependency map\n\n");
    if report.findings.is_empty() {
        out.push_str("No known dependencies were detected in local files. Review warnings and verify the live project.\n\n");
    }
    for item in &report.findings {
        out.push_str(&format!("### {}\n\n- **Area:** {}\n- **Disposition:** {}\n- **Effort:** {}\n- **Open-source compatibility:** {}\n- **Hosted-service boundary:** {}\n- **Replacement choices:** {}\n", item.name, item.category, item.portability, item.effort, item.compatibility, item.hosted_guarantee, item.replacements.join("; ")));
        out.push_str("- **Evidence:**\n");
        for evidence in &item.evidence {
            out.push_str(&format!(
                "  - `{}` line {} — `{}`\n",
                evidence.file,
                evidence.line,
                evidence.excerpt.replace('`', "'")
            ));
        }
        out.push_str("- **Next actions:**\n");
        for step in &item.next_steps {
            out.push_str(&format!("  - [ ] {step}\n"));
        }
        out.push('\n');
    }
    out.push_str("## Portable SQL and cutover checklist\n\n");
    for item in &report.checklist {
        out.push_str(&format!(
            "- [ ] **Phase {} — {}.** {}\n",
            item.phase, item.title, item.detail
        ));
    }
    out.push_str("\n## Warnings\n\n");
    for warning in &report.warnings {
        out.push_str(&format!("- {warning}\n"));
    }
    out.push_str("\n## Scope boundary\n\n");
    for note in &report.limitations {
        out.push_str(&format!("- {note}\n"));
    }
    out.push_str("\n_Generated locally by Supabase Exit Map. No live project was queried._\n");
    out
}

fn human_bytes(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{bytes} B")
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KiB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MiB", bytes as f64 / 1024.0 / 1024.0)
    }
}

#[allow(dead_code)]
fn _assert_effort_is_used(_: Effort) {}
