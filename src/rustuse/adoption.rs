use std::path::Path;

use anyhow::{Result, bail};

use super::{adapter::cargo::add::add_cargo_dependency, catalog};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum AdoptionMode {
    Cargo,
    Own,
}

impl AdoptionMode {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Cargo => "cargo",
            Self::Own => "own",
        }
    }
}

#[derive(Debug)]
pub(crate) struct AdoptionRequest<'a> {
    pub root: &'a Path,
    pub name: &'a str,
    pub mode: AdoptionMode,
    pub dry_run: bool,
}

#[derive(Debug)]
pub(crate) struct AdoptionResult {
    pub name: String,
    pub mode: AdoptionMode,
    pub dry_run: bool,
}

impl AdoptionResult {
    pub(crate) const fn status(&self) -> &'static str {
        if self.dry_run {
            return "dry-run";
        }

        match self.mode {
            AdoptionMode::Cargo => "added",
            AdoptionMode::Own => "owned",
        }
    }
}

pub(crate) fn adopt(request: AdoptionRequest<'_>) -> Result<AdoptionResult> {
    let Some(entry) = catalog::find_by_name(request.name) else {
        bail!("No RustUse entry found for `{}`.", request.name);
    };

    let name = entry.name.to_string();

    match request.mode {
        AdoptionMode::Cargo => {
            ensure_mode_supported(entry, AdoptionMode::Cargo)?;
            add_cargo_dependency(request.root, &name, request.dry_run)?;
        },
        AdoptionMode::Own => {
            adopt_owned_source(request.root, &name, request.dry_run)?;
        },
    }

    Ok(AdoptionResult {
        name,
        mode: request.mode,
        dry_run: request.dry_run,
    })
}

fn ensure_mode_supported(entry: &catalog::model::CatalogEntry, mode: AdoptionMode) -> Result<()> {
    let supported = entry
        .modes
        .iter()
        .any(|entry_mode| entry_mode.as_str() == mode.as_str());

    if !supported {
        bail!(
            "RustUse entry `{}` does not support `{}` adoption.",
            entry.name,
            mode.as_str()
        );
    }

    Ok(())
}

fn adopt_owned_source(_root: &Path, name: &str, _dry_run: bool) -> Result<()> {
    bail!(
        "Owned-source adoption is not implemented yet for `{name}`. \
         Use the default Cargo adoption mode for now."
    );
}
