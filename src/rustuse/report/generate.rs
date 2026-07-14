use std::path::Path;

use anyhow::{Context, Result};

use crate::output::Output;
use crate::rustuse::report::subject::ReportSubject;

pub(crate) fn generate_report(path: &Path, subject: ReportSubject, output: Output) -> Result<()> {
    let subject_name = subject.as_str();

    match subject {
        ReportSubject::Facade => {
            crate::rustuse::facade::report::generate_markdown_report(path, output)
        },
        ReportSubject::Fleet => {
            crate::rustuse::fleet::report::generate_markdown_report(path, output)
        },
    }
    .with_context(|| format!("failed to generate {subject_name} report"))
}
