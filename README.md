# rvlibs

[![CI](https://github.com/reasvyn/rvlibs/actions/workflows/ci.yml/badge.svg)](https://github.com/reasvyn/rvlibs/actions)
[![License](https://img.shields.io/crates/l/rvlibs.svg)](LICENSE)

**Modular · Atomic · Composite**

rvlibs is a collection of modular Rust libraries designed to be small, focused, and composable — use what you need, nothing more.

---

**🦀 New to Rust?** Dive into our [comprehensive learning paths](docs/learn/index.md) — ownership, async, testing, tooling, ecosystem, and more. No prior Rust experience required.

---

## Crates

| Crate | Description | crates.io |
|-------|-------------|-----------|
| [**rvmath**](crates/rvmath/) | Comprehensive, lightweight, type-safe mathematics library | [![crates.io](https://img.shields.io/crates/v/rvmath.svg)](https://crates.io/crates/rvmath) |
| [**rvtest**](crates/rvtest/) | Next Level Testing Library — BDD specs, property tests, coverage | [![crates.io](https://img.shields.io/crates/v/rvtest.svg)](https://crates.io/crates/rvtest) |
| [**rvtest-macros**](crates/rvtest-macros/) | Proc-macro API for rvtest | [![crates.io](https://img.shields.io/crates/v/rvtest-macros.svg)](https://crates.io/crates/rvtest-macros) |
| [**cargo-rvtest**](crates/cargo-rvtest/) | CLI binary for rvtest | [![crates.io](https://img.shields.io/crates/v/cargo-rvtest.svg)](https://crates.io/crates/cargo-rvtest) |

## Quick Start

```bash
git clone https://github.com/reasvyn/rvlibs.git
cd rvlibs

# Build everything
cargo build

# Run all tests
cargo test --workspace

# Run a specific crate
cargo test -p rvmath
cargo test -p rvtest
```

## Documentation

| Topic | Docs |
|-------|------|
| architecture | [docs/architecture.md](docs/architecture.md) |
| conventions | [docs/conventions.md](docs/conventions.md) |
| philosophy | [docs/philosophy.md](docs/philosophy.md) |
| roadmap | [docs/roadmap.md](docs/roadmap.md) |
| testing | [docs/testing.md](docs/testing.md) |
| learning paths | [docs/learn/index.md](docs/learn/index.md) |

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

## License

Dual-licensed under [MIT](LICENSE) or [Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0).
