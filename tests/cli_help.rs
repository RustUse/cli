use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output as CommandOutput};
use std::time::{SystemTime, UNIX_EPOCH};

fn run_help(args: &[&str]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_rustuse"))
        .args(args)
        .output()
        .expect("failed to run rustuse help command");

    assert!(
        output.status.success(),
        "rustuse {:?} failed\nstdout:\n{}\nstderr:\n{}",
        args,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8(output.stdout).expect("rustuse help output should be UTF-8")
}

fn run_raw(args: &[&str], current_dir: &Path) -> CommandOutput {
    Command::new(env!("CARGO_BIN_EXE_rustuse"))
        .args(args)
        .current_dir(current_dir)
        .output()
        .expect("failed to run rustuse command")
}

fn run_success(args: &[&str], current_dir: &Path) -> String {
    let output = run_raw(args, current_dir);

    assert!(
        output.status.success(),
        "rustuse {:?} failed\nstdout:\n{}\nstderr:\n{}",
        args,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8(output.stdout).expect("rustuse stdout should be UTF-8")
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
    let output = run_help(&["--help"]);

    assert!(output.contains("rustuse helps find, inspect, and plan RustUse adoption"));
    assert!(output.contains("search"));
    assert!(output.contains("--json"));
}

#[test]
fn add_help_works() {
    let output = run_help(&["add", "--help"]);

    assert!(output.contains("--copy"));
    assert!(output.contains("--with-tests"));
}

#[test]
fn init_help_works() {
    let output = run_help(&["init", "--help"]);

    assert!(output.contains("rustuse.toml"));
    assert!(output.contains("--copy-first"));
    assert!(output.contains("--dry-run"));
}

#[test]
fn copy_help_works() {
    let output = run_help(&["copy", "--help"]);

    assert!(output.contains("--with-tests"));
    assert!(output.contains("--track"));
}

#[test]
fn doctor_help_works() {
    let output = run_help(&["doctor", "--help"]);

    assert!(output.contains("Check this directory"));
}

#[test]
fn init_dry_run_does_not_create_files() {
    let project = TempProject::new("init-dry-run");
    let output = run_success(&["init", "--dry-run"], project.path());

    assert!(output.contains("Would initialize RustUse project tracking"));
    assert!(output.contains("No changes made because --dry-run was used"));
    assert!(!project.path().join("rustuse.toml").exists());
    assert!(!project.path().join(".rustuse").exists());
}

#[test]
fn init_creates_config_and_tracking_dirs() {
    let project = TempProject::new("init-create");
    let output = run_success(&["init"], project.path());

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
    run_success(&["init"], project.path());
    let config_path = project.path().join("rustuse.toml");
    fs::write(&config_path, "sentinel = true\n").expect("failed to rewrite rustuse.toml");

    let output = run_success(&["init"], project.path());

    assert!(output.contains("rustuse.toml already exists"));
    assert_eq!(
        fs::read_to_string(config_path).expect("failed to read rustuse.toml"),
        "sentinel = true\n"
    );
}

#[test]
fn init_copy_first_dry_run_reports_copy_default() {
    let project = TempProject::new("init-copy-first");
    let output = run_success(&["init", "--copy-first", "--dry-run"], project.path());

    assert!(output.contains("Default mode:"));
    assert!(output.contains("  copy"));
    assert!(!project.path().join("rustuse.toml").exists());
}

#[test]
fn init_rejects_conflicting_modes() {
    let project = TempProject::new("init-conflict");
    let output = run_raw(&["init", "--copy-first", "--cargo-first"], project.path());

    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains("--copy-first and --cargo-first cannot be used together")
    );
}

#[test]
fn copy_without_tracking_does_not_create_config() {
    let project = TempProject::new("copy-no-config");
    let output = run_success(&["copy", "use-slug"], project.path());

    assert!(output.contains("No RustUse project state is required"));
    assert!(!project.path().join("rustuse.toml").exists());
}

#[test]
fn add_copy_without_config_suggests_init() {
    let project = TempProject::new("add-copy-no-config");
    let output = run_success(&["add", "use-slug", "--copy"], project.path());

    assert!(output.contains("rustuse init"));
    assert!(!project.path().join("rustuse.toml").exists());
}
