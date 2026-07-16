//! Diagnoses RustUse installation, project, and environment problems.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::Args;
use serde::Serialize;

use crate::output::Output;
use crate::rustuse::{config, project};

#[derive(Debug, Args)]
pub struct DoctorArgs {
    /// Directory to inspect.
    #[arg(default_value = ".", value_name = "PATH")]
    pub path: PathBuf,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum DoctorStatus {
    Ok,
    Warning,
    Error,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum ConfigStatus {
    Valid,
    Missing,
    Invalid,
}

#[derive(Debug, Serialize)]
struct DoctorResponse {
    command: &'static str,
    status: DoctorStatus,
    root: PathBuf,
    cargo_toml: PathCheck,
    rustuse_toml: ConfigCheck,
    rustuse_lock: PathCheck,
    rustuse_dir: PathCheck,
    cache_dir: PathCheck,
    snapshots_dir: PathCheck,
    cli_mode: &'static str,
}

#[derive(Debug, Serialize)]
struct PathCheck {
    present: bool,
    path: PathBuf,
}

#[derive(Debug, Serialize)]
struct ConfigCheck {
    present: bool,
    path: PathBuf,
    status: ConfigStatus,
    project: Option<ProjectDetails>,
    error: Option<String>,
}

#[derive(Debug, Serialize)]
struct ProjectDetails {
    name: String,
    kind: String,
    default_adoption: String,
}

pub fn run(args: DoctorArgs, output: Output) -> Result<()> {
    let root = std::fs::canonicalize(&args.path)
        .with_context(|| format!("failed to resolve `{}`", args.path.display()))?;

    let state = project::detect(&root);
    let config_result = read_config(&state.config_path);

    let (config_status, project_details, config_error) = match config_result {
        Ok(Some(config)) => (
            ConfigStatus::Valid,
            Some(ProjectDetails {
                name: config.project.name,
                kind: config.project.kind.to_string(),
                default_adoption: config.project.default_adoption.to_string(),
            }),
            None,
        ),
        Ok(None) => (ConfigStatus::Missing, None, None),
        Err(error) => (ConfigStatus::Invalid, None, Some(error.to_string())),
    };

    let status = status_for_state(&state, config_status);

    let response = DoctorResponse {
        command: "doctor",
        status,
        root,
        cargo_toml: PathCheck {
            present: state.has_cargo_toml,
            path: state.cargo_toml_path,
        },
        rustuse_toml: ConfigCheck {
            present: state.has_config,
            path: state.config_path,
            status: config_status,
            project: project_details,
            error: config_error,
        },
        rustuse_lock: PathCheck {
            present: state.has_lock,
            path: state.lock_path,
        },
        rustuse_dir: PathCheck {
            present: state.has_state_dir,
            path: state.state_dir_path,
        },
        cache_dir: PathCheck {
            present: state.has_cache_dir,
            path: state.cache_dir_path,
        },
        snapshots_dir: PathCheck {
            present: state.has_snapshots_dir,
            path: state.snapshots_dir_path,
        },
        cli_mode: "rustuse helps apply Cargo boundaries; it is not a package manager",
    };

    render(&output, &response)
}

fn render(output: &Output, response: &DoctorResponse) -> Result<()> {
    if output.is_json() {
        return output.json(response);
    }

    output.line("RustUse doctor")?;
    output.line(format!("Root: {}", response.root.display()))?;
    output.line("")?;

    render_path_check(output, "Cargo.toml", &response.cargo_toml)?;
    render_config_check(output, &response.rustuse_toml)?;
    render_path_check(output, "rustuse.lock", &response.rustuse_lock)?;
    render_path_check(output, ".rustuse/", &response.rustuse_dir)?;
    render_path_check(output, ".rustuse/cache/", &response.cache_dir)?;
    render_path_check(output, ".rustuse/snapshots/", &response.snapshots_dir)?;

    output.line("")?;
    output.line(format!("CLI mode: {}.", response.cli_mode))
}

fn render_path_check(output: &Output, label: &str, check: &PathCheck) -> Result<()> {
    output.line(format!(
        "{label}: {} ({})",
        status_label(check.present),
        check.path.display()
    ))
}

fn render_config_check(output: &Output, check: &ConfigCheck) -> Result<()> {
    output.line(format!(
        "rustuse.toml: {} ({})",
        status_label(check.present),
        check.path.display()
    ))?;

    match check.status {
        ConfigStatus::Valid => {
            output.line("rustuse.toml status: valid")?;

            if let Some(project) = &check.project {
                output.line(format!("Project name: {}", project.name))?;
                output.line(format!("Project kind: {}", project.kind))?;
                output.line(format!("Default adoption: {}", project.default_adoption))?;
            }
        },
        ConfigStatus::Missing => {
            output.line("rustuse.toml status: missing")?;
        },
        ConfigStatus::Invalid => {
            let error = check.error.as_deref().unwrap_or("unknown error");
            output.line(format!("rustuse.toml status: invalid ({error})"))?;
        },
    }

    Ok(())
}

fn read_config(path: &Path) -> Result<Option<config::RustUseConfig>> {
    if !path.exists() {
        return Ok(None);
    }

    let raw = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read `{}`", path.display()))?;

    let config =
        config::from_toml(&raw).with_context(|| format!("failed to parse `{}`", path.display()))?;

    Ok(Some(config))
}

const fn status_for_state(
    state: &project::ProjectState,
    config_status: ConfigStatus,
) -> DoctorStatus {
    if matches!(config_status, ConfigStatus::Invalid) {
        return DoctorStatus::Error;
    }

    if state.has_cargo_toml || state.has_config || state.has_state_dir {
        DoctorStatus::Ok
    } else {
        DoctorStatus::Warning
    }
}

const fn status_label(present: bool) -> &'static str {
    if present { "found" } else { "missing" }
}
