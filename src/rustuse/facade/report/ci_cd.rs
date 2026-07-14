use crate::rustuse::facade::diagnostics::FacadeDiagnostics;
use crate::rustuse::report::markdown::{write_presence_table, yes_no};

pub(crate) fn write_ci_cd_surface(markdown: &mut String, diagnostics: &FacadeDiagnostics) {
    let github = &diagnostics.github_workflows;
    let gitlab = &diagnostics.gitlab;
    let release = &diagnostics.release;

    markdown.push_str("## CI/CD Surface\n\n");

    markdown.push_str(&format!("- Status: **{}**\n", github.status()));
    markdown.push_str(&format!(
        "- Required GitHub CI/CD surface: `{}/{}`\n",
        github.present_required_count(),
        github.required_count()
    ));
    markdown.push_str(&format!(
        "- Required GitHub workflows: `{}/{}`\n",
        github.present_workflow_count(),
        github.workflow_count()
    ));
    markdown.push_str(&format!(
        "- GitLab CI surface: `{}/{}`\n",
        gitlab.present_count(),
        gitlab.total_count()
    ));
    markdown.push_str(&format!(
        "- Release CI/CD surface: `{}/{}`\n\n",
        release.ci_present_count(),
        release.ci_total_count()
    ));

    write_missing_github_paths(markdown, diagnostics);
    write_github_surface(markdown, diagnostics);
    write_required_github_workflows(markdown, diagnostics);
    write_gitlab_surface(markdown, diagnostics);
    write_release_ci_surface(markdown, diagnostics);
}

fn write_missing_github_paths(markdown: &mut String, diagnostics: &FacadeDiagnostics) {
    let missing = diagnostics.github_workflows.missing_required_paths();

    if missing.is_empty() {
        return;
    }

    markdown.push_str("- Missing required CI/CD files:\n");

    for path in missing {
        markdown.push_str(&format!("  - `{path}`\n"));
    }

    markdown.push('\n');
}

fn write_github_surface(markdown: &mut String, diagnostics: &FacadeDiagnostics) {
    markdown.push_str("### GitHub CI/CD Surface\n\n");
    markdown.push_str("| Required | Surface | Present |\n");
    markdown.push_str("|---:|---|---:|\n");

    for check in &diagnostics.github_workflows.required_surface {
        markdown.push_str(&format!(
            "| yes | `{}` | {} |\n",
            check.path,
            yes_no(check.present)
        ));
    }

    markdown.push('\n');
}

fn write_required_github_workflows(markdown: &mut String, diagnostics: &FacadeDiagnostics) {
    markdown.push_str("### Required GitHub Workflows\n\n");
    markdown.push_str("| Workflow | Present |\n");
    markdown.push_str("|---|---:|\n");

    for check in &diagnostics.github_workflows.required_workflows {
        markdown.push_str(&format!(
            "| `{}` | {} |\n",
            check.path,
            yes_no(check.present)
        ));
    }

    markdown.push('\n');
}

fn write_gitlab_surface(markdown: &mut String, diagnostics: &FacadeDiagnostics) {
    markdown.push_str("### GitLab CI Surface\n\n");
    write_presence_table(markdown, &diagnostics.gitlab.surface);
}

fn write_release_ci_surface(markdown: &mut String, diagnostics: &FacadeDiagnostics) {
    markdown.push_str("### Release CI/CD Surface\n\n");
    write_presence_table(markdown, &diagnostics.release.ci_surface);
}
