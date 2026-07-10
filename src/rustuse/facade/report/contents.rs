pub(crate) fn write_contents(markdown: &mut String) {
    markdown.push_str("## Contents\n\n");
    markdown.push_str("- [Action Plan](#action-plan)\n");
    markdown.push_str("- [Expected Facade Structure](#expected-facade-structure)\n");
    markdown.push_str("- [Facade Shape](#facade-shape)\n");
    markdown.push_str("- [Repository Surface](#repository-surface)\n");
    markdown.push_str("- [Cargo Manifest Health](#cargo-manifest-health)\n");
    markdown.push_str("- [Child Crates](#child-crates)\n");
    markdown.push_str("- [Crate Documentation Consistency](#crate-documentation-consistency)\n");
    markdown.push_str("- [Standard File Consistency](#standard-file-consistency)\n");
    markdown.push_str("- [Non-standard Paths](#non-standard-paths)\n");
    markdown.push_str("- [Tooling Configuration](#tooling-configuration)\n");
    markdown.push_str("- [CI/CD Surface](#cicd-surface)\n");
    markdown.push_str("- [Documentation Surface](#documentation-surface)\n");
    markdown.push_str("- [Release Surface](#release-surface)\n");
    markdown.push_str("- [Notes](#notes)\n\n");
}
