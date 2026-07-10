pub(crate) fn write_expected_facade_structure(markdown: &mut String) {
    markdown.push_str("## Expected Facade Structure\n\n");

    markdown.push_str(
        "A RustUse facade repository is expected to be a root package workspace. \
The repository root is both the facade crate and the Cargo workspace root.\n\n",
    );

    markdown.push_str("```txt\n");
    markdown.push_str("use-example/\n");
    markdown.push_str("├── .cargo/\n");
    markdown.push_str("├── .github/\n");
    markdown.push_str("│   └── workflows/\n");
    markdown.push_str("├── .gitlab/\n");
    markdown.push_str("├── crates/\n");
    markdown.push_str("│   ├── use-child-one/\n");
    markdown.push_str("│   │   ├── Cargo.toml\n");
    markdown.push_str("│   │   ├── README.md\n");
    markdown.push_str("│   │   └── src/\n");
    markdown.push_str("│   │       └── lib.rs\n");
    markdown.push_str("│   └── use-child-two/\n");
    markdown.push_str("│       ├── Cargo.toml\n");
    markdown.push_str("│       ├── README.md\n");
    markdown.push_str("│       └── src/\n");
    markdown.push_str("│           └── lib.rs\n");
    markdown.push_str("├── src/\n");
    markdown.push_str("│   ├── lib.rs\n");
    markdown.push_str("│   └── prelude.rs\n");
    markdown.push_str("├── Cargo.toml\n");
    markdown.push_str("├── Cargo.lock\n");
    markdown.push_str("├── README.md\n");
    markdown.push_str("├── CHANGELOG.md\n");
    markdown.push_str("├── CONTRIBUTING.md\n");
    markdown.push_str("├── GOVERNANCE.md\n");
    markdown.push_str("├── MAINTAINERS.md\n");
    markdown.push_str("├── RELEASE.md\n");
    markdown.push_str("├── RELEASING.md\n");
    markdown.push_str("├── release-plz.toml\n");
    markdown.push_str("├── rust-toolchain.toml\n");
    markdown.push_str("├── deny.toml\n");
    markdown.push_str("├── LICENSE\n");
    markdown.push_str("├── LICENSE-APACHE\n");
    markdown.push_str("└── LICENSE-MIT\n");
    markdown.push_str("```\n\n");

    markdown.push_str("### Root package workspace\n\n");
    markdown.push_str(
        "The facade root should contain the publishable facade package and the workspace definition.\n\n",
    );

    markdown.push_str("```toml\n");
    markdown.push_str("[workspace]\n");
    markdown.push_str("members = [\"crates/*\"]\n");
    markdown.push_str("default-members = [\".\"]\n");
    markdown.push_str("resolver = \"3\"\n");
    markdown.push_str("```\n\n");

    markdown.push_str("### Facade feature wiring\n\n");
    markdown.push_str(
        "Child crates should be exposed through optional dependencies and explicit feature gates.\n\n",
    );

    markdown.push_str("```toml\n");
    markdown.push_str("[features]\n");
    markdown.push_str("default = []\n");
    markdown.push_str("full = [\"child-one\", \"child-two\"]\n\n");
    markdown.push_str("child-one = [\"dep:use-child-one\"]\n");
    markdown.push_str("child-two = [\"dep:use-child-two\"]\n\n");
    markdown.push_str("[dependencies]\n");
    markdown.push_str("use-child-one = { workspace = true, optional = true }\n");
    markdown.push_str("use-child-two = { workspace = true, optional = true }\n");
    markdown.push_str("```\n\n");

    markdown.push_str("### Child crate minimum shape\n\n");
    markdown.push_str("Each child crate should follow this minimum structure:\n\n");

    markdown.push_str("```txt\n");
    markdown.push_str("crates/use-child/\n");
    markdown.push_str("├── Cargo.toml\n");
    markdown.push_str("├── README.md\n");
    markdown.push_str("└── src/\n");
    markdown.push_str("    └── lib.rs\n");
    markdown.push_str("```\n\n");

    markdown.push_str("### Workspace lint inheritance\n\n");
    markdown
        .push_str("Child crates should inherit workspace lint policy from the facade root.\n\n");

    markdown.push_str("```toml\n");
    markdown.push_str("[lints]\n");
    markdown.push_str("workspace = true\n");
    markdown.push_str("```\n\n");
}
