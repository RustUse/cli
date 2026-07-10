use crate::rustuse::facade::diagnostics::FacadeDiagnostics;
use crate::rustuse::report::markdown::yes_no;

pub(crate) fn write_summary(markdown: &mut String, diagnostics: &FacadeDiagnostics) {
    markdown.push_str("# RustUse Facade Report\n\n");
    markdown.push_str("## Summary\n\n");

    markdown.push_str(&format!(
        "- Root: `{}`\n",
        diagnostics.facade.root.display()
    ));
    markdown.push_str(&format!("- Facade: `{}`\n", diagnostics.facade.name));
    markdown.push_str(&format!(
        "- Git: `{}`\n",
        yes_no(diagnostics.facade.has_git())
    ));
    markdown.push_str(&format!(
        "- Cargo.toml: `{}`\n",
        yes_no(diagnostics.facade.has_manifest())
    ));
    markdown.push_str(&format!(
        "- crates/: `{}`\n",
        yes_no(diagnostics.facade.has_crates_dir())
    ));
    markdown.push_str(&format!(
        "- Child crate manifests: `{}`\n",
        diagnostics.facade.crate_count()
    ));
    markdown.push_str(&format!(
        "- Manifests inspected: `{}`\n",
        diagnostics.manifest.manifest_count()
    ));
    markdown.push_str(&format!("- Issues: `{}`\n", diagnostics.issue_count()));
    markdown.push_str(&format!("- Errors: `{}`\n", diagnostics.error_count()));
    markdown.push_str(&format!("- Warnings: `{}`\n", diagnostics.warning_count()));
    markdown.push_str(&format!(
        "- Fixable issues: `{}`\n",
        diagnostics.fixable_count()
    ));
    markdown.push_str(&format!("- Status: **{}**\n\n", diagnostics.status()));
}

pub(crate) fn write_notes(markdown: &mut String) {
    markdown.push_str("## Notes\n\n");
    markdown.push_str("- This report is generated from the local filesystem.\n");
    markdown.push_str(
        "- A facade repository is expected to contain `.git`, `Cargo.toml`, and `crates/`.\n",
    );
    markdown.push_str(
        "- Child crates are direct directories under `crates/` with their own `Cargo.toml`.\n",
    );
    markdown.push_str(
        "- Diagnostics are shared by report, status, plan, and fix so all dev commands use the same issue source.\n",
    );
    markdown.push('\n');
}
