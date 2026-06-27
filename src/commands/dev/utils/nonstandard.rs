//! Shared non-standard path checks for RustUse repositories.

use std::path::Path;

#[derive(Clone, Copy, Debug)]
pub(crate) enum NonStandardPathKind {
    Directory,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct NonStandardPathRule {
    pub(crate) path: &'static str,
    pub(crate) kind: NonStandardPathKind,
    pub(crate) recommendation: &'static str,
}

#[derive(Debug)]
pub(crate) struct NonStandardPathCheck {
    pub(crate) path: &'static str,
    pub(crate) recommendation: &'static str,
    pub(crate) present: bool,
}

#[derive(Debug)]
pub(crate) struct NonStandardPathReport {
    pub(crate) checks: Vec<NonStandardPathCheck>,
}

impl NonStandardPathReport {
    pub(crate) fn present_checks(&self) -> Vec<&NonStandardPathCheck> {
        self.checks.iter().filter(|check| check.present).collect()
    }

    pub(crate) fn total_count(&self) -> usize {
        self.checks.len()
    }

    pub(crate) fn present_count(&self) -> usize {
        self.checks.iter().filter(|check| check.present).count()
    }

    pub(crate) fn status(&self) -> &'static str {
        if self.present_count() == 0 {
            "ok"
        } else {
            "warning"
        }
    }
}

pub(crate) fn inspect_non_standard_paths(
    root: &Path,
    rules: &[NonStandardPathRule],
) -> NonStandardPathReport {
    let checks = rules
        .iter()
        .map(|rule| {
            let present = match rule.kind {
                NonStandardPathKind::Directory => root.join(rule.path).is_dir(),
            };

            NonStandardPathCheck {
                path: rule.path,
                recommendation: rule.recommendation,
                present,
            }
        })
        .collect();

    NonStandardPathReport { checks }
}
