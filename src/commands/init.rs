//! Initializes optional RustUse tracking for a Cargo project.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::Args;
use serde::Serialize;

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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
enum InitStatus {
    Exists,
    Planned,
    Ok,
}

#[derive(Debug, Serialize)]
struct InitResponse {
    command: &'static str,
    status: InitStatus,
    dry_run: bool,
    force: bool,
    paths: InitPaths,
    detected: InitDetected,
    project: InitProject,
}

#[derive(Debug, Serialize)]
struct InitPaths {
    config: PathBuf,
    cache: PathBuf,
    snapshots: PathBuf,
}

#[derive(Debug, Serialize)]
struct InitDetected {
    cargo_toml: bool,
}

#[derive(Debug, Serialize)]
struct InitProject {
    name: String,
    kind: String,
    default_adoption: String,
}

pub fn run(args: InitArgs, output: Output) -> Result<()> {
    let root = std::fs::canonicalize(&args.path)
        .with_context(|| format!("failed to resolve `{}`", args.path.display()))?;

    let state = project::detect(&root);
    let project_name = project_name(&root);
    let config = config_for(project_name);

    if state.has_config && !args.force {
        let response = response_for(&state, &config, InitStatus::Exists, false, args.force);

        return render(&output, &response);
    }

    let contents = config::to_toml(&config)?;

    if args.dry_run {
        let response = response_for(&state, &config, InitStatus::Planned, true, args.force);

        return render(&output, &response);
    }

    if args.force {
        std::fs::write(&state.config_path, contents)
            .with_context(|| format!("failed to write `{}`", state.config_path.display()))?;
    } else {
        project::write_config_new(&root, &contents)?;
    }

    project::create_tracking_dirs(&root)?;

    let state = project::detect(&root);
    let response = response_for(&state, &config, InitStatus::Ok, false, args.force);

    render(&output, &response)
}

fn config_for(project_name: String) -> RustUseConfig {
    RustUseConfig::cargo_first(project_name)
}

fn project_name(root: &Path) -> String {
    root.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("rustuse-project")
        .to_owned()
}

fn response_for(
    state: &project::ProjectState,
    config: &RustUseConfig,
    status: InitStatus,
    dry_run: bool,
    force: bool,
) -> InitResponse {
    InitResponse {
        command: "init",
        status,
        dry_run,
        force,
        paths: InitPaths {
            config: state.config_path.clone(),
            cache: state.cache_dir_path.clone(),
            snapshots: state.snapshots_dir_path.clone(),
        },
        detected: InitDetected {
            cargo_toml: state.has_cargo_toml,
        },
        project: InitProject {
            name: config.project.name.clone(),
            kind: config.project.kind.to_string(),
            default_adoption: config.project.default_adoption.to_string(),
        },
    }
}

fn render(output: &Output, response: &InitResponse) -> Result<()> {
    if output.is_json() {
        return output.json(response);
    }

    match response.status {
        InitStatus::Exists => {
            return output.line(format!(
                "{} already exists; no changes made",
                response.paths.config.display()
            ));
        },
        InitStatus::Planned => {
            output.line("Would initialize RustUse project tracking.")?;
        },
        InitStatus::Ok if response.force => {
            output.line("Initialized RustUse project tracking and rewrote rustuse.toml.")?;
        },
        InitStatus::Ok => {
            output.line("Initialized RustUse project tracking.")?;
        },
    }

    output.line("")?;
    output.line(if response.dry_run {
        "Would create:"
    } else {
        "Created:"
    })?;
    output.line(format!("  {CONFIG_FILE}"))?;
    output.line("  .rustuse/cache/")?;
    output.line("  .rustuse/snapshots/")?;
    output.line("")?;

    output.line("Detected:")?;

    if response.detected.cargo_toml {
        output.line("  Cargo.toml found")?;
    } else {
        output.line("  Cargo.toml missing")?;
    }

    output.line("")?;

    if !response.detected.cargo_toml {
        output.line("Note:")?;
        output.line(
            "  No Cargo.toml was found. Cargo mode will work after this directory contains a Rust project.",
        )?;
        output.line("")?;
    }

    output.line("Project:")?;
    output.line(format!("  name: {}", response.project.name))?;
    output.line(format!("  kind: {}", response.project.kind))?;
    output.line(format!(
        "  default adoption: {}",
        response.project.default_adoption
    ))?;
    output.line("")?;

    if response.dry_run {
        output.line("No changes made because --dry-run was used.")?;
        output.line("")?;
    }

    output.line("Next:")?;
    output.line("  rustuse search geometry")?;
    output.line("  rustuse add use-geometry")
}
