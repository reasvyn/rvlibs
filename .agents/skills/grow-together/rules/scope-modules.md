# Scope: Modules and Submodules

The current `docs/learn/` structure and guidelines for adding new modules.

## Current Modules

```
docs/learn/
├── index.md                          # Master index — references module indexes only
├── rust/
│   ├── index.md
│   ├── basics/          (planned)    # ownership, borrowing, lifetimes, traits, error-handling
│   ├── collections/     (planned)    # Vec, HashMap, String, VecDeque, BinaryHeap
│   ├── concurrency/     (planned)    # threads, Send/Sync, Arc, Mutex, channels, atomics
│   ├── advanced/        (planned)    # unsafe, macros, async, FFI, Pin, interior-mutability
│   └── ecosystem/       (planned)    # cargo, crates.io, tooling, community, std-lib-deep-dive
├── math/
│   ├── index.md
│   ├── foundations/     (planned)    # numeric-types, units, percentages
│   ├── algebra/         (planned)    # expressions, simplification, polynomials
│   ├── linear-algebra/  (planned)    # vectors, matrices, tensors, decompositions
│   ├── calculus/        (planned)    # derivatives, integrals, series, numerical-methods
│   └── geometry/        (planned)    # constants, 2d-shapes, 3d-shapes
├── tests/
│   ├── index.md
│   ├── basics/          (planned)    # why-test, test-organization, writing-tests, assertions
│   ├── patterns/        (planned)    # bdd-specs, parametrized, property-based, mocking, snapshots
│   ├── advanced/        (planned)    # architecture-tests, concurrent-code, benchmark
│   └── workflow/        (planned)    # ci-integration, flaky-tests, faster-feedback, coverage
├── tooling/             (planned)
│   ├── index.md
│   ├── cargo/           (planned)    # commands, workspaces, dependencies, profiles
│   ├── compiler/        (planned)    # rustc, lints, codegen, incremental
│   └── ide/             (planned)    # rust-analyzer, clippy, rustfmt, debugging
├── async/               (planned)
│   ├── index.md
│   ├── foundations/     (planned)    # futures, async-await, executors
│   └── runtimes/        (planned)    # tokio, async-std, smol, ecosystem-comparison
└── design/              (planned)
    ├── index.md
    ├── patterns/        (planned)    # builder, newtype, RAII, typestate, entry
    ├── api-design/      (planned)    # naming, ergonomics, backwards-compat
    └── error-handling/  (planned)    # Result, Option, custom-errors, thiserror, anyhow
```

## Adding a New Module

1. Create `docs/learn/{module}/index.md` with a table of submodules.
2. Update `docs/learn/index.md` to reference the new module.
3. Create at least one submodule with one document before announcing.
4. Follow `content-structure.md` for every document.

## Adding a New Submodule

1. Create `docs/learn/{module}/{submodule}/` directory.
2. Update the module's `index.md` to list the new submodule.
3. Create the first document — one is enough to open the submodule.

## Rules

- Each submodule must have at least one document before it is listed in the index.
- A module must have at least one submodule before it is listed in the master index.
- Mark planned modules with `(planned)` in documentation. Remove the marker when the first document is written.
- Do not create empty directories — only create a directory when you are ready to write its first document.
