# Package Management

Cargo is Rust's package manager and build system — everything from creating a new project to publishing a crate.

## Prerequisites

- Basic Rust syntax
- A working Rust installation (rustup, cargo)

## Creating and Building

```bash
# Create a new binary crate
cargo new my-app
cd my-app

# Create a new library crate
cargo new my-lib --lib

# Build (debug)
cargo build

# Build (release)
cargo build --release

# Run
cargo run

# Check (fast, no binary output)
cargo check
```

## Dependencies

Add a dependency:

```bash
cargo add serde
cargo add tokio --features full
cargo add --dev rvtest
```

Or edit `Cargo.toml` directly:

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }

[dev-dependencies]
rvtest = "0.3"
```

## Semantic Versioning

Cargo follows semver:

| Requirement | Meaning |
|-------------|---------|
| `"1.2.3"` | Exactly 1.2.3 |
| `"^1.2.3"` | Compatible with 1.2.3 (≥1.2.3, <2.0.0) |
| `"~1.2.3"` | ≥1.2.3, <1.3.0 |
| `">=1.2.3"` | ≥1.2.3 |
| `"*"` | Any version (not recommended) |
| `"=1.2.3"` | Exactly 1.2.3 |

## Publishing

```bash
# Login to crates.io
cargo login

# Check before publishing
cargo package --list

# Publish
cargo publish

# Yank a bad version
cargo yank --vers 1.2.3
```

## Glossarium

| Term | Definition |
|------|------------|
| Semver | Semantic Versioning — `MAJOR.MINOR.PATCH` where MAJOR changes indicate breaking changes. |
| Cargo.lock | Lockfile that pins exact dependency versions for reproducible builds. |
| Workspace | A set of crates sharing a single `Cargo.lock` and output directory. |
| Registry | A package registry, by default crates.io. |

## Next Steps

- [Workspaces](workspaces.md) — managing multi-crate projects
- [Rust Book: Cargo](https://doc.rust-lang.org/cargo/)
