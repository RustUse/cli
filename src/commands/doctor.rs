use anyhow::{Context, Result};

use crate::output::Output;
use crate::project;

pub fn run(output: Output) -> Result<()> {
    let current_dir = std::env::current_dir().context("failed to read current directory")?;
    let state = project::detect(&current_dir);

    if output.is_json() {
        let message = format!(
            "cargo_toml={}, rustuse_toml={}, rustuse_lock={}, rustuse_dir={}, cache={}, snapshots={}",
            state.has_cargo_toml,
            state.has_config,
            state.has_lock,
            state.has_state_dir,
            state.has_cache_dir,
            state.has_snapshots_dir
        );
        output.record("doctor", "ok", &message);
        return Ok(());
    }

    output.line("RustUse doctor");
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
    output.line(
        "CLI mode: rustuse helps apply Cargo or copy boundaries; it is not a package manager.",
    );

    Ok(())
}

fn status_label(present: bool) -> &'static str {
    if present { "found" } else { "missing" }
}
