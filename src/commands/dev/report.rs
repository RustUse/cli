use std::path::PathBuf;

use anyhow::Result;
use clap::Args;

use crate::output::Output;
use crate::rustuse::report::generate::generate_report;
use crate::rustuse::report::subject::ReportSubject;

#[derive(Debug, Args)]
pub struct DevReportArgs {
    /// Path to report.
    #[arg(default_value = ".", value_name = "PATH")]
    pub path: PathBuf,

    /// Treat PATH as a RustUse fleet root containing many use-* repositories.
    #[arg(long)]
    pub fleet: bool,
}

pub(crate) fn run(
    args: DevReportArgs,
    output: Output,
    context: super::DevCommandContext,
) -> Result<()> {
    let _ = context;

    let subject = if args.fleet {
        ReportSubject::Fleet
    } else {
        ReportSubject::Facade
    };

    generate_report(&args.path, subject, output)
}
