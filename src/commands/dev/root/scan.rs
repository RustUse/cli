//! Root scan command for listing local RustUse facade repositories.

use std::fs;

use anyhow::Result;

use super::DevRootPathArgs;
use super::discover::{FacadeEntry, discover_facades, display_version};
use crate::output::Output;

pub(crate) fn run(args: DevRootPathArgs, output: Output) -> Result<()> {
    let root = fs::canonicalize(&args.root).unwrap_or(args.root);
    let facades = discover_facades(&root)?;
    let summary = ScanSummary::from_facades(&facades);

    output.line(format!("RustUse dev root scan - root: {}", root.display()));
    output.line(format!("found {} use-* directories", facades.len()));
    output.line(format!("facade repos with .git: {}", summary.repo_count));
    output.line(format!(
        "facades missing .git: {}",
        summary.missing_git_count
    ));
    output.line(format!(
        "child crates detected: {}",
        summary.child_crate_count
    ));
    output.line("");

    print_facade_table(&facades, output);

    if summary.missing_git_count > 0 {
        print_missing_git_facades(&facades, output);
    }

    output.line("");
    output.line(format!("status: {}", summary.status()));

    Ok(())
}

#[derive(Debug)]
struct ScanSummary {
    repo_count: usize,
    missing_git_count: usize,
    child_crate_count: usize,
    warning_count: usize,
    is_empty: bool,
}

impl ScanSummary {
    fn from_facades(facades: &[FacadeEntry]) -> Self {
        let repo_count = facades.iter().filter(|facade| facade.has_git()).count();
        let missing_git_count = facades.len().saturating_sub(repo_count);
        let child_crate_count = facades.iter().map(FacadeEntry::child_crate_count).sum();
        let warning_count = facades
            .iter()
            .filter(|facade| facade.status() != "ok")
            .count();

        Self {
            repo_count,
            missing_git_count,
            child_crate_count,
            warning_count,
            is_empty: facades.is_empty(),
        }
    }

    fn status(&self) -> &'static str {
        if self.is_empty || self.warning_count > 0 {
            "warning"
        } else {
            "ok"
        }
    }
}

fn print_facade_table(facades: &[FacadeEntry], output: Output) {
    output.line(format!(
        "{:<8} {:<24} {:<10} {:<5} {:<10} {:<8} {:>6}",
        "Status", "Facade", "Version", "Git", "Cargo.toml", "crates/", "Children"
    ));

    output.line(format!(
        "{:<8} {:<24} {:<10} {:<5} {:<10} {:<8} {:>6}",
        "------", "------", "-------", "---", "----------", "-------", "--------"
    ));

    for facade in facades {
        output.line(format!(
            "{:<8} {:<24} {:<10} {:<5} {:<10} {:<8} {:>6}",
            facade.status(),
            facade.name,
            display_version(&facade.version),
            yes_no(facade.has_git()),
            yes_no(facade.has_cargo_toml()),
            yes_no(facade.has_crates_dir()),
            facade.child_crate_count(),
        ));
    }
}

fn print_missing_git_facades(facades: &[FacadeEntry], output: Output) {
    output.line("");
    output.line("facades missing .git:");

    for facade in facades.iter().filter(|facade| !facade.has_git()) {
        output.line(format!("- {}", facade.name));
    }
}

fn yes_no(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}
