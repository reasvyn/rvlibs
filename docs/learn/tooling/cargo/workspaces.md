# Workspaces

Grouping multiple crates in a single repository with shared dependencies and output.

## Prerequisites

- [Package Management](package-management.md) — cargo basics, dependencies

## Workspace Root

A workspace is defined by a root `Cargo.toml` with a `[workspace]` section:

```toml
[workspace]
members = [
    "crates/rvmath",
    "crates/rvtest",
    "crates/rvtest-macros",
    "crates/cargo-rvtest",
]
```

## Commands

```bash
# Build all members
cargo build

# Test all members
cargo test --workspace

# Test a single member
cargo test -p rvmath

# Run a binary from a specific member
cargo run --bin cargo-rvtest -- --help
```

## Shared Metadata

Workspace members can inherit package metadata from the root:

```toml
# Root Cargo.toml
[workspace.package]
version = "0.1.0"
edition = "2024"
authors = ["Reas Vyn <reasvyn@gmail.com>"]

# Member Cargo.toml
[package]
name = "rvmath"
version.workspace = true
edition.workspace = true
authors.workspace = true
```

## Glossarium

| Term | Definition |
|------|------------|
| Workspace | A group of crates sharing a `Cargo.lock` and `target/` directory. |
| `workspace.package` | A section in the root `Cargo.toml` for shared package metadata. |
| Path Dependency | A dependency that points to a local crate using `path = "../crate"`. |

## Next Steps

- [rustc Basics](../compiler/rustc-basics.md) — compiler flags and optimisation
- [Cargo Book: Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)
