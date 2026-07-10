use crate::rustuse::facade::diagnostics::FacadeDiagnostics;
use crate::rustuse::report::destination::report_path;
use crate::rustuse::report::markdown::yes_no;

pub(crate) fn write_child_crates(markdown: &mut String, diagnostics: &FacadeDiagnostics) {
    markdown.push_str("## Child Crates\n\n");

    let rows = diagnostics.child_crate_rows();

    if rows.is_empty() {
        markdown.push_str("- No child crate manifests found.\n\n");
        return;
    }

    markdown
        .push_str("| Kind | Crate | Manifest | README | lib.rs | prelude.rs | Status | Issues |\n");
    markdown.push_str("|---|---|---|---:|---:|---:|---:|---:|\n");

    for row in rows {
        markdown.push_str(&format!(
            "| `{}` | `{}` | `{}` | {} | {} | {} | {} | {} |\n",
            row.kind,
            row.crate_name,
            report_path(&row.manifest_path),
            yes_no(row.readme_present),
            yes_no(row.lib_present),
            yes_no(row.prelude_present),
            row.manifest_status,
            row.manifest_issue_count
        ));
    }

    markdown.push('\n');
}
