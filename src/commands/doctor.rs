use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::Args;

use crate::output::Output;
use crate::rustuse::{config, project};

#[derive(Debug, Args)]
pub struct DoctorArgs {
    /// Directory to inspect.
    #[arg(default_value = ".", value_name = "PATH")]
    pub path: PathBuf,
}

pub fn run(args: DoctorArgs, output: Output) -> Result<()> {
    let root = std::fs::canonicalize(&args.path)
        .with_context(|| format!("failed to resolve `{}`", args.path.display()))?;

    let state = project::detect(&root);
    let config_result = read_config(&state.config_path);

    if output.is_json() {
        let config_status = match &config_result {
            Ok(Some(_)) => "valid",
            Ok(None) => "missing",
            Err(_) => "invalid",
        };

        output.record(
            "doctor",
            status_for_state(&state, &config_result),
            &format!(
                "path={}, cargo_toml={}, rustuse_toml={}, rustuse_config={}, rustuse_lock={}, rustuse_dir={}, cache={}, snapshots={}",
                root.display(),
                state.has_cargo_toml,
                state.has_config,
                config_status,
                state.has_lock,
                state.has_state_dir,
                state.has_cache_dir,
                state.has_snapshots_dir
            ),
        );

        return Ok(());
    }

    output.line("RustUse doctor");
    output.line(format!("Root: {}", root.display()));
    output.line("");

    output.line(format!(
        "Cargo.toml: {} ({})",
        status_label(state.has_cargo_toml),
        state.cargo_toml_path.display()
    ));

    output.line(format!(
        "rustuse.toml: {} ({})",
        status_label(state.has_config),
        state.config_path.display()
    ));

    match config_result {
        Ok(Some(config)) => {
            output.line("rustuse.toml status: valid");
            output.line(format!("Project name: {}", config.project.name));
            output.line(format!("Project kind: {}", config.project.kind));
            output.line(format!(
                "Default adoption: {}",
                config.project.default_adoption
            ));
        },
        Ok(None) => {
            output.line("rustuse.toml status: missing");
        },
        Err(error) => {
            output.line(format!("rustuse.toml status: invalid ({error})"));
        },
    }

    output.line(format!(
        "rustuse.lock: {} ({})",
        status_label(state.has_lock),
        state.lock_path.display()
    ));

    output.line(format!(
        ".rustuse/: {} ({})",
        status_label(state.has_state_dir),
        state.state_dir_path.display()
    ));

    output.line(format!(
        ".rustuse/cache/: {} ({})",
        status_label(state.has_cache_dir),
        state.cache_dir_path.display()
    ));

    output.line(format!(
        ".rustuse/snapshots/: {} ({})",
        status_label(state.has_snapshots_dir),
        state.snapshots_dir_path.display()
    ));

    output.line("");
    output.line("CLI mode: rustuse helps apply Cargo boundaries; it is not a package manager.");

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

fn status_for_state(
    state: &project::ProjectState,
    config_result: &Result<Option<config::RustUseConfig>>,
) -> &'static str {
    if config_result.is_err() {
        return "error";
    }

    if state.has_cargo_toml || state.has_config || state.has_state_dir {
        "ok"
    } else {
        "warning"
    }
}

fn status_label(present: bool) -> &'static str {
    if present { "found" } else { "missing" }
}
