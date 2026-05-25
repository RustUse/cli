# rustuse CLI

`rustuse` is the command-line tool for RustUse adoption workflows.

RustUse supports three distribution modes:

- Cargo mode installs a RustUse crate as a normal dependency. RustUse owns the crate; your project depends on it.
- Copy mode copies RustUse source into your project. Your project owns the copied source after adoption.
- CLI-assisted adoption helps find, add, copy, track, and inspect RustUse primitives. The CLI is not a package manager and does not replace Cargo.

This v0.1 scaffold is intentionally small. It parses the intended command surface, searches a placeholder in-memory index, prints docs URLs, and reports what commands would do. It does not edit `Cargo.toml`, copy source files, fetch network data, or create `rustuse.lock` yet.

## Project tracking

`rustuse.toml` is optional human-editable project configuration. Cargo-only workflows and copy-only workflows do not require it.

Run `rustuse init` when you want managed RustUse project state. The command creates `rustuse.toml`, `.rustuse/cache/`, and `.rustuse/snapshots/`. It is idempotent, does not overwrite an existing config, and does not modify `Cargo.toml`, copy RustUse source, add dependencies, or create `rustuse.lock`.

Use `rustuse init --copy-first` to make copy mode the default in generated project configuration. Use `rustuse init --dry-run` to preview the files and directories without writing them.

## Cargo subcommand

Installing `rustuse-cli` provides two binaries:

- `rustuse`
- `cargo-rustuse`

The second binary lets Cargo run RustUse as:

```bash
cargo rustuse <command>
```

These are equivalent:

```bash
rustuse search geometry
cargo rustuse search geometry

rustuse add use-geometry
cargo rustuse add use-geometry

rustuse copy use-slug
cargo rustuse copy use-slug
```

`cargo rustuse` is a Cargo-native entry point, not a separate package manager. It uses the same RustUse CLI implementation.

## Examples

```bash
rustuse init
rustuse init --copy-first
rustuse search geometry
rustuse info use-geometry
rustuse add use-geometry
rustuse copy use-slug
rustuse copy use-slug --with-tests
rustuse add use-slug --copy
rustuse add use-slug --copy --with-tests
rustuse list
rustuse docs use-math --workspace
rustuse doctor
```

Global flags:

```bash
rustuse --verbose search geometry
rustuse --quiet doctor
rustuse --json info use-geometry
```
