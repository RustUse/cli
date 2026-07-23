//! Shared helpers for `rustuse` / `cargo-rustuse` integration tests.
//!
//! These helpers wrap the compiled CLI binaries and provide throwaway
//! temporary project fixtures. Not every test binary uses every helper, so
//! dead-code is allowed within this shared module only.
#![allow(dead_code)]
#![allow(clippy::expect_used)]

use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// A CLI binary under test (`rustuse` or `cargo-rustuse`).
pub struct CliBinary {
    path: PathBuf,
}

impl CliBinary {
    /// The `rustuse` binary built by Cargo for this test run.
    pub fn rustuse() -> Self {
        Self {
            path: PathBuf::from(env!("CARGO_BIN_EXE_rustuse")),
        }
    }

    /// The `cargo-rustuse` binary built by Cargo for this test run.
    pub fn cargo_rustuse() -> Self {
        Self {
            path: PathBuf::from(env!("CARGO_BIN_EXE_cargo-rustuse")),
        }
    }

    /// A fresh [`Command`] for this binary.
    pub fn command(&self) -> Command {
        Command::new(&self.path)
    }
}

/// Runs the binary with `args` and returns the raw process output.
pub fn run_raw(bin: &CliBinary, args: &[&str]) -> Output {
    bin.command()
        .args(args)
        .output()
        .expect("failed to run CLI binary")
}

/// Runs the binary with `args` from `dir` and returns the raw process output.
pub fn run_raw_in(bin: &CliBinary, dir: &Path, args: &[&str]) -> Output {
    bin.command()
        .current_dir(dir)
        .args(args)
        .output()
        .expect("failed to run CLI binary")
}

/// Runs the binary, asserts success, and returns stdout as a `String`.
pub fn run_success(bin: &CliBinary, args: &[&str]) -> String {
    let output = run_raw(bin, args);

    assert!(
        output.status.success(),
        "expected success for args {args:?}, got {:?}\nstderr: {}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8_lossy(&output.stdout).into_owned()
}

/// Runs `<args> --help`, asserts success, and returns stdout as a `String`.
pub fn run_help(bin: &CliBinary, args: &[&str]) -> String {
    let mut full = args.to_vec();
    full.push("--help");

    let output = run_raw(bin, &full);

    assert!(
        output.status.success(),
        "expected `--help` success for args {args:?}, got {:?}\nstderr: {}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8_lossy(&output.stdout).into_owned()
}

/// A throwaway temporary directory used as a CLI working tree.
///
/// The directory (and everything inside it) is removed on drop.
pub struct TempProject {
    root: PathBuf,
}

impl TempProject {
    /// Creates an empty temporary project directory.
    pub fn new() -> Self {
        static COUNTER: AtomicU32 = AtomicU32::new(0);

        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();

        let unique = format!(
            "rustuse_it_{}_{}_{}",
            std::process::id(),
            COUNTER.fetch_add(1, Ordering::Relaxed),
            nanos
        );

        let root = std::env::temp_dir().join(unique);
        std::fs::create_dir_all(&root).expect("failed to create temp project dir");

        Self { root }
    }

    /// Creates a temporary facade repository fixture (root is the facade).
    ///
    /// The manifest package name uses the `use-` prefix so facade detection
    /// works regardless of the randomized directory name.
    pub fn facade(name: &str) -> Self {
        let project = Self::new();
        project.create_dir("crates");
        project.write(
            "Cargo.toml",
            &format!("[package]\nname = \"use-{name}\"\nversion = \"0.1.0\"\n"),
        );
        project
    }

    /// Creates a temporary `RustUse` root containing one `use-<name>` facade.
    pub fn root_with_facade(name: &str) -> Self {
        let project = Self::new();
        let facade = format!("use-{name}");
        project.create_dir(&format!("{facade}/crates"));
        project.write(
            &format!("{facade}/Cargo.toml"),
            &format!("[package]\nname = \"{facade}\"\nversion = \"0.1.0\"\n"),
        );
        project
    }

    /// The absolute path of this temporary project.
    pub fn path(&self) -> &Path {
        &self.root
    }

    /// Writes `contents` to `relative` inside the project, creating parents.
    pub fn write(&self, relative: &str, contents: &str) {
        let path = self.root.join(relative);

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("failed to create parent dir");
        }

        std::fs::write(path, contents).expect("failed to write fixture file");
    }

    /// Creates a directory (and parents) at `relative` inside the project.
    pub fn create_dir(&self, relative: &str) {
        std::fs::create_dir_all(self.root.join(relative)).expect("failed to create fixture dir");
    }
}

impl Default for TempProject {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for TempProject {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}
