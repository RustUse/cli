use core::fmt;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

pub const CONFIG_VERSION: u32 = 1;
pub const DEFAULT_LICENSE: &str = "MIT OR Apache-2.0";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RustUseConfig {
    pub version: u32,
    pub project: ProjectConfig,
    pub updates: UpdatesConfig,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub primitives: Vec<PrimitiveConfig>,
}

impl RustUseConfig {
    #[must_use]
    pub fn cargo_first() -> Self {
        Self {
            version: CONFIG_VERSION,
            project: ProjectConfig::cargo_first(),
            updates: UpdatesConfig::default(),
            primitives: Vec::new(),
        }
    }

    #[must_use]
    pub fn copy_first() -> Self {
        Self {
            version: CONFIG_VERSION,
            project: ProjectConfig::copy_first(),
            updates: UpdatesConfig::default(),
            primitives: Vec::new(),
        }
    }

    #[must_use]
    pub fn with_roots(mut self, copy_root: Option<String>, test_root: Option<String>) -> Self {
        if let Some(copy_root) = copy_root {
            self.project.copy_root = copy_root;
        }
        if let Some(test_root) = test_root {
            self.project.test_root = test_root;
        }
        self
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProjectConfig {
    pub default_mode: AdoptionMode,
    pub copy_root: String,
    pub test_root: String,
    pub license: String,
}

impl ProjectConfig {
    #[must_use]
    pub fn cargo_first() -> Self {
        Self {
            default_mode: AdoptionMode::Cargo,
            copy_root: String::from("src"),
            test_root: String::from("tests"),
            license: String::from(DEFAULT_LICENSE),
        }
    }

    #[must_use]
    pub fn copy_first() -> Self {
        Self {
            default_mode: AdoptionMode::Copy,
            copy_root: String::from("src/rustuse"),
            test_root: String::from("tests/rustuse"),
            license: String::from(DEFAULT_LICENSE),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdatesConfig {
    pub check_copied: bool,
    pub write_candidates: bool,
}

impl Default for UpdatesConfig {
    fn default() -> Self {
        Self {
            check_copied: true,
            write_candidates: true,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum AdoptionMode {
    Cargo,
    Copy,
}

impl AdoptionMode {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cargo => "cargo",
            Self::Copy => "copy",
        }
    }
}

impl fmt::Display for AdoptionMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrimitiveConfig {
    pub name: String,
    pub mode: AdoptionMode,
    pub version: Option<String>,
    pub target: Option<String>,
    pub with_tests: bool,
    pub with_examples: bool,
}

pub fn to_toml(config: &RustUseConfig) -> Result<String> {
    let mut raw = toml::to_string_pretty(config).context("failed to serialize rustuse.toml")?;
    if !raw.ends_with('\n') {
        raw.push('\n');
    }
    Ok(raw)
}

pub fn from_toml(raw: &str) -> Result<RustUseConfig> {
    toml::from_str(raw).context("failed to parse rustuse.toml")
}
