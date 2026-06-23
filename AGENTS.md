# AGENTS — Instructions for AI agents

This file provides context and instructions for AI coding assistants working on this monorepo.

## Project Vision

rvlibs is **not a collection of libraries without purpose**. It is an ecosystem with gravity — every crate, whether atomic or composite, has a concrete role and a clear destination: the **Rveco** application.

Rveco is the estuary. The main binary that unifies all crates into one creative development suite. Without Rveco, we have a pile of libraries waiting for a purpose. With Rveco, every crate knows where it is heading.

Architecture metaphor: **brain, body, estuary** on a foundation:

| Layer | Crate | Role |
|-------|-------|------|
| Brain | `rvnx` | Intelligence, logic, ECS, scene graph, port traits. May use external deps. |
| Body | `rvfx` | Physical implementation: rendering (wgpu), windowing (winit), editor UI. Depends on rvnx. |
| Estuary | `rveco` | Main binary. Binds brain + body into a real application. |
| Foundation | `rvlibs`, `rvmath` | Shared contracts, math. Zero external deps. |

**Not preemptive** — new crates are only created when a concrete need arises. Do not create `rvstat`, `rvphysic`, `rvui`, etc. speculatively.

## SDLC Skills

Skills follow the Software Development Life Cycle. Load the relevant skill for your current phase.

| Phase | Skill | Description |
|-------|-------|-------------|
| **Planning** | `requirements-gathering` | Elicit and document feature requirements |
| **Analysis** | `system-analysis` | Analyse feasibility, API sketch, compat |
| **Design** | `architecture-design` | Module placement, type design, API surface |
| **Implementation** | `feature-development` | Write code following conventions |
| | `code-review` | Review for correctness, style, security |
| **Testing** | `quality-assurance` | Test placement, coverage gates, verification |
| **Deployment** | `release-management` | Version bump, publish order, tagging |
| **Maintenance** | `issue-triage` | Classify, reproduce, resolve issues |
| | `code-refactoring` | Split large files (800+ lines) into sibling/submodules |
| **Docs** | `sync-docs` | Synchronise markdown docs, fix broken links, audit references |
| **Learning** | `grow-together` | Develop ecosystem-first learning content for docs/learn/ |

## Project structure

```
rvlibs/
├── Cargo.toml          # Workspace root
├── crates/             # Library crates
│   ├── rvlibs/         # Shared contracts only (zero deps)
│   ├── rvmath/         # Mathematics library (src/ + tests/)
│   ├── rvtest/         # Testing library (src/ + tests/)
│   ├── rvtest-macros/  # Proc-macros for rvtest
│   ├── rvnx/           # Brain — engine core, ECS, scene graph, ports
│   └── rvfx/           # Body — services, rendering, window, editor UI
├── apps/               # Application binaries
│   ├── cargo-rvtest/   # CLI binary for rvtest
│   └── rveco/          # Estuary — main application binary
├── docs/
│   ├── architecture.md # Crate architecture (brain/body/estuary)
│   ├── conventions.md  # Code conventions
│   ├── dep-graph.md    # Dependency graph & rules
│   ├── philosophy.md   # Design philosophy
│   ├── roadmap.md      # Development roadmap & feature proposals
│   ├── testing.md      # Testing policy
│   └── learn/          # Learning paths
├── .github/            # CI, issue templates, community files
└── .agents/skills/     # SDLC workflow skills
```

## Commands

```bash
# Build all crates (including apps)
cargo build --workspace

# Run all tests across workspace
cargo test --workspace

# Run tests for a specific crate
cargo test -p rvmath
cargo test -p rvtest
cargo test -p rvnx

# Check + lint
cargo check
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check

# Format code
cargo fmt
```

## Conventions

- **`rvlibs` crate is shared contracts only** — Must only contain shared types, traits, error types, and constants needed by multiple workspace crates. No implementation logic, business logic, or crate-specific functionality. Zero external dependencies.
- **No vendor lock-in** — Public types, traits, and functions must not carry vendor prefixes (`RvlibsFoo`, `RvlibsBar`). Disambiguate by module path, not by name. A type name must make sense even if the crate were renamed.
- **Brain defines ports, body implements** — rvfx depends on rvnx, never the other way. Ports (traits) live in rvnx. Implementations (adapters) live in rvfx.
- **Not preemptive** — New crates are only created when concrete code demands extraction. Do not create speculative crates.
- Edition 2024
- Follow existing patterns in the crate being edited
- Doc comments (`///`) on all public items
- Tests go in `tests/` directory at the crate level
- **Dogfooding is a top priority** — Every crate in this workspace must use rvtest's BDD API (`describe`/`it`) for its tests. Never use external test frameworks or raw `#[test]` for complex test scenarios. Every `describe` block **must** end with `.run().assert_all_pass()` to ensure failures are properly surfaced. This guarantees the entire ecosystem eats its own dogfood and bugs surface early.
