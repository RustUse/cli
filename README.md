# rustuse CLI

`rustuse` is the command-line tool for RustUse adoption, inspection, reporting, and maintainer workflows.

RustUse focuses on small, composable Rust crates. The CLI helps users find and adopt RustUse crates, and helps maintainers keep RustUse facade repositories aligned.

The `rustuse` CLI is not a package manager and does not replace Cargo. It is a workflow tool layered on top of Cargo, local repository inspection, and RustUse conventions.

## Adoption model

RustUse supports three adoption modes:

- **Cargo mode** adds a RustUse crate as a normal Rust dependency. RustUse owns the crate; your project depends on it through Cargo.
- **CLI-assisted adoption** helps find, add, track, inspect, and validate RustUse primitives.

External projects can use RustUse through Cargo without any RustUse-specific configuration.

## Installation

Install a local development build from this repository:

```bash
cargo install --path . --force
```

Installing `rustuse-cli` provides two binaries:

```text
rustuse
cargo-rustuse
```

The `cargo-rustuse` binary lets Cargo run RustUse as a native Cargo subcommand:

```bash
cargo rustuse <command>
```

These commands are equivalent:

```bash
rustuse search geometry
cargo rustuse search geometry

rustuse info use-geometry
cargo rustuse info use-geometry

rustuse add use-geometry
cargo rustuse add use-geometry
```

## Current command surface

```bash
rustuse init
rustuse search <query>
rustuse info <crate-or-facade>
rustuse list [path]

rustuse add <crate>
rustuse add <crate> --dry-run

rustuse docs [crate-or-facade] [--api|--workspace]
rustuse doctor [path]

rustuse catalog discover [path]
rustuse catalog generate [path] [--output <path>]
rustuse catalog check [path]
rustuse catalog info <name>
rustuse catalog search <query> [--limit <count>]

rustuse dev inspect [path]
rustuse dev report [path]
rustuse dev report [path] --fleet

rustuse ci check [path]
rustuse ci check [path] --deny-warnings
```

Global flags:

```bash
rustuse --verbose search geometry
rustuse --quiet doctor
rustuse --json info use-geometry
```

## Configuration model

RustUse supports two configuration surfaces:

- `Cargo.toml` metadata for Cargo-native package and workspace metadata.
- `rustuse.toml` for RustUse CLI project state and RustUse-specific workflow policy.

Both are supported intentionally.

The Cargo-native metadata path is useful for Rust projects that want RustUse information to live inside the normal Cargo manifest. The `rustuse.toml` path is useful when a project wants explicit RustUse-managed state without adding extra tool-specific configuration to `Cargo.toml`.

RustUse-owned facade repositories should generally include:

```text
Cargo.toml
rustuse.toml
.rustuse/
```

External projects adopting RustUse do not need `rustuse.toml` unless they want managed RustUse project state.

## Cargo metadata

RustUse metadata may be stored in `Cargo.toml` using standard Cargo metadata tables.

For a workspace or facade repository:

```toml
[workspace.metadata.rustuse]
kind = "facade"
facade = "use-geometry"
homepage = "https://rustuse.org/use-geometry"
default_adoption = "cargo"
```

For an individual package:

```toml
[package.metadata.rustuse]
kind = "primitive"
facade = "use-geometry"
slug = "use-point"
homepage = "https://rustuse.org/use-geometry/use-point"
```

Cargo metadata is best for information that belongs to the crate or workspace itself:

- facade name
- primitive slug
- RustUse homepage
- documentation path
- adoption support
- category or domain hints
- generated catalog hints

RustUse should avoid duplicating normal Cargo fields such as `name`, `version`, `edition`, `license`, `repository`, `homepage`, and `documentation` unless the RustUse-specific value has a different meaning.

## `rustuse.toml`

`rustuse.toml` is optional, human-editable project configuration.

Cargo adoption does not require `rustuse.toml`. Run `rustuse init` when you want managed RustUse project state.

```bash
rustuse init
```

The command creates:

```text
rustuse.toml
.rustuse/cache/
.rustuse/snapshots/
```

`rustuse init` is idempotent. It does not overwrite an existing config, modify `Cargo.toml`, copy RustUse source, add dependencies, or create `rustuse.lock`.

A minimal `rustuse.toml` for an external adopting project may look like this:

```toml
[project]
name = "my-project"
default_adoption = "cargo"

[tracking]
snapshots = true
cache = true
```

A minimal `rustuse.toml` for a RustUse-owned facade repository may look like this:

```toml
[project]
name = "use-geometry"
kind = "facade"
default_adoption = "cargo"

[facade]
name = "use-geometry"
crates_dir = "crates"
homepage = "https://rustuse.org/use-geometry"

[dev]
standard_files = true
manifest_checks = true
facade_wiring_checks = true
```

Preview generated files and directories without writing them:

```bash
rustuse init --dry-run
```

## Configuration precedence

When both `Cargo.toml` metadata and `rustuse.toml` are present, RustUse should read both.

Recommended rules:

1. Cargo package fields remain authoritative for Cargo package identity.
2. `Cargo.toml` RustUse metadata describes public crate and workspace metadata.
3. `rustuse.toml` describes RustUse CLI behavior, local workflow policy, and managed project state.
4. If the same RustUse-specific setting exists in both files, `rustuse.toml` should override local behavior.
5. If the files disagree about crate identity, facade identity, or workspace shape, the CLI should warn or fail validation instead of silently choosing one.

