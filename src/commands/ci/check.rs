//! Runs RustUse validation checks for CI systems.

use std::path::PathBuf;

use anyhow::{Result, bail};
use clap::{Args, ValueEnum};
use serde::Serialize;

use crate::output::Output;
use crate::rustuse::facade::diagnostics::FacadeDiagnostics;

#[derive(Debug, Args)]
pub struct CheckCiArgs {
    /// Path to check.
    #[arg(default_value = ".", value_name = "PATH")]
    pub path: PathBuf,

    /// Check target kind.
    #[arg(long, value_enum, default_value_t = CheckKind::Auto)]
    pub kind: CheckKind,

    /// Fail if warnings are found.
    #[arg(long)]
    pub deny_warnings: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum CheckKind {
    Auto,
    Facade,
}

#[derive(Debug, Serialize)]
struct CheckResponse {
    command: &'static str,
    status: String,
    passed: bool,
    kind: &'static str,
    deny_warnings: bool,
    facade: PathBuf,
    errors: usize,
    warnings: usize,
    issues: Vec<CheckIssue>,
}

#[derive(Debug, Serialize)]
struct CheckIssue {
    severity: String,
    code: String,
    message: String,
    path: Option<PathBuf>,
}

pub fn run(args: CheckCiArgs, output: Output) -> Result<()> {
    let diagnostics = match args.kind {
        CheckKind::Auto | CheckKind::Facade => FacadeDiagnostics::inspect(&args.path)?,
    };

    let errors = diagnostics.error_count();
    let warnings = diagnostics.warning_count();
    let passed = errors == 0 && (!args.deny_warnings || warnings == 0);

    let response = CheckResponse {
        command: "ci check",
        status: diagnostics.status().to_owned(),
        passed,
        kind: args.kind.as_str(),
        deny_warnings: args.deny_warnings,
        facade: diagnostics.facade.root.clone(),
        errors,
        warnings,
        issues: diagnostics
            .issues
            .iter()
            .map(|issue| CheckIssue {
                severity: issue.severity.as_str().to_owned(),
                code: issue.code.to_string(),
                message: issue.message.clone(),
                path: issue.path.clone(),
            })
            .collect(),
    };

    render(&output, &response)?;

    if !response.passed {
        bail!(
            "ci check failed with {} error(s) and {} warning(s){}",
            response.errors,
            response.warnings,
            if response.deny_warnings {
                "; --deny-warnings is enabled"
            } else {
                ""
            }
        );
    }

    Ok(())
}

fn render(output: &Output, response: &CheckResponse) -> Result<()> {
    if output.is_json() {
        return output.json(response);
    }

    output.line(format!(
        "RustUse CI check - facade: {}; errors: {}; warnings: {}",
        response.facade.display(),
        response.errors,
        response.warnings
    ))?;

    for issue in &response.issues {
        let path = issue
            .path
            .as_deref()
            .map_or_else(String::new, |path| format!(" [{}]", path.display()));

        output.line(format!(
            "{} {}: {}{}",
            issue.severity, issue.code, issue.message, path
        ))?;
    }

    Ok(())
}

impl CheckKind {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Facade => "facade",
        }
    }
}
