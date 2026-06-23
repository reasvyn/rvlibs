# Dependency Graph

## Workspace Dependency Tree

```
                    rvlibs
                        │
            ┌───────────┼───────────┐
            │           │           │
            ▼           ▼           ▼
         rvmath     rvtest    rvtest-macros
            │           │
            │      ┌────┴────┐
            │      │         │
            │      │    cargo-rvtest (apps/)
            │      │         │
            └──────┼─────────┘
                   │
                   ▼
                 rvnx (brain)
                    │
               ┌────┴────┐
               │         │
               ▼         ▼
             rvfx     rveco (apps/)
            (body)   (estuary)
              │
         ┌────┴────┐
         │         │
       wgpu     winit
       naga     ...
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
| **rvnx** | `rvlibs` | direct |
| | `rvmath` | direct |
| | *(external crates as needed)* | direct |
| | `rvtest` | dev-only |
| **rvfx** | `rvnx` | direct |
| | `wgpu`, `winit`, `naga`, ... | direct |
| | `rvtest` | dev-only |
| **rveco** | `rvnx` | direct |
| | `rvfx` | direct |
| | `rvtest` | dev-only |

## Key Principles

1. **rvlibs is the root** — Zero dependencies. All ecosystem crates depend on `rvlibs`, never the other way.

2. **rvnx (brain) defines, rvfx (body) implements** — rvfx depends on rvnx to implement its port traits. Never the other way around.

3. **rveco is the estuary** — Depends on rvnx and rvfx, unifying both into a single application.

4. **No circular deps** — The dependency graph is a DAG. `rvtest` → `rvtest-macros` (optional) and `rvtest-macros` → `rvtest` (dev-only) are managed via dev-dependencies.

5. **Dev-only edges never propagate** — `rvtest` as a dev-dep does not create circular dependencies.

6. **Shared contracts live in rvlibs** — Any trait, type, or constant needed by >= 2 ecosystem crates must go in `rvlibs`.
