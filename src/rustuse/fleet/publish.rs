// Publish
// publish.rs should be a long running script. It should have a 10 minute pause in between each publish to avoid rate limiting.
// It should be able to skip crates that are on the latest version when checking local Cargo.toml version and crates.io
