use std::io::IsTerminal;

use anyhow::{Result, bail};
use dialoguer::{Select, theme::ColorfulTheme};

use crate::output::Output;

pub(crate) fn run(output: Output, non_interactive: bool) -> Result<()> {
    if non_interactive || !std::io::stdin().is_terminal() {
        bail!("a command is required when running non-interactively; try `rustuse --help`");
    }

    let choices = [
        "Add a RustUse crate",
        "Copy RustUse source",
        "Search RustUse crates",
        "Generate a report",
        "Scan a RustUse repository",
        "Inspect a facade",
        "Initialize RustUse tracking",
        "Run doctor checks",
        "Ask Ferris",
        "Exit",
    ];

    let selected = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What do you want to do?")
        .items(choices)
        .default(0)
        .interact()?;

    match selected {
        0 => crate::commands::placeholder(output, "add", "interactive add flow"),
        1 => crate::commands::placeholder(output, "copy", "interactive copy flow"),
        2 => crate::commands::placeholder(output, "search", "interactive search flow"),
        3 => crate::commands::placeholder(output, "report", "interactive report flow"),
        4 => crate::commands::placeholder(output, "scan", "interactive scan flow"),
        5 => crate::commands::placeholder(output, "facade", "interactive facade flow"),
        6 => crate::commands::placeholder(output, "init", "interactive init flow"),
        7 => crate::commands::placeholder(output, "doctor", "interactive doctor flow"),
        8 => crate::commands::placeholder(output, "ferris", "interactive ferris flow"),
        _ => Ok(()),
    }
}
