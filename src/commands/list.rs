use anyhow::{Context, Result};

use crate::config;
use crate::output::Output;
use crate::project;

pub fn run(output: Output) -> Result<()> {
    let current_dir = std::env::current_dir().context("failed to read current directory")?;
    let state = project::detect(&current_dir);
    if !state.has_config {
        let message = format!(
            "No rustuse.toml found at {}. Run `rustuse init` to enable managed RustUse tracking.",
            state.config_path.display()
        );
        output.record("list", "missing", &message);
        return Ok(());
    };

    let raw = std::fs::read_to_string(&state.config_path)
        .with_context(|| format!("failed to read {}", state.config_path.display()))?;
    let rustuse_config = config::from_toml(&raw)
        .with_context(|| format!("failed to load {}", state.config_path.display()))?;

    if output.is_json() {
        let message = format!(
            "version={}, default_mode={}, primitives={}",
            rustuse_config.version,
            rustuse_config.project.default_mode,
            rustuse_config.primitives.len()
        );
        output.record("list", "ok", &message);
        return Ok(());
    }

    output.line(format!("RustUse config: {}", state.config_path.display()));
    output.line(format!("config version: {}", rustuse_config.version));
    output.line(format!(
        "default mode: {}",
        rustuse_config.project.default_mode
    ));
    output.line(format!("copy root: {}", rustuse_config.project.copy_root));
    output.line(format!("test root: {}", rustuse_config.project.test_root));

    if rustuse_config.primitives.is_empty() {
        output.line("No copied primitives are tracked yet.");
    } else {
        output.line("Tracked primitives:");
        for primitive in rustuse_config.primitives {
            output.line(format!(
                "- {} ({}) tests={} examples={}",
                primitive.name, primitive.mode, primitive.with_tests, primitive.with_examples
            ));
        }
    }

    Ok(())
}
