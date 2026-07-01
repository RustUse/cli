use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use clap::Args;

use crate::output::Output;
use crate::rustuse::{
    config::{self, RustUseConfig},
    project::{self, CONFIG_FILE},
};

#[derive(Debug, Args)]
pub struct InitArgs {
    /// Directory to initialize.
    #[arg(default_value = ".", value_name = "PATH")]
    pub path: PathBuf,

    /// Prefer copy-mode defaults in rustuse.toml.
    #[arg(long)]
    pub copy_first: bool,

    /// Prefer Cargo-mode defaults in rustuse.toml.
    #[arg(long)]
    pub cargo_first: bool,

    /// Show what would be created without writing files.
    #[arg(long)]
    pub dry_run: bool,

    /// Overwrite existing rustuse.toml if present.
    #[arg(long)]
    pub force: bool,
}

pub fn run(args: InitArgs, output: Output) -> Result<()> {
    if args.copy_first && args.cargo_first {
        bail!("--copy-first and --cargo-first cannot be used together");
    }

    let root = std::fs::canonicalize(&args.path)
        .with_context(|| format!("failed to resolve `{}`", args.path.display()))?;

    let state = project::detect(&root);
    let project_name = project_name(&root);
    let config = config_for(&args, project_name);

    if state.has_config && !args.force {
        let message = format!(
            "{} already exists; no changes made",
            state.config_path.display()
        );

        output.record("init", "exists", &message);
        return Ok(());
    }

    let contents = config::to_toml(&config)?;

    if args.dry_run {
        print_plan(output, &state, &config, true, args.force);
        return Ok(());
    }

    if args.force {
        std::fs::write(&state.config_path, contents)
            .with_context(|| format!("failed to write `{}`", state.config_path.display()))?;
    } else {
        project::write_config_new(&root, &contents)?;
    }

    project::create_tracking_dirs(&root)?;

    let state = project::detect(&root);
    print_plan(output, &state, &config, false, args.force);

    Ok(())
}

fn config_for(args: &InitArgs, project_name: String) -> RustUseConfig {
    if args.copy_first {
        RustUseConfig::copy_first(project_name)
    } else {
        RustUseConfig::cargo_first(project_name)
    }
}

fn project_name(root: &Path) -> String {
    root.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("rustuse-project")
        .to_owned()
}

fn print_plan(
    output: Output,
    state: &project::ProjectState,
    config: &RustUseConfig,
    dry_run: bool,
    force: bool,
) {
    if output.is_json() {
        let status = if dry_run { "dry-run" } else { "ok" };
        let message = format!(
            "config={}, cache={}, snapshots={}, project={}, kind={}, default_adoption={}, cargo_toml={}, force={}",
            state.config_path.display(),
            state.cache_dir_path.display(),
            state.snapshots_dir_path.display(),
            config.project.name,
            config.project.kind,
            config.project.default_adoption,
            state.has_cargo_toml,
            force
        );

        output.record("init", status, &message);
        return;
    }

    if dry_run {
        output.line("Would initialize RustUse project tracking.");
    } else if force {
        output.line("Initialized RustUse project tracking and rewrote rustuse.toml.");
    } else {
        output.line("Initialized RustUse project tracking.");
    }

    output.line("");
    output.line(if dry_run { "Would create:" } else { "Created:" });
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

    output.line("Project:");
    output.line(format!("  name: {}", config.project.name));
    output.line(format!("  kind: {}", config.project.kind));
    output.line(format!(
        "  default adoption: {}",
        config.project.default_adoption
    ));
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
