//! Guided interactive menu shown when `rustuse` runs without a subcommand.
//!
//! The menu collects command arguments and delegates execution to the
//! corresponding command modules.

use std::io::IsTerminal;
use std::path::PathBuf;

use anyhow::{Result, bail};
use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};

use crate::output::Output;

use super::NamedCommandArgs;
use super::add::{self, AddArgs, AddMode};
use super::catalog::{
    self, CatalogArgs, CatalogCommand, CatalogGenerateArgs, CatalogInfoArgs, CatalogPathArgs,
    CatalogSearchArgs,
};
use super::check::{self, CheckArgs};
use super::ci::check::{CheckCiArgs, CheckKind};
use super::ci::{self, CiArgs, CiCommand};
use super::dev::{self, DevArgs};
use super::diff::{self, DiffArgs};
use super::docs::{self, DocsArgs};
use super::doctor::{self, DoctorArgs};
use super::ferris::{self, FerrisArgs};
use super::info::{self, InfoArgs};
use super::init::{self, InitArgs};
use super::list::{self, ListArgs};
use super::outdated::{self, OutdatedArgs};
use super::remove::{self, RemoveArgs};
use super::search::{self, SearchArgs};
use super::update::{self, UpdateArgs};
use super::upgrade::{self, UpgradeArgs};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MenuAction {
    Search,
    Info,
    Add,
    Catalog,
    Check,
    Diff,
    Docs,
    Doctor,
    Init,
    List,
    Outdated,
    Remove,
    Update,
    Upgrade,
    DevelopmentAndCi,
    Ferris,
    Exit,
}

impl MenuAction {
    const ALL: [Self; 17] = [
        Self::Search,
        Self::Info,
        Self::Add,
        Self::Catalog,
        Self::Check,
        Self::Diff,
        Self::Docs,
        Self::Doctor,
        Self::Init,
        Self::List,
        Self::Outdated,
        Self::Remove,
        Self::Update,
        Self::Upgrade,
        Self::DevelopmentAndCi,
        Self::Ferris,
        Self::Exit,
    ];

