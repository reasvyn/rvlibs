# AGENTS — Instructions for AI agents

This file provides context and instructions for AI coding assistants working on this monorepo.

## SDLC Skills

Skills follow the Software Development Life Cycle. Load the relevant skill
for your current phase.

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

## Project structure

```
rvlibs/
├── Cargo.toml          # Workspace root
├── crates/
│   ├── rvlibs/         # Shared contracts only
│   ├── rvmath/         # Mathematics library (src/ + tests/)
│   ├── rvtest/         # Testing library (src/ + tests/)
│   ├── rvtest-macros/  # Proc-macros for rvtest
│   └── cargo-rvtest/   # CLI binary for rvtest
├── docs/
│   ├── architecture.md    # Crate architecture
│   ├── conventions.md     # Code conventions
│   ├── philosophy.md      # Design philosophy
│   ├── roadmap.md         # Development roadmap
│   ├── testing.md         # Testing policy
│   └── learn/             # Learning paths
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

- **`rvlibs` crate is shared contracts only** — The `crates/rvlibs/` crate must only contain shared types, traits, error types, and constants that are needed by multiple workspace crates. It must not contain implementation logic, business logic, or crate-specific functionality. Zero external dependencies.
- **No vendor lock-in** — Public types, traits, and functions must not carry vendor prefixes (`RvlibsFoo`, `RvlibsBar`). Disambiguate by module path, not by name. A type name must make sense even if the crate were renamed.
- Edition 2024
- Follow existing patterns in the crate being edited
- Doc comments (`///`) on all public items
- Tests go in `tests/` directory at the crate level
- **Dogfooding is a top priority** — rvtest must use its own BDD API (`describe`/`it`) for all its integration tests. Never use external test frameworks or raw `#[test]` for complex test scenarios inside rvtest. This ensures rvtest eats its own dogfood and bugs surface early.
