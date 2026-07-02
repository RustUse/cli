use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::{Args, ValueEnum};

use crate::{output::Output, rustuse, rustuse::utils::report::ReportDestination};

use super::placeholder;

#[derive(Debug, Args)]
pub struct ReportArgs {
    /// Path to report on.
    #[arg(default_value = ".", value_name = "PATH")]
    pub path: PathBuf,

    /// Report target kind.
    #[arg(long, value_enum, default_value_t = ReportKind::Auto)]
    pub kind: ReportKind,

    /// Print the report to standard output instead of writing a file.
    #[arg(long, conflicts_with = "output")]
    pub stdout: bool,

    /// Optional output file.
    ///
    /// Reserved for explicit report output routing. Existing domain report
    /// writers may still use their default report path until wired.
    #[arg(long, value_name = "FILE")]
    pub output: Option<PathBuf>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum ReportKind {
    Auto,
    Facade,
    Root,
    Catalog,
    Ci,
}

pub fn run(args: ReportArgs, output: Output) -> Result<()> {
    if let Some(report_output) = &args.output {
        output.detail(format!(
            "requested report output: {}",
            report_output.display()
        ));
    }

    let destination = report_destination(&args);

    match args.kind {
        ReportKind::Auto => run_auto(&args.path, output, destination),
        ReportKind::Facade => {
            rustuse::facade::report::generate_markdown_report(&args.path, output, destination)
        },
        ReportKind::Root => {
            rustuse::root::report::generate_markdown_report(&args.path, output, destination)
        },
        ReportKind::Catalog => placeholder(
            output,
            "report",
            format!(
                "path={}, kind=Catalog, output={}",
                args.path.display(),
                output_path_label(args.output.as_deref())
            ),
        ),
        ReportKind::Ci => placeholder(
            output,
            "report",
            format!(
                "path={}, kind=Ci, output={}",
                args.path.display(),
                output_path_label(args.output.as_deref())
            ),
        ),
    }
}

fn report_destination(args: &ReportArgs) -> ReportDestination {
    if args.stdout {
        ReportDestination::Stdout
    } else {
        ReportDestination::File(args.output.clone())
    }
}

fn run_auto(path: &Path, output: Output, destination: ReportDestination) -> Result<()> {
    if rustuse::facade::discover::looks_like_facade(path) {
        rustuse::facade::report::generate_markdown_report(path, output, destination)
    } else {
        rustuse::root::report::generate_markdown_report(path, output, destination)
    }
}

fn output_path_label(path: Option<&Path>) -> String {
    path.map(|path| path.display().to_string())
        .unwrap_or_else(|| "<default>".to_owned())
}