## Adoption commands

### `search`

Searches the RustUse catalog.

```bash
rustuse search geometry
rustuse search quant
rustuse search constants
```

### `info`

Prints catalog information for a RustUse facade or crate.

```bash
rustuse info use-geometry
rustuse info use-point
```

### `add`

Adds a RustUse crate through Cargo mode by invoking `cargo add`. Use
`--dry-run` to validate the catalog entry without changing the manifest.

```bash
rustuse add use-geometry
rustuse add use-point
```

### `list`

Lists adopted or known RustUse crates for the current project.

```bash
rustuse list
```

### `docs`

Prints or opens RustUse documentation links.

```bash
rustuse docs use-math
rustuse docs use-math --workspace
```

### `doctor`

Validates local RustUse configuration and adoption state.

```bash
rustuse doctor
```

## Maintainer reports

The `dev report` command generates human-readable maintainer reports.

```bash
rustuse dev report [path]
```

Use `--fleet` for a RustUse development directory that contains multiple facade repositories:

```bash
rustuse dev report . --fleet
```

### Facade report

A facade report is intended to run inside a single `use-*` facade repository.

```bash
cd use-fs
rustuse dev inspect .
rustuse dev report .
```

The default facade report file is:

```text
rustuse-report.md
```

A facade report includes:

- facade repository shape
- child crate discovery
- Cargo manifest health
- child crate inventory
- crate documentation consistency
- standard file consistency
- non-standard paths
- tooling configuration
- CI/CD surface
- documentation surface
- release surface
- generated and local artifacts
- recommended action plan

### Fleet report

A fleet report is intended to run against a RustUse development directory containing multiple repositories.

Example development directory:

```text
git_local/
  .github/
  .github-private/
  cli/
  docs/
  infra/
  mcp/
  rustuse/
  use-geometry/
  use-math/
  use-quant/
  use-*
```

Generate a fleet report:

```bash
cd git_local
rustuse dev report . --fleet
```

The default fleet report file is:

```text
rustuse-fleet-report.md
```

A fleet directory may not itself be a Git repository. The CLI treats it as a development directory and inspects the repositories inside it.

A fleet report includes:

- development directory discovery
- repository discovery
- `use-*` facade discovery
- Git repository checks for child repositories
- child crate counts
- Cargo manifest health
- crates.io category validation
- facade inventory
- top manifest offenders
- standard file consistency summaries
- recommended action plan

The fleet report intentionally summarizes large issue sets. Full per-manifest dumps should be reserved for future verbose or focused report modes.

## Facade maintenance model

A `use-*` facade should usually be a thin Rust feature-export crate.

Child crates own behavior.

Facades own:

- optional dependencies
- feature flags
- public re-exports
- prelude exports
- workspace metadata
- README discovery tables
- repository-level maintenance files

The CLI should keep these surfaces aligned:

```text
crates/use-* directories
Cargo.toml workspace dependencies
Cargo.toml facade dependencies
Cargo.toml features
src/lib.rs exports
src/prelude.rs exports
README child table
docs catalog
CI and release configuration
```

If these surfaces disagree, the CLI should report the drift.

The current CLI reports drift without mutating facade surfaces.

Current public maintainer-oriented commands are intentionally conservative:

```bash
rustuse dev inspect .
rustuse dev report .
rustuse dev report . --fleet
rustuse ci check .
```

## Continuous Integration

RustUse CI commands are intended to be used both inside CI systems and locally by maintainers.

```bash
rustuse ci check [path]
rustuse ci check [path] --deny-warnings
```

Example deterministic check:

```bash
rustuse ci check .
```

The CI command should be:

- stable as an automation entrypoint
- safe by default
- profile-based
- machine-friendly
- non-writing unless explicitly designed otherwise

## Generated artifacts

RustUse may generate local development artifacts during inspection, reporting, and managed adoption workflows.

Expected generated artifacts include:

```text
rustuse-report.md
rustuse-fleet-report.md
.rustuse/cache/
.rustuse/snapshots/
```

Generated artifacts should be deterministic where possible. Files that represent useful repository health snapshots may be committed when they are intentionally part of the repository maintenance workflow.

Cache and snapshot directories should be treated as managed RustUse state.

## CLI development workflow

Format the code:

```bash
cargo fmt
```

Build the CLI:

```bash
cargo build
```

Run tests:

```bash
cargo test
```

Run Clippy:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

Generate documentation without opening a browser:

```bash
cargo doc --no-deps --document-private-items
```

Generate documentation and open it in a browser:

```bash
cargo doc --no-deps --document-private-items --open
```

Install the local CLI build:

```bash
cargo install --path . --force
```

## Notes

The CLI is intentionally conservative. Commands that inspect, report, or preview changes should be safe to run repeatedly.

Commands that write changes should require explicit write-oriented flags such as `--write`.

RustUse should prefer explicit configuration over hidden behavior, but it should not require configuration for simple Cargo-only usage.

Internal modules for Cargo manifests, crates.io metadata, GitHub files, GitLab files, release configuration, documentation, reports, scans, and standard files are implementation details. They should stay composable and reusable without becoming public command scopes unless a clear maintainer workflow requires it.

## License

RustUse is dual-licensed under either of:

- Apache License, Version 2.0
- MIT License

You may choose either license, at your option.

See `LICENSE-APACHE` and `LICENSE-MIT` for details.
