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
fn dev_facade_run_reports_basic_facade_shape() {
    let project = TempProject::new("dev-facade-run");

    fs::write(project.path().join("Cargo.toml"), "[workspace]\n")
        .expect("failed to write Cargo.toml");
    fs::create_dir_all(project.path().join(".git")).expect("failed to create .git directory");
    fs::create_dir_all(project.path().join("crates/use-example"))
        .expect("failed to create crate directory");
    fs::write(
        project.path().join("crates/use-example/Cargo.toml"),
        "[package]\nname = \"use-example\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    )
    .expect("failed to write child Cargo.toml");

    let output = run_success(
        CliBinary::Rustuse,
        &["dev", "facade", "run", "."],
        project.path(),
    );

    assert!(output.contains("RustUse dev facade run"));
    assert!(output.contains("git: yes"));
    assert!(output.contains("Cargo.toml: yes"));
    assert!(output.contains("crates/: yes"));
    assert!(output.contains("crate manifests: 1"));
    assert!(output.contains("status: ok"));
}

#[test]
fn dev_root_scan_reports_basic_facade_inventory() {
    let project = TempProject::new("dev-root-scan");
    let facade_root = project.path().join("use-example");

    fs::create_dir_all(facade_root.join(".git")).expect("failed to create facade .git directory");
    fs::create_dir_all(facade_root.join("crates/use-example"))
        .expect("failed to create facade crate directory");

    fs::write(
        facade_root.join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/*\"]\n",
    )
    .expect("failed to write facade workspace Cargo.toml");

    fs::write(
        facade_root.join("crates/use-example/Cargo.toml"),
        "[package]\nname = \"use-example\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    )
    .expect("failed to write facade package Cargo.toml");

    let output = run_success(
        CliBinary::Rustuse,
        &["dev", "root", "scan", "."],
        project.path(),
    );

    assert!(output.contains("RustUse dev root scan - root:"));
    assert!(output.contains("found 1 use-* directories"));
    assert!(output.contains("facade repos with .git: 1"));
    assert!(output.contains("facades missing .git: 0"));
    assert!(output.contains("child crates detected: 1"));
    assert!(output.contains("use-example"));
    assert!(output.contains("0.1.0"));
    assert!(output.contains("status: ok"));
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
fn dev_help_works() {
    let output = run_help(CliBinary::Rustuse, &["dev", "--help"]);

    assert!(output.contains("check"));
    assert!(output.contains("facade"));
    assert!(output.contains("info"));
    assert!(output.contains("root"));
}

#[test]
fn dev_root_help_works() {
    let output = run_help(CliBinary::Rustuse, &["dev", "root", "--help"]);

    assert!(output.contains("inspect"));
    assert!(output.contains("manifests"));
    assert!(output.contains("report"));
    assert!(output.contains("scan"));
}

#[test]
fn dev_facade_help_works() {
    let output = run_help(CliBinary::Rustuse, &["dev", "facade", "--help"]);

    assert!(output.contains("run"));
    assert!(output.contains("report"));
}

#[test]
fn dev_facade_run_help_works() {
    let output = run_help(CliBinary::Rustuse, &["dev", "facade", "run", "--help"]);

    assert!(output.contains("Facade repository root"));
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
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains("--copy-first and --cargo-first cannot be used together")
    );
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

#[test]
fn dev_without_subcommand_shows_help() {
    let project = TempProject::new("dev-no-subcommand");
    let output = run_raw(CliBinary::Rustuse, &["dev"], project.path());

    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Usage:"));
    assert!(stderr.contains("Commands:"));
}

#[test]
fn dev_root_without_subcommand_shows_help() {
    let project = TempProject::new("dev-root-no-subcommand");
    let output = run_raw(CliBinary::Rustuse, &["dev", "root"], project.path());

    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Usage:"));
    assert!(stderr.contains("Commands:"));
}

#[test]
fn dev_facade_without_subcommand_shows_help() {
    let project = TempProject::new("dev-facade-no-subcommand");
    let output = run_raw(CliBinary::Rustuse, &["dev", "facade"], project.path());

    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Usage:"));
    assert!(stderr.contains("Commands:"));
}

#[test]
fn dev_root_report_stdout_includes_basic_sections() {
    let project = TempProject::new("dev-root-report-stdout");
    let facade_root = project.path().join("use-example");

    fs::create_dir_all(facade_root.join(".git")).expect("failed to create facade .git directory");
    fs::create_dir_all(facade_root.join("crates/use-example"))
        .expect("failed to create facade crate directory");

    fs::write(
        facade_root.join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/*\"]\nresolver = \"3\"\n",
    )
    .expect("failed to write facade workspace Cargo.toml");

    fs::write(
        facade_root.join("crates/use-example/Cargo.toml"),
        "[package]\nname = \"use-example\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    )
    .expect("failed to write facade package Cargo.toml");

    let output = run_success(
        CliBinary::Rustuse,
        &["dev", "root", "report", ".", "--stdout"],
        project.path(),
    );

    assert!(output.contains("RustUse dev root report - root:"));
    assert!(output.contains("# RustUse Development Root Report"));
    assert!(output.contains("## Summary"));
    assert!(output.contains("## Action Plan"));
    assert!(output.contains("## Cargo Manifest Health"));
    assert!(output.contains("## Facade Inventory"));
    assert!(output.contains("use-example"));
}

#[test]
fn dev_facade_report_help_works() {
    let output = run_help(CliBinary::Rustuse, &["dev", "facade", "report", "--help"]);

    assert!(output.contains("Facade repository root"));
    assert!(output.contains("--output"));
    assert!(output.contains("--stdout"));
}

#[test]
fn dev_facade_report_stdout_includes_basic_sections() {
    let project = TempProject::new("dev-facade-report-stdout");

    fs::write(project.path().join("Cargo.toml"), "[workspace]\n")
        .expect("failed to write Cargo.toml");
    fs::create_dir_all(project.path().join(".git")).expect("failed to create .git directory");
    fs::create_dir_all(project.path().join("crates/use-example"))
        .expect("failed to create crate directory");
    fs::write(
        project.path().join("crates/use-example/Cargo.toml"),
        "[package]\nname = \"use-example\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    )
    .expect("failed to write child Cargo.toml");

    let output = run_success(
        CliBinary::Rustuse,
        &["dev", "facade", "report", ".", "--stdout"],
        project.path(),
    );

    assert!(output.contains("RustUse dev facade report - root:"));
    assert!(output.contains("# RustUse Facade Report"));
    assert!(output.contains("## Summary"));
    assert!(output.contains("## Action Plan"));
    assert!(output.contains("## Facade Shape"));
    assert!(output.contains("- [Cargo Manifest Health](#cargo-manifest-health)"));
    assert!(output.contains("## Cargo Manifest Health"));
    assert!(output.contains("### Manifest Inventory"));
    assert!(output.contains("## Child Crates"));
    assert!(output.contains("use-example"));
    assert!(output.contains("crates/use-example/Cargo.toml"));
    assert!(output.contains("Status:"));
    assert!(output.contains("- Status: **warning**"));
    assert!(output.contains("- Clean up manifest warnings."));
}
#[test]
fn dev_facade_report_writes_default_report() {
    let project = TempProject::new("dev-facade-report-write");

    fs::write(project.path().join("Cargo.toml"), "[workspace]\n")
        .expect("failed to write Cargo.toml");
    fs::create_dir_all(project.path().join(".git")).expect("failed to create .git directory");
    fs::create_dir_all(project.path().join("crates/use-example"))
        .expect("failed to create crate directory");
    fs::write(
        project.path().join("crates/use-example/Cargo.toml"),
        "[package]\nname = \"use-example\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    )
    .expect("failed to write child Cargo.toml");

    let output = run_success(
        CliBinary::Rustuse,
        &["dev", "facade", "report", "."],
        project.path(),
    );

    let report_path = project.path().join("rustuse-report.md");
    let report = fs::read_to_string(&report_path).expect("failed to read generated report");

    assert!(output.contains("RustUse dev facade report - root:"));
    assert!(output.contains("wrote:"));
    assert!(report_path.is_file());
    assert!(report.contains("# RustUse Facade Report"));
    assert!(report.contains("## Summary"));
    assert!(report.contains("## Action Plan"));
    assert!(report.contains("## Facade Shape"));
    assert!(report.contains("- [Cargo Manifest Health](#cargo-manifest-health)"));
    assert!(report.contains("## Cargo Manifest Health"));
    assert!(report.contains("### Manifest Inventory"));
    assert!(report.contains("## Child Crates"));
    assert!(report.contains("use-example"));
    assert!(report.contains("crates/use-example/Cargo.toml"));
    assert!(report.contains("- Status: **warning**"));
    assert!(report.contains("- Clean up manifest warnings."));
}

#[test]
fn dev_facade_report_writes_custom_report_path() {
    let project = TempProject::new("dev-facade-report-custom-output");

    fs::write(project.path().join("Cargo.toml"), "[workspace]\n")
        .expect("failed to write Cargo.toml");
    fs::create_dir_all(project.path().join(".git")).expect("failed to create .git directory");
    fs::create_dir_all(project.path().join("crates/use-example"))
        .expect("failed to create crate directory");
    fs::write(
        project.path().join("crates/use-example/Cargo.toml"),
        "[package]\nname = \"use-example\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    )
    .expect("failed to write child Cargo.toml");

    let output = run_success(
        CliBinary::Rustuse,
        &[
            "dev",
            "facade",
            "report",
            ".",
            "--output",
            "reports/facade-report.md",
        ],
        project.path(),
    );

    let report_path = project.path().join("reports/facade-report.md");
    let report = fs::read_to_string(&report_path).expect("failed to read generated report");

    assert!(output.contains("wrote:"));
    assert!(report_path.is_file());
    assert!(report.contains("# RustUse Facade Report"));
    assert!(report.contains("- [Cargo Manifest Health](#cargo-manifest-health)"));
    assert!(report.contains("## Cargo Manifest Health"));
    assert!(report.contains("### Manifest Inventory"));
    assert!(report.contains("use-example"));
    assert!(report.contains("crates/use-example/Cargo.toml"));
}

#[test]
fn dev_facade_report_includes_manifest_issue_counts() {
    let project = TempProject::new("dev-facade-report-manifest-health");

    fs::write(project.path().join("Cargo.toml"), "[workspace]\n")
        .expect("failed to write Cargo.toml");
    fs::create_dir_all(project.path().join(".git")).expect("failed to create .git directory");
    fs::create_dir_all(project.path().join("crates/use-example"))
        .expect("failed to create crate directory");
    fs::write(
        project.path().join("crates/use-example/Cargo.toml"),
        "[package]\nname = \"use-example\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    )
    .expect("failed to write child Cargo.toml");

    let output = run_success(
        CliBinary::Rustuse,
        &["dev", "facade", "report", ".", "--stdout"],
        project.path(),
    );

    assert!(output.contains("## Cargo Manifest Health"));
    assert!(output.contains("- Status: **warning**"));
    assert!(output.contains("- Manifests inspected: `2`"));
    assert!(output.contains("- Issues: `"));
    assert!(output.contains("- Warnings: `"));
    assert!(output.contains("### Manifest Issues"));
    assert!(output.contains("missing-workspace-resolver"));
    assert!(output.contains("- Clean up manifest warnings."));
}

#[test]
fn dev_facade_report_stdout_includes_repository_surface_sections() {
    let project = TempProject::new("dev-facade-report-repository-surface");

    fs::write(
        project.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/*\"]\nresolver = \"3\"\n",
    )
    .expect("failed to write Cargo.toml");

    fs::create_dir_all(project.path().join(".git")).expect("failed to create .git directory");
    fs::create_dir_all(project.path().join(".cargo")).expect("failed to create .cargo");
    fs::create_dir_all(project.path().join(".github")).expect("failed to create .github");
    fs::create_dir_all(project.path().join("crates/use-example/src"))
        .expect("failed to create crate src");

    fs::write(
        project.path().join("crates/use-example/Cargo.toml"),
        "[package]\nname = \"use-example\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    )
    .expect("failed to write child Cargo.toml");

    fs::write(
        project.path().join("crates/use-example/src/lib.rs"),
        "#![forbid(unsafe_code)]\n",
    )
    .expect("failed to write lib.rs");

    let output = run_success(
        CliBinary::Rustuse,
        &["dev", "facade", "report", ".", "--stdout"],
        project.path(),
    );

    assert!(output.contains("## Repository Surface"));
    assert!(output.contains("## Standard File Consistency"));
    assert!(output.contains("## Tooling Configuration"));
    assert!(output.contains("## Development Environment"));
    assert!(output.contains(".cargo/config.toml"));
    assert!(output.contains("## CI/CD Surface"));
    assert!(output.contains("## Documentation Surface"));
    assert!(output.contains("## Release Surface"));
    assert!(output.contains("## Generated / Local Artifacts"));
    assert!(output.contains("| `.cargo` | yes |"));
    assert!(output.contains("| `.github/` | yes |"));
}

#[test]
fn dev_facade_report_flags_docs_directory_as_non_standard() {
    let project = TempProject::new("dev-facade-report-non-standard-docs");

    fs::write(
        project.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/*\"]\nresolver = \"3\"\n",
    )
    .expect("failed to write Cargo.toml");

    fs::create_dir_all(project.path().join(".git")).expect("failed to create .git directory");
    fs::create_dir_all(project.path().join("docs")).expect("failed to create docs directory");
    fs::create_dir_all(project.path().join("crates/use-example/src"))
        .expect("failed to create crate src");

    fs::write(
        project.path().join("crates/use-example/Cargo.toml"),
        "[package]\nname = \"use-example\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    )
    .expect("failed to write child Cargo.toml");

    fs::write(
        project.path().join("crates/use-example/src/lib.rs"),
        "#![forbid(unsafe_code)]\n",
    )
    .expect("failed to write lib.rs");

    let output = run_success(
        CliBinary::Rustuse,
        &["dev", "facade", "report", ".", "--stdout"],
        project.path(),
    );

    assert!(output.contains("## Non-standard Paths"));
    assert!(
        output.contains("| `docs/` | Move facade documentation to the central docs repository. |")
    );
    assert!(output.contains("## Documentation Surface"));
    assert!(!output.contains("docs/maintainer-release-flow.md"));
    assert!(output.contains("- [Non-standard Paths](#non-standard-paths)"));
}

#[test]
fn dev_facade_report_includes_github_gitlab_and_release_ci_surfaces() {
    let project = TempProject::new("use-example");

    fs::write(
        project.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/*\"]\nresolver = \"3\"\n",
    )
    .expect("failed to write Cargo.toml");

    fs::create_dir_all(project.path().join(".git")).expect("failed to create .git directory");
    fs::create_dir_all(project.path().join(".github/workflows"))
        .expect("failed to create GitHub workflows directory");
    fs::create_dir_all(project.path().join(".gitlab")).expect("failed to create GitLab directory");

    fs::write(
        project.path().join(".github/dependabot.yml"),
        "version: 2\nupdates: []\n",
    )
    .expect("failed to write dependabot.yml");

    fs::write(project.path().join(".gitlab-ci.yml"), "stages: []\n")
        .expect("failed to write .gitlab-ci.yml");

    fs::write(project.path().join("release-plz.toml"), "[workspace]\n")
        .expect("failed to write release-plz.toml");

    for (path, contents) in [
        ("README.md", "# use-example\n"),
        ("CHANGELOG.md", "# Changelog\n"),
        ("CONTRIBUTING.md", "# Contributing\n"),
        ("GOVERNANCE.md", "# Governance\n"),
        ("MAINTAINERS.md", "# Maintainers\n"),
        ("RELEASE.md", "# Release\n"),
        ("RELEASING.md", "# Releasing\n"),
        (
            "Cargo.lock",
            "# This file is automatically @generated by Cargo.\n",
        ),
    ] {
        fs::write(project.path().join(path), contents)
            .unwrap_or_else(|error| panic!("failed to write {path}: {error}"));
    }

    fs::create_dir_all(project.path().join("crates/use-example/src"))
        .expect("failed to create facade crate src");

    fs::write(
        project.path().join("crates/use-example/Cargo.toml"),
        "[package]\nname = \"use-example\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    )
    .expect("failed to write facade Cargo.toml");

    fs::write(
        project.path().join("crates/use-example/README.md"),
        "# use-example\n",
    )
    .expect("failed to write README.md");

    fs::write(
        project.path().join("crates/use-example/src/lib.rs"),
        "#![forbid(unsafe_code)]\n",
    )
    .expect("failed to write lib.rs");

    fs::write(
        project.path().join("crates/use-example/src/prelude.rs"),
        "pub use crate::*;\n",
    )
    .expect("failed to write prelude.rs");

    fs::create_dir_all(project.path().join(".cargo")).expect("failed to create .cargo");
    fs::create_dir_all(project.path().join(".devcontainer"))
        .expect("failed to create .devcontainer");
    fs::create_dir_all(project.path().join("scripts")).expect("failed to create scripts");

    for (path, contents) in [
        (".cargo/config.toml", ""),
        (".clippy.toml", ""),
        (".rustfmt.toml", ""),
        (".taplo.toml", ""),
        ("deny.toml", ""),
        (".gitleaks.toml", ""),
        (".trivyignore", ""),
        ("rust-toolchain.toml", "[toolchain]\nchannel = \"1.95.0\"\n"),
        (".devcontainer/devcontainer.json", "{}\n"),
        (".devcontainer/post-create.sh", "#!/usr/bin/env sh\n"),
        ("scripts/bootstrap-dev-tools.ps1", ""),
        ("scripts/bootstrap-dev-tools.sh", "#!/usr/bin/env sh\n"),
        ("scripts/sync-mirrors.sh", "#!/usr/bin/env sh\n"),
    ] {
        fs::write(project.path().join(path), contents)
            .unwrap_or_else(|error| panic!("failed to write {path}: {error}"));
    }

    let output = run_success(
        CliBinary::Rustuse,
        &["dev", "facade", "report", ".", "--stdout"],
        project.path(),
    );

    assert!(output.contains("## CI/CD Surface"));
    assert!(output.contains("### GitHub CI/CD Surface"));
    assert!(output.contains("### Required GitHub Workflows"));
    assert!(output.contains("### GitLab CI Surface"));
    assert!(output.contains("### Release CI/CD Surface"));
    assert!(output.contains("- GitLab CI surface: `2/2`"));
    assert!(output.contains("| `.gitlab/` | yes |"));
    assert!(output.contains("| `.gitlab-ci.yml` | yes |"));
    assert!(output.contains("| `release-plz.toml` | yes |"));
    assert!(output.contains("## Documentation Surface\n\n- Status: **ok**\n- Present: `5/5`"));
    assert!(output.contains("## Tooling Configuration\n\n- Status: **ok**\n- Present: `8/8`"));
    assert!(output.contains("## Development Environment\n\n- Status: **ok**\n- Present: `6/6`"));
    assert!(output.contains("## Release Surface\n\n- Status: **ok**\n- Present: `5/5`"));
    assert!(output.contains("- Release CI/CD surface: `1/1`"));
}

#[test]
fn dev_facade_report_includes_generated_local_artifacts() {
    let project = TempProject::new("dev-facade-report-generated-artifacts");

    fs::write(
        project.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/*\"]\nresolver = \"3\"\n",
    )
    .expect("failed to write Cargo.toml");

    fs::create_dir_all(project.path().join(".git")).expect("failed to create .git directory");
    fs::create_dir_all(project.path().join("crates/use-example/src"))
        .expect("failed to create crate src");

    fs::write(
        project.path().join("crates/use-example/Cargo.toml"),
        "[package]\nname = \"use-example\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    )
    .expect("failed to write child Cargo.toml");

    fs::write(
        project.path().join("crates/use-example/src/lib.rs"),
        "#![forbid(unsafe_code)]\n",
    )
    .expect("failed to write lib.rs");

    fs::create_dir_all(project.path().join("target/flycheck0"))
        .expect("failed to create flycheck0 directory");
    fs::create_dir_all(project.path().join("target/flycheck20"))
        .expect("failed to create flycheck20 directory");

    let output = run_success(
        CliBinary::Rustuse,
        &["dev", "facade", "report", ".", "--stdout"],
        project.path(),
    );

    assert!(output.contains("## Generated / Local Artifacts"));
    assert!(output.contains("| `target` | Cargo build output |"));
    assert!(output.contains("| `target/flycheck0` | rust-analyzer flycheck output |"));
    assert!(output.contains("| `target/flycheck20` | rust-analyzer flycheck output |"));
}
