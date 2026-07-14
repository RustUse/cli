use crate::rustuse::facade::diagnostics::FacadeDiagnostics;

pub(crate) fn write_crate_documentation_consistency(
    markdown: &mut String,
    diagnostics: &FacadeDiagnostics,
) {
    let _ = diagnostics;

    markdown.push_str("## Crate Documentation Consistency\n\n");
    markdown
        .push_str("- Deferred until facade diagnostics exposes documentation display rows.\n\n");
}

pub(crate) fn write_documentation_surface(markdown: &mut String, diagnostics: &FacadeDiagnostics) {
    let _ = diagnostics;

    markdown.push_str("## Documentation Surface\n\n");
    markdown
        .push_str("- Deferred until facade diagnostics exposes documentation surface data.\n\n");
}
