use anyhow::{Context, Result, bail};
use clap::Args;
// use std::path::PathBuf;

// use crate::cli::InitArgs;
use crate::config::{self, RustUseConfig};
use crate::output::Output;
use crate::project::{self, CONFIG_FILE};

#[derive(Debug, Args)]
pub struct InitArgs {
    /// Prefer copy-mode defaults in rustuse.toml.
    #[arg(long)]
    pub copy_first: bool,

    /// Prefer Cargo-mode defaults in rustuse.toml.
    #[arg(long)]
    pub cargo_first: bool,

    /// Accept the v0.1 defaults without prompting.
    #[arg(long)]
    pub yes: bool,

    /// Show what would be created without writing files.
    #[arg(long)]
    pub dry_run: bool,

    /// Override the configured copy root.
    #[arg(long, value_name = "PATH")]
    pub copy_root: Option<String>,

    /// Override the configured test root.
    #[arg(long, value_name = "PATH")]
    pub test_root: Option<String>,
}

pub fn run(args: InitArgs, output: Output) -> Result<()> {
    if args.copy_first && args.cargo_first {
        bail!("--copy-first and --cargo-first cannot be used together");
    }

    let current_dir = std::env::current_dir().context("failed to read current directory")?;
    let state = project::detect(&current_dir);
    let config = config_for(&args);

    if state.has_config {
        let message = format!(
            "{} already exists; no changes made",
            state.config_path.display()
        );
        output.record("init", "exists", &message);
        return Ok(());
    }

    if args.yes {
        output.detail("--yes accepted; rustuse init has no interactive prompts in v0.1.");
    }

    if args.dry_run {
        print_plan(output, &state, &config, true);
        return Ok(());
    }

    let raw = config::to_toml(&config)?;
    project::write_config_new(&current_dir, &raw)?;
    project::create_tracking_dirs(&current_dir)?;
    let state = project::detect(&current_dir);

    print_plan(output, &state, &config, false);

    Ok(())
}

fn config_for(args: &InitArgs) -> RustUseConfig {
    let config = if args.copy_first {
        RustUseConfig::copy_first()
    } else {
        RustUseConfig::cargo_first()
    };

    config.with_roots(args.copy_root.clone(), args.test_root.clone())
}

fn print_plan(
    output: Output,
    state: &project::ProjectState,
    config: &RustUseConfig,
    dry_run: bool,
) {
    if output.is_json() {
        let status = if dry_run { "dry-run" } else { "ok" };
        let message = format!(
            "config={}, cache={}, snapshots={}, default_mode={}, cargo_toml={}, rustuse_lock_created=false",
            state.config_path.display(),
            state.cache_dir_path.display(),
            state.snapshots_dir_path.display(),
            config.project.default_mode,
            state.has_cargo_toml
        );
        output.record("init", status, &message);
        return;
    }

    if dry_run {
        output.line("Would initialize RustUse project tracking.");
        output.line("");
        output.line("Would create:");
    } else {
        output.line("Initialized RustUse project tracking.");
        output.line("");
        output.line("Created:");
    }

    output.line(format!("  {CONFIG_FILE}"));
    output.line("  .rustuse/cache/");
    output.line("  .rustuse/snapshots/");
    output.line("");
    output.line("Detected:");
    if state.has_cargo_toml {
        output.line("  Cargo.toml found");
    } else {
        output.line("  Cargo.toml missing");
    }
    output.line("");

    if !state.has_cargo_toml {
        output.line("Note:");
        output.line(
            "  No Cargo.toml was found. Cargo mode will work after this directory contains a Rust project.",
        );
        output.line("");
    }

    output.line("Default mode:");
    output.line(format!("  {}", config.project.default_mode));
    output.line("");

    if dry_run {
        output.line("No changes made because --dry-run was used.");
        output.line("");
    }

    output.line("Next:");
    output.line("  rustuse search geometry");
    output.line("  rustuse add use-geometry");
    output.line("  rustuse copy use-slug");
    output.line("  rustuse add use-slug --copy");
}
