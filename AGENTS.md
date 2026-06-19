# AGENTS — Instructions for AI agents

This file provides context and instructions for AI coding assistants working on this monorepo.

## Project structure

```
rvlibs/
├── Cargo.toml          # Workspace root
├── crates/
│   ├── rvmath/         # Mathematics library (src/ + tests/)
│   ├── rvtest/         # Testing library (src/ + tests/)
│   ├── rvtest-macros/  # Proc-macros for rvtest
│   └── cargo-rvtest/   # CLI binary for rvtest
├── docs/
│   ├── rvmath/
│   └── rvtest/
└── .github/            # CI, issue templates, community files
```

## Commands

```bash
# Build all crates
cargo build

# Run all tests across workspace
cargo test --workspace

# Run tests for a specific crate
cargo test -p rvmath
cargo test -p rvtest

# Check + lint
cargo check
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check

# Format code
cargo fmt
```

## Conventions

- Edition 2024
- Follow existing patterns in the crate being edited
- Doc comments (`///`) on all public items
- Tests go in `tests/` directory at the crate level
- rvtest uses its own BDD API for integration tests (dogfooding)
