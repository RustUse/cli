# rustuse CLI

`rustuse` is the command-line tool for RustUse adoption and development workflows.

RustUse supports three adoption modes:

- **Cargo mode** adds a RustUse crate as a normal Rust dependency. RustUse owns the crate; your project depends on it through Cargo.
- **Copy mode** copies RustUse source into your project. Your project owns the copied source after adoption.
- **CLI-assisted adoption** helps find, add, copy, track, inspect, and validate RustUse primitives.

The `rustuse` CLI is not a package manager and does not replace Cargo. It is a workflow tool for adopting RustUse crates, inspecting RustUse packages, and maintaining RustUse repositories.

## Configuration model

RustUse supports two configuration surfaces:

- `Cargo.toml` metadata for Cargo-native package and workspace metadata.
- `rustuse.toml` for RustUse CLI project state and RustUse-specific workflow policy.

Both are supported intentionally.

The Cargo-native metadata path is useful for Rust projects that want RustUse information to live inside the normal Cargo manifest. The `rustuse.toml` path is useful when a project wants explicit RustUse-managed state without adding extra tool-specific configuration to `Cargo.toml`.

RustUse-owned facade repositories should generally include both:

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
copy_supported = true
```

For an individual package:

```toml
[package.metadata.rustuse]
kind = "primitive"
facade = "use-geometry"
slug = "use-point"
homepage = "https://rustuse.org/use-geometry/use-point"
copy_supported = true
```

Cargo metadata is best for information that belongs to the crate or workspace itself:

- facade name
- primitive slug
- RustUse homepage
- documentation path
- adoption support
- copy-mode support
- category or domain hints
- generated catalog hints

RustUse should avoid duplicating normal Cargo fields such as `name`, `version`, `edition`, `license`, `repository`, `homepage`, and `documentation` unless the RustUse-specific value has a different meaning.

## `rustuse.toml`

`rustuse.toml` is optional, human-editable project configuration.

Cargo-only and copy-only workflows do not require `rustuse.toml`. Run `rustuse init` when you want managed RustUse project state.

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

Use copy-first mode when you want copied source to be the default adoption strategy:

```bash
rustuse init --copy-first
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

## Cargo subcommand

Installing `rustuse-cli` provides two binaries:

```bash
cargo install --path . --force
```

Installed binaries:

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

rustuse add use-geometry
cargo rustuse add use-geometry

rustuse copy use-slug
cargo rustuse copy use-slug
```

`cargo rustuse` uses the same CLI implementation as `rustuse`.

## Current command surface

```bash
rustuse init
rustuse init --copy-first
rustuse init --dry-run

rustuse search geometry
rustuse info use-geometry
rustuse list

rustuse add use-geometry
rustuse copy use-slug
rustuse copy use-slug --with-tests

rustuse add use-slug --copy
rustuse add use-slug --copy --with-tests

rustuse docs use-math --workspace
rustuse doctor
```

Global flags:

```bash
rustuse --verbose search geometry
rustuse --quiet doctor
rustuse --json info use-geometry
```

## Planned adoption API

The long-term RustUse adoption workflow is expected to support:

```bash
rustuse search <query>
rustuse info <crate-or-facade>
rustuse add <crate>
rustuse copy <crate>
rustuse list
rustuse docs <crate-or-facade>
rustuse doctor
```

Planned behavior:

- `search` finds RustUse facades and primitives.
- `info` prints crate metadata, docs links, adoption options, and feature information.
- `add` adds a RustUse crate through Cargo mode.
- `copy` copies RustUse source into the current project.
- `list` shows adopted RustUse crates and copied primitives.
- `docs` opens or prints RustUse documentation URLs.
- `doctor` validates local RustUse configuration and adoption state.

Future managed-state behavior may include:

- reading `Cargo.toml` RustUse metadata
- reading `rustuse.toml`
- writing `rustuse.lock`
- tracking copied source snapshots
- checking copied source drift
- validating facade workspace shape
- validating standard RustUse repository files
- validating crates.io metadata
- validating docs links and homepage links

## Development

This section covers development workflows for the `rustuse` CLI itself.

### CLI development workflow

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

## Root development commands

Root development commands are intended to run against a RustUse development root that contains multiple `use-*` facade repositories.

Generate a root development report:

```bash
rustuse dev root report .
```

The root report is intended to be saved as:

```text
rustuse-report.md
```

The root report includes:

- root repository discovery
- `use-*` facade discovery
- Git repository checks
- child crate counts
- Cargo manifest health
- crates.io category validation
- standard file consistency
- facade inventory
- recommended action plan

Run a root manifest check from the CLI repository against the parent development root:

```bash
rustuse dev root manifests ..
```

Apply facade wiring fixes for a specific facade:

```bash
rustuse dev root manifests .. --fix --code facade-wiring --facade use-geometry --write
rustuse dev root manifests .. --fix --code facade-wiring --facade use-math --write
rustuse dev root manifests .. --fix --code facade-wiring --facade use-quant --write
```

## Facade development commands

Facade development commands are intended to run from inside a `use-*` facade repository.

Generate a facade development report:

```bash
rustuse dev facade report
```

## Planned development API

The RustUse development command surface is intended to grow around three scopes:

```bash
rustuse dev root <command>
rustuse dev facade <command>
rustuse dev github <command>
```

Root-level commands inspect a development root containing many RustUse repositories:

```bash
rustuse dev root report .
rustuse dev root manifests .
```

Facade-level commands inspect one `use-*` facade repository:

```bash
rustuse dev facade report
```

GitHub-level commands inspect or prepare GitHub repository metadata, labels, issue policy, and generated reports:

```bash
rustuse dev github report
```

Planned development checks include:

- facade repository shape
- child crate discovery
- workspace member consistency
- workspace dependency consistency
- facade-to-child wiring
- crates.io category validity
- package metadata completeness
- standard file consistency
- README and docs link consistency
- release configuration
- GitHub issue label policy
- generated root and facade reports

## Generated artifacts

RustUse may generate local development artifacts during inspection, reporting, and managed adoption workflows.

Expected generated artifacts include:

```text
rustuse-report.md
.rustuse/cache/
.rustuse/snapshots/
```

Future generated artifacts may include:

```text
rustuse.lock
```

Generated artifacts should be deterministic where possible. Files that represent useful repository health snapshots, such as `rustuse-report.md`, may be committed when they are intentionally part of the repository maintenance workflow.

Cache and snapshot directories should be treated as managed RustUse state.

## Notes

The CLI is intentionally conservative. Commands that inspect, report, or preview changes should be safe to run repeatedly. Commands that write changes require explicit write-oriented flags such as `--write` where applicable.

RustUse should prefer explicit configuration over hidden behavior, but it should not require configuration for simple Cargo-only usage.

## License

RustUse is dual-licensed under either of:

- Apache License, Version 2.0
- MIT License

You may choose either license, at your option.

See `LICENSE-APACHE` and `LICENSE-MIT` for details.
