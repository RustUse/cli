//! Runs RustUse validation checks for CI systems.

use std::path::PathBuf;

use anyhow::{Result, bail};
use clap::{Args, ValueEnum};

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

pub fn run(args: CheckCiArgs, output: Output) -> Result<()> {
    if !matches!(args.kind, CheckKind::Auto | CheckKind::Facade) {
        bail!(
            "ci check currently supports only auto and facade kinds; received {:?}",
            args.kind
        );
    }

    let diagnostics = FacadeDiagnostics::inspect(&args.path)?;
    let status = diagnostics.status();
    let summary = format!(
        "RustUse CI check - facade: {}; errors: {}; warnings: {}",
        diagnostics.facade.root.display(),
        diagnostics.error_count(),
        diagnostics.warning_count()
    );

    if output.is_json() {
        output.record("ci check", status, &summary);
    } else {
        output.line(&summary);
        for issue in &diagnostics.issues {
            let path = issue
                .path
                .as_deref()
                .map_or_else(|| "".to_owned(), |path| format!(" [{}]", path.display()));
            output.line(format!(
                "{} {}: {}{}",
                issue.severity.as_str(),
                issue.code,
                issue.message,
                path
            ));
        }
    }

    if diagnostics.error_count() > 0 || (args.deny_warnings && diagnostics.warning_count() > 0) {
        bail!(
            "ci check failed with {} error(s) and {} warning(s){}",
            diagnostics.error_count(),
            diagnostics.warning_count(),
            if args.deny_warnings {
                "; --deny-warnings is enabled"
            } else {
                ""
            }
        );
    }

    Ok(())
}
