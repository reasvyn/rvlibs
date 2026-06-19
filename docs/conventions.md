# Conventions

## Rust Edition and Toolchain

- **Edition:** 2024
- **Format:** `rustfmt` with default settings
- **Lints:** Zero warnings expected across all crates
- **MSRV:** 1.85+ (rvmath), 1.96+ (rvtest)

## Naming

### Case Convention

| Category | Convention | rvmath Example | rvtest Example |
|----------|------------|----------------|----------------|
| Crates | Lowercase, hyphenated | `rvmath` | `rvtest`, `rvtest-macros` |
| Modules | `snake_case` | `num`, `linear_algebra` | `property`, `coverage_raw` |
| Types | `PascalCase` | `Num`, `VecN`, `Percentage` | `TestSuite`, `RunnerConfig` |
| Traits | `PascalCase` | `Numeric`, `Signed`, `Meta` | `TestReporter` |
| Functions | `snake_case` | `simplify()`, `to_f64()` | `describe()`, `assert_all_pass()` |
| Methods on builders | Fluent, consume `self` | — | `.it()`, `.tag()`, `.run()` |
| Enum variants | `PascalCase` | `Expr::Add`, `BinaryOp::Mul` | `TestStatus::Passed` |
| Constants | `UPPER_SNAKE_CASE` | `PI`, `TAU`, `EULER_MASCHERONI` | — |
| Statics | `UPPER_SNAKE_CASE` | — | — |
| Macros | `snake_case!` | `declare_family!`, `declare_units!` | — |
| Feature flags | `snake_case` | `serde` | `macros` |
| Type parameters | Single uppercase | `T`, `N`, `U` | — |
| Lifetimes | `'a`, `'b`, etc. | — | — |
| Error messages | Sentence case, no trailing period | `division by zero` | `test panicked` |
| Doc comments | Sentence case, trailing period | `/// A 3D vector.` | `/// Create a new empty suite.` |

### General Rules

- **Clarity over brevity** — Names should be unambiguous even if slightly longer. Prefer `compute_volume` over `calc_v`.
- **Avoid abbreviations** — Use `configuration` not `cfg`, `evaluate` not `eval`, `derivative` not `deriv`.
- **Consistent verb forms** — Use imperative for constructors (`new`, `from`, `parse`), past tense for conversions (`to_f64`, `as_bytes`).
- **Acronyms** — Treat as normal words: `to_json` not `to_JSON`, `parse_html` not `parse_HTML`. Exception: single-letter acronyms (`n_tcp`, `set_dna`).
- **Get/Set omitted** — Rust does not use `get_`/`set_` prefixes. Use `name()` and `set_name()`.
- **Conversions** — Use `from_` (self → other), `to_` (other → self with copy), `into_` (other → self with move), `as_` (borrowed conversion).
- **Iterator methods** — Use `iter()`, `iter_mut()`, `into_iter()` following std conventions.
- **Error types** — Module-specific errors get `Error` suffix: `ParseError`, `ConfigError`. Shared/public errors use bare `Error` and are disambiguated by path: `rvlibs::Error`. Never prefix with vendor/crate name (`RvlibsParseError`).
- **Builder types** — The build method is `.build()`, the finalisation method is `.finish()`.

### No Vendor Lock-In

- Types, traits, and functions must not carry vendor prefixes or suffixes (`RvlibsFoo`, `RvlibsBar`).
- Disambiguate by module path: `rvlibs::Error`, not `RvlibsError`.
- Crate name prefixes on public types create coupling to the project name. Use plain, descriptive names that would make sense even if the crate were renamed.
- Exception: binary/CLI crate names may use the project prefix for disambiguation (`cargo-rvtest`).

### Test Naming

- Test functions use `snake_case` with descriptive names: `add_returns_sum`, `divide_by_zero_panics`.
- rvtest integration tests use `rvtest_` prefix: `rvtest_spec`, `rvtest_property`.
- Test modules mirror the module structure: `mod add_tests`, `mod subtract_tests`.
- Speak behaviour, not implementation: `returns_not_found` not `test_error_case_1`.

### File and Directory Naming

- Source files: `snake_case.rs` — `algebra.rs`, `coverage_raw.rs`.
- Directories: `snake_case/` for module directories.
- Test files: `snake_case.rs` in `tests/` — `num_basic.rs`, `matrix_basic.rs`.
- Doc files: `snake_case.md` — `architecture.md`, `dep-graph.md`.

## Error Handling

- Functions that can fail return `Result<T, String>` with descriptive error messages.
- Panics are reserved for programming errors (unit dimension mismatches, invariant violations).
- Invalid mathematical input (division by zero, sqrt of negative) returns `NaN` rather than an error.
- For internal invariants that should never fail, use `expect()` with a message explaining *why*.

## Documentation

- All documentation is written in full English.
- Public items have doc comments (`///`) explaining what they do.
- Module-level docs (`//!`) explain the module's purpose and key design decisions.
- Cross-references use backtick syntax in doc comments and relative markdown links in `.md` files.

## Testing

- Tests go in the `tests/` directory at the crate level (integration tests).
- Inline unit tests (`#[cfg(test)]`) and doctests are permitted but should be used sparingly.
- Floating-point comparisons use `assert!((a - b).abs() < EPSILON)` rather than `assert_eq!`.
- **Dogfooding is mandatory for rvtest** — all complex test scenarios use `describe`/`it`/`run`/`assert_all_pass()`.
- Use descriptive spec names that describe the expected behaviour (e.g., `"computes volume correctly"`).

## Module Structure

Each module follows a consistent structure:
```
mod.rs          — re-exports, top-level public API functions
sub_module.rs   — related functionality grouped into sub-modules
```

Sub-modules are re-exported via `pub use` in `mod.rs` for ergonomic access. Internal implementation details are kept private.

## Dependencies

- **rvmath** has a single optional dependency (`serde` for serialization).
- **rvtest** has minimal dependencies; the optional `macros` feature adds `rvtest-macros` (proc-macros).
- External dependencies are kept to a minimum to maintain fast compile times and avoid version conflicts.

## Serde Support (rvmath)

Serialization is optional (behind the `serde` feature). When enabled, `Unit` and `VecN` derive `Serialize`/`Deserialize`. Custom serialization logic is avoided — standard derive is used wherever possible.
