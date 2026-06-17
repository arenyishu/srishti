use crate::typechecker::{Diagnostic, Severity};
use colored::*;

pub fn format_diagnostics(diagnostics: &[Diagnostic], source: &str, filename: &str) -> String {
    let mut out = String::new();
    let lines: Vec<&str> = source.lines().collect();

    for diag in diagnostics {
        let (color_msg, header) = match diag.severity {
            Severity::Error => (diag.message.red().bold(), format!("error[{}]", diag.code).red().bold()),
            Severity::Warning => (diag.message.yellow().bold(), format!("warning[{}]", diag.code).yellow().bold()),
            Severity::Info => (diag.message.cyan().bold(), format!("info[{}]", diag.code).cyan().bold()),
        };

        out.push_str(&format!("{}: {}\n", header, color_msg));
        out.push_str(&format!("  --> {}:{}:{}\n", filename, diag.span.line, diag.span.column));
        
        let line_idx = diag.span.line.saturating_sub(1);
        if line_idx < lines.len() {
            let line_text = lines[line_idx];
            out.push_str(&format!("   |\n"));
            out.push_str(&format!("{:2} | {}\n", diag.span.line, line_text));
            out.push_str(&format!("   | {}^{}\n", " ".repeat(diag.span.column.saturating_sub(1)), "-".repeat(5).red()));
        }

        if let Some(sugg) = &diag.suggestion {
            out.push_str(&format!("   = help: {}\n", sugg.green()));
        }
        out.push('\n');
    }

    out
}

pub fn format_summary(diagnostics: &[Diagnostic]) -> String {
    let errs = diagnostics.iter().filter(|d| matches!(d.severity, Severity::Error)).count();
    let warns = diagnostics.iter().filter(|d| matches!(d.severity, Severity::Warning)).count();
    
    if errs == 0 && warns == 0 {
        "No issues found.".green().to_string()
    } else {
        format!("Found {} errors and {} warnings.", errs, warns).yellow().to_string()
    }
}
