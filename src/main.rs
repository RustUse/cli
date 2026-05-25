#![forbid(unsafe_code)]

mod cli;
mod commands;
mod config;
mod index;
mod manifest;
mod output;
mod project;

use anyhow::Result;
use clap::Parser;

use crate::cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    commands::run(cli)
}
