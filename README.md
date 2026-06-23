# rvlibs

[![CI](https://github.com/reasvyn/rvlibs/actions/workflows/ci.yml/badge.svg)](https://github.com/reasvyn/rvlibs/actions)
[![License](https://img.shields.io/crates/l/rvlibs.svg)](LICENSE)

**Modular · Atomic · Composite · Toward Rveco**

rvlibs is a modular Rust library ecosystem heading toward one goal: **Rveco** — a creative development suite.

Every crate (rvmath, rvtest, rvnx, rvfx) is an independent atomic or composite building block. Rveco is the estuary that unifies them into a complete application.

```
Atomic:      rvmath  rvtest  rv* ...
                  \     /
Composite:    rvnx (brain)  rvfx (body)
                    \          /
Estuary:          rveco (application)
```

---

## Crates & Apps

| Crate | Role | Description | Status |
|-------|------|-------------|--------|
| [**rvmath**](crates/rvmath/) | Foundation | Mathematics: numerics, LA, geometry, unit types | ✅ Active |
| [**rvtest**](crates/rvtest/) | Cross-cutting | BDD tests, property-based, coverage, mocking | ✅ Active |
| [**rvtest-macros**](crates/rvtest-macros/) | Cross-cutting | Proc-macro API for rvtest | ✅ Active |
| **rvlibs** | Foundation | Shared contracts, error, version, meta | ⏳ Internal |
| **rvnx** | **Brain** | Engine core: ECS, scene graph, ports (GpuPort, WindowPort, AssetPort) | 🔜 Planned |
| **rvfx** | **Body** | Services: rendering (wgpu), windowing (winit), asset I/O, editor UI | 🔜 Planned |

| App | Description | Status |
|-----|-------------|--------|
| [**cargo-rvtest**](apps/cargo-rvtest/) | CLI binary for rvtest | ✅ Active |
| **rveco** | Main application — ecosystem estuary | 🔜 Planned |

---

**🦀 New to Rust?** Dive into our [comprehensive learning paths](docs/learn/index.md) — ownership, async, testing, tooling, ecosystem, and more. No prior Rust experience required.

---

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
