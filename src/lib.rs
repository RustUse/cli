//! `RustUse` command-line adoption helper.
//!
//! This crate powers the `rustuse` and `cargo-rustuse` binaries.
//!
//! Runtime entry points are [`run`], [`run_cargo_subcommand`], and [`run_from`].
//! Command modules adapt CLI arguments into `RustUse` workflows.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

use std::ffi::{OsStr, OsString};

use anyhow::Result;
use clap::Parser;

use crate::cli::Cli;

mod cli;
mod commands;
mod output;
mod rustuse;

/// Runs the `rustuse` CLI using arguments from the current process.
///
/// # Errors
///
/// Returns an error if the selected command cannot complete successfully,
/// including failures involving configuration, filesystem access, external
/// commands, or output rendering.
pub fn run() -> Result<()> {
    run_from(std::env::args_os())
}

/// Runs `RustUse` as the `cargo rustuse` subcommand.
///
/// Cargo may invoke a subcommand binary with an additional `rustuse` argument.
/// This entry point normalizes that argument shape before parsing the command.
///
/// # Errors
///
/// Returns an error if the selected command cannot complete successfully,
/// including failures involving configuration, filesystem access, external
/// commands, or output rendering.
pub fn run_cargo_subcommand() -> Result<()> {
    run_from(cargo_subcommand_args())
}

/// Runs the `RustUse` CLI using the provided command-line arguments.
///
/// This entry point supports embedding and testing the CLI without reading
/// arguments directly from the current process.
///
/// # Errors
///
/// Returns an error if the selected command cannot complete successfully,
/// including failures involving configuration, filesystem access, external
/// commands, or output rendering.
pub fn run_from<I, T>(args: I) -> Result<()>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let cli = Cli::parse_from(args);
    commands::run(cli)
}

fn cargo_subcommand_args() -> Vec<OsString> {
    normalize_cargo_subcommand_args(std::env::args_os())
}

fn normalize_cargo_subcommand_args<I, T>(raw_args: I) -> Vec<OsString>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString>,
{
    let mut raw_args = raw_args.into_iter().map(Into::into);
    let mut args = Vec::new();

    if let Some(binary) = raw_args.next() {
        args.push(binary);
    }

    match raw_args.next() {
        Some(first) if first.as_os_str() == OsStr::new("rustuse") => {},
        Some(first) => args.push(first),
        None => {},
    }

    args.extend(raw_args);
    args
}

#[cfg(test)]
mod tests {
    use super::*;

    fn os_args(args: &[&str]) -> Vec<OsString> {
        args.iter().map(OsString::from).collect()
    }

    #[test]
    fn direct_cargo_rustuse_args_are_preserved() {
        assert_eq!(
            normalize_cargo_subcommand_args(os_args(&["cargo-rustuse", "report", "."])),
            os_args(&["cargo-rustuse", "report", "."])
        );
    }

    #[test]
    fn cargo_subcommand_name_is_stripped() {
        assert_eq!(
            normalize_cargo_subcommand_args(os_args(&["cargo-rustuse", "rustuse", "report", ".",])),
            os_args(&["cargo-rustuse", "report", "."])
        );
    }

    #[test]
    fn cargo_help_subcommand_shape_is_supported() {
        assert_eq!(
            normalize_cargo_subcommand_args(os_args(&["cargo-rustuse", "rustuse", "--help",])),
            os_args(&["cargo-rustuse", "--help"])
        );
    }
}
