# Architecture

rvmath is organized as a layered crate within the rvlibs monorepo. The architecture follows a dependency hierarchy where each layer builds on the types and traits defined below it.

## Module Dependency Graph

```
           ┌──────────┐
           │  prelude  │
           └────┬─────┘
                │ re-exports
    ┌───────────┼───────────┐
    │           │           │
    ▼           ▼           ▼
┌────────┐ ┌────────┐ ┌──────────┐
│ algebra│ │calculus│ │ geometry │
│────────│ │────────│ │──────────│
│symbolic│ │analyt. │ │constants │
│  expr  │ │ & num. │ │ 2D & 3D │
│ system │ │calculus│ │ formulas │
└────────┘ └────────┘ └──────────┘
    ▲           ▲           ▲
    │           │           │
    └───────────┼───────────┘
                │
         ┌──────┴──────┐
         │     num     │
         │  ─────────  │
         │ Num<T>      │
         │ Numeric     │
         │ Percentage  │
         │ Set         │
         └──────┬──────┘
                │
    ┌───────────┼───────────┐
    │           │           │
    ▼           ▼           ▼
┌──────────────────────────────┐  ┌────────┐
│             la               │  │  unit  │
│  ──────────────────────────  │  │────────│
│  ┌────────┐ ┌────────┐       │  │type-safe│
│  │ vector │ │ matrix │       │  │units   │
│  │────────│ │────────│ tensor│  └────────┘
│  │ VecN   │ │ MatN   │───────┘      ▲
│  │ Vec2-4 │ │2×2-4×4 │              │
│  └────────┘ └────────┘              │
└──────────────────┬──────────────────┘
                   │
         ┌─────────┴──────────┐
         │       consts       │
         │   ────────────     │
         │ math & phys consts │
         └────────────────────┘
```

### Layer 0: Constants (`consts`)

Standalone module providing mathematical and physical constants. No dependencies on other rvmath modules.

### Layer 1: Foundation (`num`, `unit`, `la`)

- **`num`** — Core type system: `Num<T>` wrapper and `Numeric` trait. Everything else depends on this.
- **`unit`** — Type-safe dimensional analysis built on `Numeric`. Declares families and units via macros.
- **`la`** — Linear algebra parent module containing:
  - **`vector`** — `VecN<T, N>` with compile-time dimensions. Uses `Numeric` for math operations.
  - **`matrix`** — `MatN<T, R, C>` with fixed rows/columns. Depends on `vector` for row/column access.
  - **`tensor`** — `Tensor<T>` with dynamic shape. Generic over element type.

### Layer 2: Intermediate (`geometry`, `utils`)

- **`geometry`** — Shape formulas built on `Numeric`. Applies foundation types to geometric problems.
- **`utils`** — String expression parser and evaluator. Uses `Num<f64>` for results.

### Layer 3: Advanced (`algebra`, `calculus`)

- **`algebra`** — Symbolic expression system built on `Expr` enum. Uses `num` for evaluation.
- **`calculus`** — Analytical and numerical calculus. Generic over `Numeric`.

### Cross-Cutting: `prelude`

Re-exports the most commonly used types from all layers for convenience.

## Key Design Decisions

### Generic Numerics

All mathematical operations are defined on the `Numeric` trait, not on concrete types. This means geometry formulas, calculus functions, and vector operations all work with `f32`, `f64`, `i32`, `Num<f64>`, and any custom type implementing `Numeric`.

### Type-Level Dimensions

Units carry their dimension at the type level (`Unit<N, Meter>` vs `Unit<N, Kilometer>`), while powers are tracked at runtime. This gives compile-time safety for dimension mismatches while keeping power arithmetic flexible.

### Expression Trees vs Evaluation

The `algebra` module uses a recursive `Expr` enum for symbolic manipulation, while the `utils` module uses a flat token stream approach for fast evaluation. These serve different use cases — symbolic transformation vs numeric computation.

### Re-export Strategy

Top-level functions (like `simplify`, `derivative`, `evaluate`) are re-exported at the crate root for ergonomic access. Types are organized in modules following Rust convention.
