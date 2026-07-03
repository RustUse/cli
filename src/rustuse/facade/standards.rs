use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use super::discover::FacadeEntry;

pub(crate) const EXACT_STANDARD_FILES: &[&str] = &[
    ".clippy.toml",
    ".editorconfig",
    ".gitattributes",
    ".gitignore",
    ".gitlab-ci.yml",
    ".gitleaks.toml",
    ".markdownlintignore",
    ".rustfmt.toml",
    ".taplo.toml",
    ".trivyignore",
    "Cargo.lock",
    "Cargo.toml",
    "CRATE_TEMPLATE.md",
    "deny.toml",
    "LICENSE-APACHE",
    "LICENSE-MIT",
    "Makefile",
    "README.md",
    "release-plz.toml",
    "rust-toolchain.toml",
];

#[derive(Debug)]
pub(crate) struct StandardFileReport {
    pub(crate) file_name: &'static str,
    pub(crate) present_count: usize,
    pub(crate) missing: Vec<String>,
    pub(crate) variants: Vec<StandardFileVariant>,
}

impl StandardFileReport {
    pub(crate) fn is_consistent(&self, expected_count: usize) -> bool {
        self.present_count == expected_count && self.variants.len() == 1
    }
}

#[derive(Debug)]
pub(crate) struct StandardFileVariant {
    pub(crate) hash: String,
    pub(crate) byte_len: usize,
    pub(crate) line_count: usize,
    pub(crate) facades: Vec<String>,
}

pub(crate) fn analyze_exact_standard_files(
    root: &Path,
    facades: &[FacadeEntry],
) -> Result<Vec<StandardFileReport>> {
    EXACT_STANDARD_FILES
        .iter()
        .map(|file_name| analyze_standard_file(root, facades, file_name))
        .collect()
}

pub(crate) fn analyze_standard_file(
    root: &Path,
    facades: &[FacadeEntry],
    file_name: &'static str,
) -> Result<StandardFileReport> {
    let mut missing = Vec::new();
    let mut variants: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for facade in facades {
        let file_path = root.join(&facade.name).join(file_name);

        if !file_path.is_file() {
            missing.push(facade.name.clone());
            continue;
        }

        let content = read_normalized_text_file(&file_path)?;
        variants
            .entry(content)
            .or_default()
            .push(facade.name.clone());
    }

    missing.sort();

    let present_count: usize = variants.values().map(Vec::len).sum();

    let mut variants = variants
        .into_iter()
        .map(|(content, mut facades)| {
            facades.sort();

            StandardFileVariant {
                hash: stable_hash_hex(&content),
                byte_len: content.len(),
                line_count: content.lines().count(),
                facades,
            }
        })
        .collect::<Vec<_>>();

    variants.sort_by(|left, right| {
        right
            .facades
            .len()
            .cmp(&left.facades.len())
            .then_with(|| left.hash.cmp(&right.hash))
    });

    Ok(StandardFileReport {
        file_name,
        present_count,
        missing,
        variants,
    })
}

fn read_normalized_text_file(path: &Path) -> Result<String> {
    let raw =
        fs::read_to_string(path).with_context(|| format!("failed to read `{}`", path.display()))?;

    Ok(raw.replace("\r\n", "\n").replace('\r', "\n"))
}

fn stable_hash_hex(content: &str) -> String {
    let mut hash = 0xcbf29ce484222325u64;

    for byte in content.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }

    format!("{hash:016x}")
}
