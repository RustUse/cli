//! Output formatting and presentation utilities for the RustUse CLI.
//!
//! This module centralizes human-readable and machine-readable command output,
//! including plain text, JSON responses, quiet mode, and verbose diagnostics.
//! Command implementations should use [`Output`] rather than writing directly
//! to standard output so that formatting remains consistent across the CLI.
//!
//! Human-readable command results are written to standard output, while
//! verbose diagnostic information is written to standard error. In JSON mode,
//! standard output should contain only valid machine-readable JSON.

use std::io::{self, Write};

use anyhow::{Context, Result};
use serde::Serialize;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OutputFormat {
    Plain,
    Json,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Verbosity {
    Quiet,
    Normal,
    Verbose,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Output {
    format: OutputFormat,
    verbosity: Verbosity,
}

#[derive(Debug, Serialize)]
struct CommandRecord<'a> {
    command: &'a str,
    status: &'a str,
    message: &'a str,
}

impl Output {
    #[must_use]
    pub const fn new(json: bool, quiet: bool, verbose: bool) -> Self {
        let format = if json {
            OutputFormat::Json
        } else {
            OutputFormat::Plain
        };

        let verbosity = if quiet {
            Verbosity::Quiet
        } else if verbose {
            Verbosity::Verbose
        } else {
            Verbosity::Normal
        };

        Self { format, verbosity }
    }

    /* #[must_use]
    pub const fn format(&self) -> OutputFormat {
        self.format
    } */

    #[must_use]
    pub const fn is_json(&self) -> bool {
        matches!(self.format, OutputFormat::Json)
    }

    #[must_use]
    pub const fn is_quiet(&self) -> bool {
        matches!(self.verbosity, Verbosity::Quiet)
    }

    #[must_use]
    pub const fn is_verbose(&self) -> bool {
        matches!(self.verbosity, Verbosity::Verbose)
    }

    /// Writes a human-readable command result to stdout.
    ///
    /// Human-readable lines are suppressed in JSON and quiet modes.
    pub fn line(&self, message: impl AsRef<str>) -> Result<()> {
        if self.is_json() || self.is_quiet() {
            return Ok(());
        }

        let stdout = io::stdout();
        let mut writer = stdout.lock();

        Self::write_line(&mut writer, message.as_ref()).context("failed to write command output")
    }

    /// Writes verbose diagnostic information to stderr.
    pub fn detail(&self, message: impl AsRef<str>) -> Result<()> {
        if !self.is_verbose() {
            return Ok(());
        }

        let stderr = io::stderr();
        let mut writer = stderr.lock();

        Self::write_line(&mut writer, message.as_ref()).context("failed to write verbose output")
    }

    /// Writes one JSON value followed by a newline.
    ///
    /// Commands should generally call this once per invocation. Streaming
    /// output should eventually use an explicit JSON Lines format.
    pub fn json<T>(&self, value: &T) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        let stdout = io::stdout();
        let mut writer = stdout.lock();

        Self::write_json(&mut writer, value)
    }

    /// Writes a simple command result.
    ///
    /// This is useful for commands with no command-specific response data.
    /// Commands with structured results should use `json`.
    pub fn record(&self, command: &str, status: &str, message: &str) -> Result<()> {
        if self.is_json() {
            return self.json(&CommandRecord {
                command,
                status,
                message,
            });
        }

        self.line(message)
    }

    fn write_line(writer: &mut impl Write, message: &str) -> Result<()> {
        writeln!(writer, "{message}").context("failed to write output")
    }

    fn write_json<T>(writer: &mut impl Write, value: &T) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        serde_json::to_writer(&mut *writer, value).context("failed to serialize JSON output")?;

        writer
            .write_all(b"\n")
            .context("failed to terminate JSON output")
    }
}
