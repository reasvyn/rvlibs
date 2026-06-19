# Dependency Graph

## Workspace Dependency Tree

```
                    ┌──────────────────────────────────────────────┐
                    │                  rvlibs                      │
                    │  Shared contracts: Error, Result, Version,  │
                    │  meta — zero external dependencies           │
                    └──────────┬────────────────┬────────┬────────┘
                               │                │        │
              ┌────────────────┼────────────────┼────────┼────────┐
              ▼                ▼                ▼        ▼        │
    ┌─────────────────┐ ┌──────────┐ ┌──────────────┐ ┌──────────┐│
    │     rvmath      │ │  rvtest  │ │rvtest-macros │ │cargo-    ││
    │  math library   │ │ testing  │ │  proc-macros │ │rvtest    ││
    └────────┬────────┘ │ library  │ └──────────────┘ │  CLI     ││
             │          └────┬─────┘        │         └────┬─────┘│
             │               │              │              │      │
             │       ┌───────┘              └──────────┐    │      │
             │       │         (optional)              │    │      │
             │       ▼                                 ▼    │      │
             │  ┌─────────────────────────────────────────┐ │      │
             │  │            rvtest-macros                 │ │      │
             │  │  (only when rvtest feature="macros")     │ │      │
             │  └─────────────────────────────────────────┘ │      │
             │                                              │      │
             └──────────────────────────┬───────────────────┘      │
                                        │ (dev-dep only)             │
                                        ▼                          │
                              ┌──────────────────┐                 │
                              │   rvtest-macros   │                 │
                              │  (dev-dependency) │                 │
                              └──────────────────┘                 │
                                                                   │
                              ┌────────────────────────────────────┘
                              │ (dev-dep only)
                              ▼
                     ┌──────────────────┐
                     │     rvtest        │
                     │  (crate metadata  │
                     │   + integration)  │
                     └──────────────────┘
```

## Dependency Rules

| Crate | Depends On | Via |
|-------|------------|-----|
| **rvlibs** | *(none)* | — |
| **rvmath** | `rvlibs` | direct |
| | `serde` (optional) | direct |
| | `rvtest` | dev-only |
| **rvtest** | `rvlibs` | direct |
| | `rvtest-macros` | optional (`macros` feature) |
| **rvtest-macros** | `rvlibs` | direct |
| | `rvtest` | dev-only |
| **cargo-rvtest** | `rvlibs` | direct |
| | `rvtest` | direct |

## Key Principles

1. **rvlibs is the root** — it has zero dependencies and sits at the bottom of every dependency chain. All ecosystem crates depend on `rvlibs`, never the other way.

2. **No circular deps** — The dependency graph is a DAG. `rvtest` → `rvtest-macros` (optional) and `rvtest-macros` → `rvtest` (dev-only) are managed: dev-dependencies do not create cycles in published packages.

3. **Dev-only edges never propagate** — `rvtest` as a dev-dep of `rvmath` and `rvtest-macros` means those crates can use rvtest for testing without creating a circular dependency.

4. **Shared contracts live in rvlibs** — Any type, trait, or constant that two or more ecosystem crates need should live in `rvlibs` to avoid coupling them directly.
