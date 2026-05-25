use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output as CommandOutput};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Copy, Debug)]
enum CliBinary {
    Rustuse,
    CargoRustuse,
}

impl CliBinary {
    const fn name(self) -> &'static str {
        match self {
            Self::Rustuse => "rustuse",
            Self::CargoRustuse => "cargo-rustuse",
        }
    }

    fn executable(self) -> &'static str {
        match self {
            Self::Rustuse => env!("CARGO_BIN_EXE_rustuse"),
            Self::CargoRustuse => env!("CARGO_BIN_EXE_cargo-rustuse"),
        }
    }
}

fn run_help(binary: CliBinary, args: &[&str]) -> String {
    let output = Command::new(binary.executable())
        .args(args)
        .output()
        .unwrap_or_else(|error| panic!("failed to run {} help command: {error}", binary.name()));

    assert!(
        output.status.success(),
        "{} {:?} failed\nstdout:\n{}\nstderr:\n{}",
        binary.name(),
        args,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8(output.stdout)
        .unwrap_or_else(|error| panic!("{} help output should be UTF-8: {error}", binary.name()))
}

fn run_raw(binary: CliBinary, args: &[&str], current_dir: &Path) -> CommandOutput {
    Command::new(binary.executable())
        .args(args)
        .current_dir(current_dir)
        .output()
        .unwrap_or_else(|error| panic!("failed to run {} command: {error}", binary.name()))
}

fn run_success(binary: CliBinary, args: &[&str], current_dir: &Path) -> String {
    let output = run_raw(binary, args, current_dir);

    assert!(
        output.status.success(),
        "{} {:?} failed\nstdout:\n{}\nstderr:\n{}",
        binary.name(),
        args,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8(output.stdout)
        .unwrap_or_else(|error| panic!("{} stdout should be UTF-8: {error}", binary.name()))
}

struct TempProject {
    root: PathBuf,
}

impl TempProject {
    fn new(label: &str) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after Unix epoch")
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "rustuse-cli-{label}-{}-{timestamp}",
            std::process::id()
        ));
        fs::create_dir_all(&root).expect("failed to create temporary project");

        Self { root }
    }

    fn path(&self) -> &Path {
        &self.root
    }
}

