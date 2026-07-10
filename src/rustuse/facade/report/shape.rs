use crate::rustuse::facade::diagnostics::FacadeDiagnostics;
use crate::rustuse::report::markdown::yes_no;

pub(crate) fn write_detected_facade_shape(markdown: &mut String, diagnostics: &FacadeDiagnostics) {
    let facade = &diagnostics.facade;

    markdown.push_str("## Facade Shape\n\n");
    markdown.push_str(&format!("- Structure status: **{}**\n", facade.status()));
    markdown.push_str(&format!(
        "- Root manifest: `{}`\n",
        facade.root_manifest_display_path()
    ));
    markdown.push_str(&format!(
        "- Child crates directory: `{}`\n\n",
        facade.crates_dir_display_path()
    ));
    markdown.push_str("| Check | Status |\n");
    markdown.push_str("|---|---:|\n");
    markdown.push_str(&format!("| `.git` | {} |\n", yes_no(facade.has_git())));
    markdown.push_str(&format!(
        "| `Cargo.toml` | {} |\n",
        yes_no(facade.has_manifest())
    ));
    markdown.push_str(&format!(
        "| `crates/` | {} |\n",
        yes_no(facade.has_crates_dir())
    ));
    markdown.push_str(&format!(
        "| Child crate manifests | {} |\n",
        facade.crate_count()
    ));
    markdown.push('\n');
}
