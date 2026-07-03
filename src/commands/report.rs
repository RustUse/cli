use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::{Args, ValueEnum};

use crate::output::Output;
use crate::rustuse::facade::discover::is_facade;
use crate::rustuse::report::destination::ReportDestination;
use crate::rustuse::report::generate::generate_report;
use crate::rustuse::report::subject::ReportSubject;

use super::placeholder;

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum ReportKind {
    Auto,
    Facade,
    Root,
    Catalog,
    Ci,
}

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
    #[arg(long, value_name = "FILE")]
    pub output: Option<PathBuf>,
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
        ReportKind::Auto => {
            let subject = report_subject_for_auto(&args.path);
            generate_report(&args.path, subject, output, destination)
        },
        ReportKind::Facade => {
            generate_report(&args.path, ReportSubject::Facade, output, destination)
        },
        ReportKind::Root => generate_report(&args.path, ReportSubject::Root, output, destination),
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
    ReportDestination::from_output(args.stdout, args.output.clone())
}

fn report_subject_for_auto(path: &Path) -> ReportSubject {
    if is_facade(path) {
        ReportSubject::Facade
    } else {
        ReportSubject::Root
    }
}

fn output_path_label(path: Option<&Path>) -> String {
    path.map(|path| path.display().to_string())
        .unwrap_or_else(|| "<default>".to_owned())
}
