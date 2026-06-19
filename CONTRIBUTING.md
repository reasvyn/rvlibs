# Contributing to rvlibs

First off, thank you for considering contributing! Every issue, pull request, and discussion makes this project better.

---

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How to Contribute](#how-to-contribute)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Pull Request Process](#pull-request-process)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Documentation](#documentation)

---

## Code of Conduct

This project is governed by the [Contributor Covenant](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

---

## How to Contribute

### Report a Bug

Open an issue at https://github.com/reasvyn/rvlibs/issues/new. Include:

- Crate name and version (check `Cargo.toml`)
- Rust version (`rustc --version`)
- A minimal reproduction
- Expected vs actual behaviour

### Suggest a Feature

Check the relevant roadmap first — your idea might already be planned:
- [rvmath roadmap](docs/rvmath/roadmap.md)
- [rvtest roadmap](docs/rvtest/roadmap.md)

If not, open an issue with:
- A clear description of the problem
- How you envision the solution
- Any alternative approaches you considered

### Submit a Pull Request

1. Fork the repo
2. Create a feature branch (`git checkout -b feat/my-feature`)
3. Make your changes
4. Run the tests (`cargo test --workspace`)
5. Ensure zero warnings (`cargo check`)
6. Submit a PR against `main`

---

## Development Setup

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/rvlibs.git
cd rvlibs

# Build everything
cargo build

# Run all tests
cargo test --workspace

# Run a specific crate's tests
cargo test -p rvmath
cargo test -p rvtest

# Run the rvtest CLI
cargo run --bin cargo-rvtest -- -v

# Run coverage
cargo run --bin cargo-rvtest -- --coverage
```

### Prerequisites

- **Rust 1.85+** for rvmath
- **Rust 1.96+** for rvtest (edition 2024)

---

## Project Structure

```
rvlibs/
├── Cargo.toml              # Workspace root
├── crates/
│   ├── rvmath/             # Mathematics library
│   │   ├── src/            # Source code
│   │   └── tests/          # Integration tests
│   ├── rvtest/             # Testing library
│   │   ├── src/            # Source code
│   │   └── tests/          # Integration tests
│   ├── rvtest-macros/      # Proc-macro API for rvtest
│   │   ├── src/
│   │   └── tests/
│   └── cargo-rvtest/       # CLI binary for rvtest
│       └── src/
├── docs/
│   ├── rvmath/             # rvmath documentation
│   ├── rvtest/             # rvtest documentation
│   └── learn/              # Learning paths (math, rust, tests)
└── .github/                # CI, issue templates, community files
```

---

## Pull Request Process

1. **One feature per PR.** If you have multiple unrelated changes, submit separate PRs.
2. **Keep PRs small.** A focused PR is easier to review and merge. Aim for < 400 lines changed.
3. **Write tests.** New features should include integration tests. For rvtest, dogfooding is mandatory — use `describe`/`it` for all complex test scenarios.
4. **Update docs.** If you change the public API or add a feature, update the relevant doc comments and documentation files.
5. **Pass CI.** Ensure `cargo check` and `cargo test --workspace` pass with zero warnings.
6. **Sign-off.** Your commits should include a `Signed-off-by` line (`git commit -s`) to certify that you wrote the code or have the right to contribute it.

---

## Coding Standards

See the per-crate conventions docs:
- [rvmath conventions](docs/rvmath/conventions.md)
- [rvtest conventions](docs/rvtest/conventions.md)

Key points:
- **Edition 2024**, fmt with defaults
- **Zero warnings** — `cargo check` must be clean
- **No `unsafe`** unless absolutely necessary and documented
- **Doc comments** on all public items
- **Use existing patterns** — look at similar code before writing new

---

## Testing

```bash
# Full test suite
cargo test --workspace

# rvmath tests
cargo test -p rvmath

# rvtest library tests
cargo test -p rvtest --lib

# rvtest CLI tests
cargo test -p rvtest --bin cargo-rvtest

# Specific test
cargo test -p rvtest rvtest_spec

# Run the CLI
cargo run --bin cargo-rvtest -- -v

# Coverage
cargo run --bin cargo-rvtest -- --coverage
```

All rvtest integration tests use its own BDD API (dogfooding).

---

## Documentation

- Public API items must have doc comments (`///`)
- Module-level docs (`//!`) describe the module's purpose and provide usage examples
- Documentation files in `docs/` should follow the same style as existing files

---

## Questions?

Open a [discussion](https://github.com/reasvyn/rvlibs/discussions) or ask in the issue tracker.
