use std::path::Path;

use anyhow::Result;

use crate::output::Output;
use crate::rustuse::report::destination::ReportDestination;
use crate::rustuse::report::subject::ReportSubject;

pub(crate) fn generate_report(
    path: &Path,
    subject: ReportSubject,
    output: Output,
    destination: ReportDestination,
) -> Result<()> {
    match subject {
        ReportSubject::Facade => {
            crate::rustuse::facade::report::generate_markdown_report(path, output, destination)
        },
        ReportSubject::Root => {
            crate::rustuse::root::report::generate_markdown_report(path, output, destination)
        },
    }
}
