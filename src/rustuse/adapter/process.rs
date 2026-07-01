use std::{
    path::Path,
    process::{Command, Stdio},
};

use anyhow::{Context, Result, bail};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CommandOutput {
    pub(crate) stdout: String,
    pub(crate) stderr: String,
    pub(crate) status_code: Option<i32>,
}

pub(crate) fn run_checked(cwd: &Path, program: &str, args: &[&str]) -> Result<CommandOutput> {
    let output = Command::new(program)
        .args(args)
        .current_dir(cwd)
        .stdin(Stdio::null())
        .output()
        .with_context(|| {
            format!(
                "failed to run `{}` in `{}`",
                format_command(program, args),
                cwd.display()
            )
        })?;

    let command_output = CommandOutput {
        stdout: String::from_utf8_lossy(&output.stdout).trim().to_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).trim().to_owned(),
        status_code: output.status.code(),
    };

    if !output.status.success() {
        bail!(
            "command failed: `{}` in `{}`\nstatus: {}\nstdout:\n{}\nstderr:\n{}",
            format_command(program, args),
            cwd.display(),
            command_output.status_code.map_or_else(
                || "terminated by signal".to_owned(),
                |code| code.to_string()
            ),
            empty_label(&command_output.stdout),
            empty_label(&command_output.stderr),
        );
    }

    Ok(command_output)
}

fn format_command(program: &str, args: &[&str]) -> String {
    if args.is_empty() {
        return program.to_owned();
    }

    format!("{} {}", program, args.join(" "))
}

fn empty_label(value: &str) -> &str {
    if value.is_empty() { "<empty>" } else { value }
}
