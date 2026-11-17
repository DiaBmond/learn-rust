# Chapter 14: More About Cargo and Crates.io

## Release Profiles

Cargo uses release profiles to control compilation settings.

- `cargo build` uses the `dev` profile, which prioritizes compilation speed.
- `cargo build --release` uses the `release` profile, which prioritizes runtime performance.

Release profiles can be customized in `Cargo.toml`:

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

Higher optimization levels generally improve runtime performance but increase compilation time.

## Publishing Crates to Crates.io

[crates.io](https://crates.io/) is the official Rust package registry for sharing crates.

### Documentation Comments

Rust uses documentation comments to generate API documentation.

- `///` documents the item that follows it.
- `//!` documents the containing crate or module.
- `cargo doc --open` generates and opens HTML documentation.
- `cargo test` also runs Rust code examples in documentation comments as documentation tests.

Common documentation sections include `# Examples`, `# Panics`, `# Errors`, and `# Safety`.

### Re-exporting a Public API

The `pub use` keyword re-exports public items, allowing users to access them through more convenient paths without changing the internal module structure.

```rust
pub use self::kinds::PrimaryColor;
pub use self::utils::mix;
```

Users can then import these items directly from the crate root instead of navigating through nested modules.

### Publishing and Versioning

Before publishing, a crate needs a unique name and appropriate package metadata, including a description and license.

Useful Cargo commands:

- `cargo login` — Authenticate with crates.io using an API token.
- `cargo publish --dry-run` — Verify packaging without publishing.
- `cargo publish` — Publish a crate version.
- `cargo yank --vers 1.0.1` — Prevent new dependency resolutions from selecting a published version.
- `cargo yank --vers 1.0.1 --undo` — Reverse a yank.

Published crate versions cannot be overwritten. New releases require a version change following Semantic Versioning conventions.

Yanking does not delete published source code or break existing lockfiles that already reference the yanked version.

## Cargo Workspaces

A Cargo workspace manages multiple related packages together.

Workspace members share:

- A single `Cargo.lock` file.
- A common `target/` directory.
- Dependency resolution across packages.

A workspace root `Cargo.toml` can contain:

```toml
[workspace]
resolver = "3"
members = ["adder", "add_one", "add_two"]
```

Packages in the same workspace do not automatically depend on one another. Dependencies must be declared explicitly:

```toml
[dependencies]
add_one = { path = "../add_one" }
add_two = { path = "../add_two" }
```

Useful commands:

- `cargo build` — Build the workspace's default members.
- `cargo run -p adder` — Run a specific binary package.
- `cargo test --workspace` — Test all workspace members.
- `cargo test -p add_one` — Test a specific package.

Each workspace package must be published separately if it is intended for crates.io.

## Installing Binaries with Cargo

The `cargo install` command installs executable programs from Rust packages.

```bash
cargo install ripgrep
```

The `ripgrep` package installs an executable named `rg`.

By default, installed binaries are stored in `$HOME/.cargo/bin`, which must be included in `$PATH` to run them by name.

Unlike adding a dependency to `Cargo.toml`, `cargo install` installs a tool for local use.

## Extending Cargo with Custom Commands

Cargo supports external subcommands without modifying Cargo itself.

If an executable named `cargo-something` exists in `$PATH`, it can be invoked using:

```bash
cargo something
```

For example, an installed `cargo-audit` executable can be invoked with `cargo audit`.

Run `cargo --list` to see available Cargo commands, including detected custom subcommands.

This allows the Rust ecosystem to extend Cargo with additional developer tools.

## Key Takeaways

Cargo provides tools for optimizing builds, documenting libraries, designing public APIs, publishing crates, managing workspaces, installing executables, and extending development workflows.

These features help Rust projects remain modular, maintainable, and easy to share.