    const fn label(self) -> &'static str {
        match self {
            Self::Search => "Search RustUse crates",
            Self::Info => "Show crate or primitive info",
            Self::Add => "Add a RustUse crate",
            Self::Catalog => "Manage the RustUse catalog",
            Self::Check => "Check a RustUse repository",
            Self::Diff => "Diff a RustUse repository",
            Self::Docs => "Show RustUse documentation",
            Self::Doctor => "Run doctor checks",
            Self::Init => "Initialize RustUse tracking",
            Self::List => "List RustUse entries",
            Self::Outdated => "Check for outdated RustUse dependencies",
            Self::Remove => "Remove a RustUse dependency",
            Self::Update => "Update RustUse dependencies",
            Self::Upgrade => "Upgrade a RustUse repository",
            Self::DevelopmentAndCi => "Development and CI tools",
            Self::Ferris => "Ask Ferris",
            Self::Exit => "Exit",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CatalogAction {
    Discover,
    Generate,
    Check,
    Info,
    Search,
    Back,
}

impl CatalogAction {
    const ALL: [Self; 6] = [
        Self::Discover,
        Self::Generate,
        Self::Check,
        Self::Info,
        Self::Search,
        Self::Back,
    ];

    const fn label(self) -> &'static str {
        match self {
            Self::Discover => "Discover local catalog entries",
            Self::Generate => "Generate catalog artifacts",
            Self::Check => "Check catalog consistency",
            Self::Info => "Show a catalog entry",
            Self::Search => "Search catalog entries",
            Self::Back => "Back",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DocsKind {
    Website,
    Api,
    Workspace,
}

impl DocsKind {
    const ALL: [Self; 3] = [Self::Website, Self::Api, Self::Workspace];

    const fn label(self) -> &'static str {
        match self {
            Self::Website => "RustUse documentation",
            Self::Api => "API documentation",
            Self::Workspace => "Workspace documentation",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ToolAction {
    Development,
    Ci,
    Back,
}

impl ToolAction {
    const ALL: [Self; 3] = [Self::Development, Self::Ci, Self::Back];

    const fn label(self) -> &'static str {
        match self {
            Self::Development => "RustUse development tools",
            Self::Ci => "Run CI validation",
            Self::Back => "Back",
        }
    }
}

pub(crate) fn run(output: Output, non_interactive: bool, yes: bool) -> Result<()> {
    validate_interactive_mode(
        non_interactive,
        yes,
        output.is_json(),
        std::io::stdin().is_terminal(),
    )?;

    let action = prompt_menu_action()?;
    run_action(action, output)
}

fn validate_interactive_mode(
    non_interactive: bool,
    yes: bool,
    json: bool,
    stdin_is_terminal: bool,
) -> Result<()> {
    if non_interactive {
        bail!("a command is required when running non-interactively; try `rustuse --help`");
    }

    if yes {
        bail!("a command is required when using `--yes`; try `rustuse --help`");
    }

    if json {
        bail!("a command is required when using `--json`; try `rustuse --help`");
    }

    if !stdin_is_terminal {
        bail!("a command is required when running non-interactively; try `rustuse --help`");
    }

    Ok(())
}

fn prompt_menu_action() -> Result<MenuAction> {
    let actions = MenuAction::ALL;
    let labels = actions.map(MenuAction::label);
    let selected = prompt_select("What do you want to do?", &labels, 0)?;

    Ok(actions[selected])
}

fn run_action(action: MenuAction, output: Output) -> Result<()> {
    match action {
        MenuAction::Search => run_search(output),
        MenuAction::Info => run_info(output),
        MenuAction::Add => run_add(output),
        MenuAction::Catalog => run_catalog(output),
        MenuAction::Check => run_check(output),
        MenuAction::Diff => run_diff(output),
        MenuAction::Docs => run_docs(output),
        MenuAction::Doctor => run_doctor(output),
        MenuAction::Init => run_init(output),
        MenuAction::List => run_list(output),
        MenuAction::Outdated => run_outdated(output),
        MenuAction::Remove => run_remove(output),
        MenuAction::Update => run_update(output),
        MenuAction::Upgrade => run_upgrade(output),
        MenuAction::DevelopmentAndCi => run_development_and_ci(output),
        MenuAction::Ferris => ferris::run(FerrisArgs {}, output),
        MenuAction::Exit => Ok(()),
    }
}

fn run_search(output: Output) -> Result<()> {
    search::run(
        SearchArgs {
            query: prompt_text("Search query")?,
            limit: prompt_limit("Maximum results", 20)?,
        },
        output,
    )
}

fn run_info(output: Output) -> Result<()> {
    info::run(
        InfoArgs {
            name: NamedCommandArgs {
                name: prompt_text("Crate or primitive name")?,
            },
        },
        output,
    )
}

fn run_add(output: Output) -> Result<()> {
    add::run(
        AddArgs {
            name: NamedCommandArgs {
                name: prompt_text("Crate name")?,
            },
            mode: AddMode::Cargo,
            dry_run: prompt_confirm("Preview without writing changes?", true)?,
        },
        output,
    )
}

fn run_catalog(output: Output) -> Result<()> {
    let actions = CatalogAction::ALL;
    let labels = actions.map(CatalogAction::label);
    let selected = prompt_select("What catalog task do you want to run?", &labels, 0)?;

    let command = match actions[selected] {
        CatalogAction::Discover => CatalogCommand::Discover(CatalogPathArgs {
            path: prompt_path("RustUse root path", ".")?,
        }),
        CatalogAction::Generate => CatalogCommand::Generate(CatalogGenerateArgs {
            path: prompt_path("RustUse root path", ".")?,
            output: prompt_optional_path(
                "Output file or directory; leave blank to print the generated catalog",
            )?,
        }),
        CatalogAction::Check => CatalogCommand::Check(CatalogPathArgs {
            path: prompt_path("RustUse root path", ".")?,
        }),
        CatalogAction::Info => CatalogCommand::Info(CatalogInfoArgs {
            name: prompt_text("Catalog entry name")?,
        }),
        CatalogAction::Search => CatalogCommand::Search(CatalogSearchArgs {
            query: prompt_text("Catalog search query")?,
            limit: prompt_limit("Maximum results", 20)?,
        }),
        CatalogAction::Back => return Ok(()),
    };

    catalog::run(CatalogArgs { command }, output)
}

fn run_check(output: Output) -> Result<()> {
    check::run(
        CheckArgs {
            path: prompt_path("Repository to check", ".")?,
        },
        output,
    )
}

fn run_diff(output: Output) -> Result<()> {
    diff::run(
        DiffArgs {
            path: prompt_path("Repository to diff", ".")?,
        },
        output,
    )
}

fn run_docs(output: Output) -> Result<()> {
    let kinds = DocsKind::ALL;
    let labels = kinds.map(DocsKind::label);
    let selected = prompt_select("Which documentation do you want?", &labels, 0)?;
    let kind = kinds[selected];

    let name = match kind {
        DocsKind::Website => {
            prompt_optional_text("Crate, primitive, or facade name; leave blank for rustuse.org")?
        },
        DocsKind::Api | DocsKind::Workspace => {
            Some(prompt_text("Crate, primitive, or facade name")?)
        },
    };

    let (api, workspace) = match kind {
        DocsKind::Website => (false, false),
        DocsKind::Api => (true, false),
        DocsKind::Workspace => (false, true),
    };

    docs::run(
        DocsArgs {
            name,
            api,
            workspace,
        },
        output,
    )
}

fn run_doctor(output: Output) -> Result<()> {
    doctor::run(
        DoctorArgs {
            path: prompt_path("Directory to inspect", ".")?,
        },
        output,
    )
}

fn run_init(output: Output) -> Result<()> {
    let path = prompt_path("Directory to initialize", ".")?;
    let cargo_first = prompt_confirm("Prefer Cargo-mode defaults?", true)?;
    let dry_run = prompt_confirm("Preview without writing files?", true)?;
    let force = !dry_run && prompt_confirm("Overwrite an existing rustuse.toml?", false)?;

    init::run(
        InitArgs {
            path,
            cargo_first,
            dry_run,
            force,
        },
        output,
    )
}

fn run_list(output: Output) -> Result<()> {
    list::run(
        ListArgs {
            all: prompt_confirm("Show all catalog entries?", false)?,
            path: prompt_path("Project directory", ".")?,
        },
        output,
    )
}

fn run_outdated(output: Output) -> Result<()> {
    outdated::run(
        OutdatedArgs {
            path: prompt_path("Repository to inspect", ".")?,
        },
        output,
    )
}

fn run_remove(output: Output) -> Result<()> {
    let path = prompt_path("Repository to remove from", ".")?;

    if !prompt_confirm("Continue with removal?", false)? {
        return Ok(());
    }

    remove::run(RemoveArgs { path }, output)
}

fn run_update(output: Output) -> Result<()> {
    update::run(
        UpdateArgs {
            path: prompt_path("Repository to update", ".")?,
        },
        output,
    )
}

fn run_upgrade(output: Output) -> Result<()> {
    upgrade::run(
        UpgradeArgs {
            dry_run: prompt_confirm("Preview the upgrade command?", false)?,
        },
        output,
    )
}

fn run_development_and_ci(output: Output) -> Result<()> {
    let actions = ToolAction::ALL;
    let labels = actions.map(ToolAction::label);
    let selected = prompt_select(
        "Which development or CI workflow do you want to run?",
        &labels,
        0,
    )?;

    match actions[selected] {
        ToolAction::Development => dev::run(DevArgs { command: None }, output, false, false),
        ToolAction::Ci => run_ci(output),
        ToolAction::Back => Ok(()),
    }
}

fn run_ci(output: Output) -> Result<()> {
    ci::run(
        CiArgs {
            command: CiCommand::Check(CheckCiArgs {
                path: prompt_path("Path to validate", ".")?,
                kind: CheckKind::Auto,
                deny_warnings: prompt_confirm("Fail when warnings are found?", false)?,
            }),
        },
        output,
    )
}

fn prompt_text(prompt: &str) -> Result<String> {
    let value = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .validate_with(|value: &String| {
            if value.trim().is_empty() {
                Err("a value is required")
            } else {
                Ok(())
            }
        })
        .interact_text()?;

    Ok(value.trim().to_owned())
}

fn prompt_optional_text(prompt: &str) -> Result<Option<String>> {
    let value = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .allow_empty(true)
        .interact_text()?;

    let value = value.trim();

    if value.is_empty() {
        Ok(None)
    } else {
        Ok(Some(value.to_owned()))
    }
}

fn prompt_path(prompt: &str, default: &str) -> Result<PathBuf> {
    let value = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(default.to_owned())
        .interact_text()?;

    Ok(PathBuf::from(value))
}

fn prompt_optional_path(prompt: &str) -> Result<Option<PathBuf>> {
    Ok(prompt_optional_text(prompt)?.map(PathBuf::from))
}

fn prompt_limit(prompt: &str, default: usize) -> Result<usize> {
    let value = Input::<usize>::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(default)
        .validate_with(|value: &usize| {
            if *value == 0 {
                Err("the limit must be greater than zero")
            } else {
                Ok(())
            }
        })
        .interact_text()?;

    Ok(value)
}

fn prompt_confirm(prompt: &str, default: bool) -> Result<bool> {
    Ok(Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(default)
        .interact()?)
}

fn prompt_select(prompt: &str, items: &[&str], default: usize) -> Result<usize> {
    Ok(Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(items)
        .default(default)
        .interact()?)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::{CatalogAction, DocsKind, MenuAction, ToolAction, validate_interactive_mode};

    #[test]
    fn menu_labels_are_non_empty_and_unique() {
        let labels = MenuAction::ALL.map(MenuAction::label);
        assert_labels_are_valid(&labels);
    }

    #[test]
    fn catalog_labels_are_non_empty_and_unique() {
        let labels = CatalogAction::ALL.map(CatalogAction::label);
        assert_labels_are_valid(&labels);
    }

    #[test]
    fn docs_labels_are_non_empty_and_unique() {
        let labels = DocsKind::ALL.map(DocsKind::label);
        assert_labels_are_valid(&labels);
    }

    #[test]
    fn tool_labels_are_non_empty_and_unique() {
        let labels = ToolAction::ALL.map(ToolAction::label);
        assert_labels_are_valid(&labels);
    }

    #[test]
    fn navigation_actions_are_last() {
        assert_eq!(MenuAction::ALL.last().copied(), Some(MenuAction::Exit));
        assert_eq!(
            CatalogAction::ALL.last().copied(),
            Some(CatalogAction::Back)
        );
        assert_eq!(ToolAction::ALL.last().copied(), Some(ToolAction::Back));
    }

    #[test]
    fn interactive_mode_accepts_a_terminal_session() {
        assert!(validate_interactive_mode(false, false, false, true).is_ok());
    }

    #[test]
    fn interactive_mode_rejects_non_interactive_execution() {
        assert!(validate_interactive_mode(true, false, false, true).is_err());
    }

    #[test]
    fn interactive_mode_rejects_yes_without_a_command() {
        assert!(validate_interactive_mode(false, true, false, true).is_err());
    }

    #[test]
    fn interactive_mode_rejects_json_without_a_command() {
        assert!(validate_interactive_mode(false, false, true, true).is_err());
    }

    #[test]
    fn interactive_mode_rejects_non_terminal_input() {
        assert!(validate_interactive_mode(false, false, false, false).is_err());
    }

    fn assert_labels_are_valid(labels: &[&str]) {
        assert!(labels.iter().all(|label| !label.trim().is_empty()));

        let unique: HashSet<_> = labels.iter().copied().collect();
        assert_eq!(unique.len(), labels.len());
    }
}
