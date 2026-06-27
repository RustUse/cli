//! RustUse command-line adoption helper.
//!
//! This crate powers the `rustuse` and `cargo-rustuse` binaries.
//!
//! The stable runtime entry points are [`run`] and [`run_cargo_subcommand`].
//! Most modules are internal command adapters and development utilities.
//!
//! Use this while working on CLI internals:
//!
//! ```text
//! cargo doc --no-deps --document-private-items --open
//! ```

#![forbid(unsafe_code)]

use std::ffi::{OsStr, OsString};

mod cli;
mod commands;
mod config;
// mod dev;
mod index;
mod manifest;
mod output;
mod project;

use anyhow::Result;
use clap::Parser;

use crate::cli::Cli;

pub fn run() -> Result<()> {
    run_from(std::env::args_os())
}

pub fn run_cargo_subcommand() -> Result<()> {
    run_from(cargo_subcommand_args())
}

pub fn run_from<I, T>(args: I) -> Result<()>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let cli = Cli::parse_from(args);
    commands::run(cli)
}

fn cargo_subcommand_args() -> Vec<OsString> {
    let mut raw_args = std::env::args_os();
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
