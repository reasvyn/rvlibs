# Conventions

## Rust Edition and Toolchain

- **Edition:** 2024
- **Format:** `rustfmt` with default settings
- **Lints:** Zero warnings expected across all crates
- **MSRV:** 1.85+ (rvmath), 1.96+ (rvtest)

## Naming

| Category | Convention | rvmath Example | rvtest Example |
|----------|------------|----------------|----------------|
| Crates | Lowercase, hyphenated | `rvmath` | `rvtest`, `rvtest-macros` |
| Types | `PascalCase` | `Num`, `VecN`, `Percentage` | `TestSuite`, `RunnerConfig` |
| Traits | `PascalCase` | `Numeric`, `Signed`, `Meta` | `TestReporter` |
| Functions | `snake_case` | `simplify()`, `to_f64()` | `describe()`, `assert_all_pass()` |
| Methods on builders | Fluent, consume `self` | — | `.it()`, `.tag()`, `.run()` |
| Constants | `UPPER_SNAKE_CASE` | `PI`, `TAU`, `EULER_MASCHERONI` | — |
| Macros | `snake_case!` | `declare_family!`, `declare_units!` | — |
| Error messages | Sentence case, no trailing period | `division by zero` | `test panicked` |
| Doc comments | Sentence case, trailing period | `/// A 3D vector.` | `/// Create a new empty suite.` |

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
