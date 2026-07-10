use crate::rustuse::facade::diagnostics::FacadeDiagnostics;
use crate::rustuse::report::markdown::write_presence_table;

pub(crate) fn write_tooling_configuration(markdown: &mut String, diagnostics: &FacadeDiagnostics) {
    let tooling = &diagnostics.tooling;

    markdown.push_str("## Tooling Configuration\n\n");
    markdown.push_str(&format!("- Status: **{}**\n", tooling.status()));
    markdown.push_str(&format!(
        "- Present: `{}/{}`\n\n",
        tooling.present_count(),
        tooling.total_count()
    ));

    write_presence_table(markdown, &tooling.surface);
}
