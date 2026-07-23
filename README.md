# RustUse CLI

[![Crates.io](https://img.shields.io/crates/v/rustuse-cli.svg)](https://crates.io/crates/rustuse-cli)
[![Documentation](https://docs.rs/rustuse-cli/badge.svg)](https://docs.rs/rustuse-cli)
[![CI](https://github.com/RustUse/cli/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/RustUse/cli/actions/workflows/ci.yml)
[![CodeQL](https://github.com/RustUse/cli/actions/workflows/codeql.yml/badge.svg?branch=main)](https://github.com/RustUse/cli/actions/workflows/codeql.yml)
[![Trivy](https://github.com/RustUse/cli/actions/workflows/trivy.yml/badge.svg?branch=main)](https://github.com/RustUse/cli/actions/workflows/trivy.yml)
[![MSRV](https://img.shields.io/badge/MSRV-1.95.0-blue.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/crates/l/rustuse-cli.svg)](https://github.com/RustUse/cli#license)

`rustuse` is the command-line tool for discovering, adopting, inspecting, validating, and maintaining RustUse crates and repositories.

RustUse focuses on small, composable Rust crates. The CLI helps Rust users find and adopt those crates through Cargo, while providing RustUse maintainers with repository inspection, reporting, repair, and CI workflows.

The RustUse CLI is not a package manager and does not replace Cargo. It is a workflow layer built around Cargo, local repository inspection, and RustUse conventions.

> `rustuse-cli` is currently in beta. Command names, configuration formats, and generated output may change before the first stable release.

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

The `cargo-rustuse` binary allows Cargo to invoke RustUse as a Cargo subcommand:

```bash
cargo rustuse <command>
```

For example, these command pairs are equivalent:

```bash
rustuse search geometry
cargo rustuse search geometry

rustuse info use-geometry
cargo rustuse info use-geometry

rustuse add use-geometry
cargo rustuse add use-geometry
```

## Quick start

Search the RustUse catalog:

```bash
rustuse search geometry
```

Inspect a catalog entry:

```bash
rustuse info use-geometry
```

Preview adding a RustUse crate:

```bash
rustuse add use-geometry --dry-run
```

Add the dependency through Cargo:

```bash
rustuse add use-geometry
```

Inspect the current project:

```bash
rustuse check .
rustuse doctor .
```

Run `rustuse` without a subcommand in an interactive terminal to open the guided menu:

```bash
rustuse
```

## Command surface

Run `rustuse <command> --help` for the complete arguments and options supported by a command.

### Discovery and adoption

```bash
rustuse search <query> [--limit <count>]
rustuse info <name>
rustuse add <crate> [--dry-run]
rustuse remove [path]
rustuse list [path] [--all]
rustuse outdated [path]
rustuse update [path]
```

### Project tools

```bash
rustuse init [path] [--dry-run] [--force]
rustuse check [path]
rustuse diff [path]
rustuse doctor [path]
rustuse docs [name] [--api | --workspace]
```

### Catalog tools

```bash
rustuse catalog discover [path]
rustuse catalog generate [path] [--output <path>]
rustuse catalog check [path]
rustuse catalog info <name>
rustuse catalog search <query> [--limit <count>]
```

### Maintainer tools

```bash
rustuse dev inspect [path]
rustuse dev report [path]
rustuse dev report [path] --fleet
rustuse dev fix [path]
rustuse dev fix [path] --all
rustuse dev fix [path] --all --write
```

Run `rustuse dev` without a dev subcommand to open the guided maintainer menu.

### Automation

```bash
rustuse ci check [path]
rustuse ci check [path] --deny-warnings
```

### CLI utilities

```bash
rustuse completions <shell>
rustuse upgrade [--dry-run]
rustuse ferris
```

## Global flags

Global flags may be supplied before or after a subcommand:

```bash
rustuse --verbose search geometry
rustuse search geometry --verbose

rustuse --quiet doctor .
rustuse --json info use-geometry
rustuse --yes dev fix . --all --write
rustuse --non-interactive ci check .
```

Available global flags:

- `-v`, `--verbose` shows additional execution details.
- `-q`, `--quiet` prints only essential output.
- `--json` emits machine-readable JSON where supported.
- `--yes` accepts safe command-specific defaults without prompting.
- `--non-interactive` disables prompts and fails when required input has no default.

`--yes`, `--non-interactive`, and `--json` require an explicit command. They do not select a default workflow when `rustuse` or `rustuse dev` is invoked without a subcommand.

## Adoption model

RustUse crates are adopted through Cargo.

The CLI assists with:

- catalog discovery
- dependency adoption and removal
- documentation lookup
- version inspection and updates
- optional project tracking
- repository validation
- maintainer workflows

External projects can depend on RustUse crates through Cargo without adding RustUse-specific configuration.

Use `rustuse init` only when a project should opt into optional RustUse-managed state through `rustuse.toml`.

## Adoption commands

### `search`

Searches the RustUse catalog for crates and primitives.

```bash
rustuse search geometry
rustuse search quant
rustuse search constants
```

Use `--limit` to restrict the number of results:

```bash
rustuse search geometry --limit 10
```

### `info`

Shows catalog metadata for a RustUse crate, primitive, facade, or package.

```bash
rustuse info use-geometry
rustuse info use-point
```

### `add`

Adds a RustUse crate as a Cargo dependency.

RustUse validates the requested catalog entry and delegates dependency installation to Cargo.

```bash
rustuse add use-geometry
rustuse add use-point
```

Preview the operation without modifying the project:

```bash
rustuse add use-geometry --dry-run
```

### `remove`

Removes a RustUse dependency through the current project workflow.

```bash
rustuse remove
```

Use `rustuse remove --help` for the current path and selection options.

### `list`

Shows RustUse project tracking or catalog entries.

```bash
rustuse list
rustuse list . --all
```

### `outdated`

Finds outdated RustUse crate and primitive dependencies.

```bash
rustuse outdated
rustuse outdated .
```

### `update`

Updates RustUse Cargo dependencies used by the current project.

```bash
rustuse update
rustuse update .
```

`update` changes project dependencies. It does not upgrade the installed RustUse CLI.

## Project tools

### `init`

Initializes optional RustUse project tracking with `rustuse.toml`.

```bash
rustuse init
rustuse init .
```

Preview the planned files and directories without writing them:

```bash
rustuse init --dry-run
```

RustUse configuration is optional for Cargo-only adoption.

### `check`

Checks a Cargo project and its optional RustUse tracking state.

```bash
rustuse check
rustuse check .
```

### `diff`

Shows differences detected by the current RustUse project workflow.

```bash
rustuse diff
rustuse diff .
```

### `doctor`

Diagnoses RustUse CLI, Cargo, environment, and project configuration problems.

```bash
rustuse doctor
rustuse doctor .
```

`check` focuses on project state. `doctor` is intended for broader diagnosis of the RustUse environment and configuration.

### `docs`

Prints RustUse website, API, or workspace documentation URLs.

```bash
rustuse docs
rustuse docs use-math
rustuse docs use-math --api
rustuse docs use-math --workspace
```

## Catalog tools

The catalog command exposes lower-level catalog discovery, generation, inspection, search, and validation workflows.

### Discover local entries

```bash
rustuse catalog discover .
```

### Generate catalog artifacts

```bash
rustuse catalog generate .
rustuse catalog generate . --output rustuse-catalog.json
```

### Validate catalog state

```bash
rustuse catalog check .
```

### Inspect an entry

```bash
rustuse catalog info use-geometry
```

### Search entries

```bash
rustuse catalog search geometry
rustuse catalog search geometry --limit 10
```

## Optional configuration

RustUse supports two configuration surfaces:

- standard Cargo package and workspace metadata in `Cargo.toml`
- optional RustUse-managed project state in `rustuse.toml`

Cargo remains authoritative for normal package identity and dependency configuration.

`rustuse.toml` is intended for RustUse-specific local workflow settings and managed project state. It is not required for projects that only consume RustUse crates through Cargo.

The configuration schema is still evolving during the beta release series. Use the files generated by the installed version of `rustuse init` as the source of truth for that version.

## Maintainer workflows

Maintainer workflows live under `rustuse dev`.

They are intended for RustUse facade repositories and development directories containing multiple RustUse repositories.

### Inspect a facade

```bash
cd use-fs
rustuse dev inspect .
```

Inspection analyzes repository shape and reports findings without writing managed changes.

### Generate a facade report

```bash
cd use-fs
rustuse dev report .
```

The default facade report file is:

```text
rustuse-report.md
```

A facade report may include:

- facade repository shape
- child crate discovery
- Cargo manifest health
- child crate inventory
- facade feature and dependency wiring
- package and workspace metadata
- standard file consistency
- CI and release surfaces
- documentation surfaces
- recommended actions

### Generate a fleet report

A fleet is a RustUse development directory containing multiple repositories, including `use-*` facade repositories.

Example:

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

Generate the fleet report:

```bash
cd git_local
rustuse dev report . --fleet
```

The default fleet report file is:

```text
rustuse-fleet-report.md
```

The fleet directory does not need to be a Git repository. RustUse inspects the repositories contained within it.

A fleet report may include:

- development directory and repository discovery
- `use-*` facade discovery
- Git repository checks
- child crate and Cargo manifest counts
- facade inventory
- issue summaries
- category and metadata validation
- standard file consistency
- recommended actions

### Repair facade drift

Plan repairs without writing changes:

```bash
rustuse dev fix .
rustuse dev fix . --all
```

Apply all selected repairs:

```bash
rustuse dev fix . --all --write
```

By default, inspection, reporting, and repair planning are non-writing. `rustuse dev fix` writes changes only when `--write` is supplied.

The interactive maintainer menu can select all repairs or specific issue codes:

```bash
rustuse dev
```

## Facade maintenance model

A `use-*` facade should generally be a thin feature-export crate.

Child crates own behavior. Facades coordinate the public surface.

Facade responsibilities commonly include:

- optional child dependencies
- feature flags
- public re-exports
- prelude exports
- workspace metadata
- README discovery tables
- repository-level maintenance files

RustUse checks alignment across surfaces such as:

```text
crates/use-* directories
Cargo.toml workspace dependencies
Cargo.toml facade dependencies
Cargo.toml features
src/lib.rs exports
src/prelude.rs exports
README child tables
documentation catalogs
CI and release configuration
```

A facade must not contain a child crate with the same package name as the facade itself. For example, `use-quant` must not contain `crates/use-quant`.

## Continuous integration

RustUse CI commands are designed for deterministic, non-interactive validation in local automation and hosted CI systems.

```bash
rustuse ci check .
```

Fail the command when warnings are found:

```bash
rustuse ci check . --deny-warnings
```

The CI surface is intended to remain:

- stable as an automation entry point
- non-interactive
- safe by default
- non-writing
- machine-friendly

Use `--json` where the selected CI workflow supports structured output.

## Shell completions

Generate a completion script for a supported shell:

```bash
rustuse completions bash
rustuse completions zsh
rustuse completions fish
rustuse completions powershell
```

The generated script is written to standard output so it can be redirected to the appropriate shell configuration location.

## Upgrading the CLI

Upgrade the installed RustUse CLI through Cargo:

```bash
rustuse upgrade
```

Preview the Cargo command without running it:

```bash
rustuse upgrade --dry-run
```

The upgrade workflow uses:

```text
cargo install rustuse-cli --force --locked
```

`upgrade` replaces the installed RustUse CLI. `update` changes RustUse dependencies in a project.

## Generated artifacts

RustUse workflows may generate local development artifacts such as:

```text
rustuse-report.md
rustuse-fleet-report.md
.rustuse/
```

Generated reports should be deterministic where practical.

Report files may be committed when they are intentionally used as repository health snapshots. Cache and temporary managed state should follow the ignore policy of the repository using RustUse.

## CLI development

Format the code:

```bash
cargo fmt
```

Verify formatting:

```bash
cargo fmt --check
```

Build the CLI:

```bash
cargo build
```

Run tests:

```bash
cargo test
```

Run tests with all features:

```bash
cargo test --all-features
```

Run Clippy with warnings denied:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

Generate private-item documentation without opening a browser:

```bash
cargo doc --no-deps --document-private-items
```

Generate documentation and open it:

```bash
cargo doc --no-deps --document-private-items --open
```

Install the current local build:

```bash
cargo install --path . --force
```

## Design principles

The CLI is intentionally conservative.

- Inspection, reporting, checking, and dry-run workflows should be safe to repeat.
- Commands that write managed changes require explicit write-oriented options.
- Cargo remains the dependency and package-management authority.
- Simple Cargo adoption does not require RustUse configuration.
- Command modules parse arguments, adapt them into RustUse workflows, and render results.
- RustUse workflow modules own repository and domain behavior.
- External-system adapters remain focused and reusable.

## License

RustUse is dual-licensed under either:

- Apache License, Version 2.0
- MIT License

You may choose either license at your option.

See `LICENSE-APACHE` and `LICENSE-MIT` for details.