impl Drop for TempProject {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[test]
fn root_help_works() {
    let output = run_help(CliBinary::Rustuse, &["--help"]);

    assert_root_help_output(&output);
}

#[test]
fn cargo_rustuse_root_help_works() {
    let output = run_help(CliBinary::CargoRustuse, &["--help"]);

    assert_root_help_output(&output);
}

fn assert_root_help_output(output: &str) {
    assert!(output.contains("rustuse helps find, inspect, and plan RustUse adoption"));
    assert!(output.contains("search"));
    assert!(output.contains("--json"));
}

#[test]
fn add_help_works() {
    let output = run_help(CliBinary::Rustuse, &["add", "--help"]);

    assert!(output.contains("--copy"));
    assert!(output.contains("--with-tests"));
}

#[test]
fn init_help_works() {
    let output = run_help(CliBinary::Rustuse, &["init", "--help"]);

    assert!(output.contains("rustuse.toml"));
    assert!(output.contains("--copy-first"));
    assert!(output.contains("--dry-run"));
}

#[test]
fn copy_help_works() {
    let output = run_help(CliBinary::Rustuse, &["copy", "--help"]);

    assert!(output.contains("--with-tests"));
    assert!(output.contains("--track"));
}

#[test]
fn doctor_help_works() {
    let output = run_help(CliBinary::Rustuse, &["doctor", "--help"]);

    assert!(output.contains("Check this directory"));
}

#[test]
fn rustuse_search_geometry_works() {
    let project = TempProject::new("rustuse-search-geometry");
    let output = run_success(CliBinary::Rustuse, &["search", "geometry"], project.path());

    assert!(output.contains("use-geometry"));
    assert!(output.contains("Found 1 RustUse entry"));
}

#[test]
fn cargo_rustuse_search_geometry_works() {
    let project = TempProject::new("cargo-rustuse-search-geometry");
    let output = run_success(
        CliBinary::CargoRustuse,
        &["search", "geometry"],
        project.path(),
    );

    assert!(output.contains("use-geometry"));
    assert!(output.contains("Found 1 RustUse entry"));
}

#[test]
fn cargo_dispatched_rustuse_search_geometry_works() {
    let project = TempProject::new("cargo-dispatched-rustuse-search-geometry");
    let output = run_success(
        CliBinary::CargoRustuse,
        &["rustuse", "search", "geometry"],
        project.path(),
    );

    assert!(output.contains("use-geometry"));
    assert!(output.contains("Found 1 RustUse entry"));
}

#[test]
fn init_dry_run_does_not_create_files() {
    let project = TempProject::new("init-dry-run");
    let output = run_success(CliBinary::Rustuse, &["init", "--dry-run"], project.path());

    assert_init_dry_run_output(&output, project.path());
}

#[test]
fn cargo_rustuse_init_dry_run_does_not_create_files() {
    let project = TempProject::new("cargo-rustuse-init-dry-run");
    let output = run_success(
        CliBinary::CargoRustuse,
        &["init", "--dry-run"],
        project.path(),
    );

    assert_init_dry_run_output(&output, project.path());
}

#[test]
fn cargo_dispatched_rustuse_init_dry_run_does_not_create_files() {
    let project = TempProject::new("cargo-dispatched-rustuse-init-dry-run");
    let output = run_success(
        CliBinary::CargoRustuse,
        &["rustuse", "init", "--dry-run"],
        project.path(),
    );

    assert_init_dry_run_output(&output, project.path());
}

fn assert_init_dry_run_output(output: &str, project_path: &Path) {
    assert!(output.contains("Would initialize RustUse project tracking"));
    assert!(output.contains("No changes made because --dry-run was used"));
    assert!(!project_path.join("rustuse.toml").exists());
    assert!(!project_path.join(".rustuse").exists());
}

#[test]
fn init_creates_config_and_tracking_dirs() {
    let project = TempProject::new("init-create");
    let output = run_success(CliBinary::Rustuse, &["init"], project.path());

    assert!(output.contains("Initialized RustUse project tracking"));
    assert!(project.path().join("rustuse.toml").is_file());
    assert!(project.path().join(".rustuse/cache").is_dir());
    assert!(project.path().join(".rustuse/snapshots").is_dir());
    assert!(!project.path().join("rustuse.lock").exists());

    let raw_config = fs::read_to_string(project.path().join("rustuse.toml"))
        .expect("failed to read generated rustuse.toml");
    assert!(raw_config.contains("default_mode = \"cargo\""));
}

#[test]
fn init_is_idempotent_and_does_not_overwrite() {
    let project = TempProject::new("init-idempotent");
    run_success(CliBinary::Rustuse, &["init"], project.path());
    let config_path = project.path().join("rustuse.toml");
    fs::write(&config_path, "sentinel = true\n").expect("failed to rewrite rustuse.toml");

    let output = run_success(CliBinary::Rustuse, &["init"], project.path());

    assert!(output.contains("rustuse.toml already exists"));
    assert_eq!(
        fs::read_to_string(config_path).expect("failed to read rustuse.toml"),
        "sentinel = true\n"
    );
}

#[test]
fn init_copy_first_dry_run_reports_copy_default() {
    let project = TempProject::new("init-copy-first");
    let output = run_success(
        CliBinary::Rustuse,
        &["init", "--copy-first", "--dry-run"],
        project.path(),
    );

    assert!(output.contains("Default mode:"));
    assert!(output.contains("  copy"));
    assert!(!project.path().join("rustuse.toml").exists());
}

#[test]
fn init_rejects_conflicting_modes() {
    let project = TempProject::new("init-conflict");
    let output = run_raw(
        CliBinary::Rustuse,
        &["init", "--copy-first", "--cargo-first"],
        project.path(),
    );

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr)
        .contains("--copy-first and --cargo-first cannot be used together"));
}

#[test]
fn copy_without_tracking_does_not_create_config() {
    let project = TempProject::new("copy-no-config");
    let output = run_success(CliBinary::Rustuse, &["copy", "use-slug"], project.path());

    assert!(output.contains("No RustUse project state is required"));
    assert!(!project.path().join("rustuse.toml").exists());
}

#[test]
fn cargo_rustuse_copy_use_slug_works() {
    let project = TempProject::new("cargo-rustuse-copy-use-slug");
    let output = run_success(
        CliBinary::CargoRustuse,
        &["copy", "use-slug"],
        project.path(),
    );

    assert!(output.contains("Would copy use-slug source into this project"));
    assert!(output.contains("No RustUse project state is required"));
    assert!(!project.path().join("rustuse.toml").exists());
}

#[test]
fn add_copy_without_config_suggests_init() {
    let project = TempProject::new("add-copy-no-config");
    let output = run_success(
        CliBinary::Rustuse,
        &["add", "use-slug", "--copy"],
        project.path(),
    );

    assert!(output.contains("rustuse init"));
    assert!(!project.path().join("rustuse.toml").exists());
}

#[test]
fn cargo_rustuse_add_use_geometry_works() {
    let project = TempProject::new("cargo-rustuse-add-use-geometry");
    let output = run_success(
        CliBinary::CargoRustuse,
        &["add", "use-geometry"],
        project.path(),
    );

    assert!(output.contains("Would add use-geometry as a Cargo dependency"));
    assert!(!project.path().join("rustuse.toml").exists());
}
