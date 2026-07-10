use crate::rustuse::facade::diagnostics::FacadeDiagnostics;
use crate::rustuse::report::markdown::write_presence_table;

pub(crate) fn write_release_surface(markdown: &mut String, diagnostics: &FacadeDiagnostics) {
    let release = &diagnostics.release;

    markdown.push_str("## Release Surface\n\n");
    markdown.push_str(&format!("- Status: **{}**\n", release.status()));
    markdown.push_str(&format!(
        "- Present: `{}/{}`\n\n",
        release.present_count(),
        release.total_count()
    ));

    write_presence_table(markdown, &release.surface);
}
