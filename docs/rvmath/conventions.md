# Conventions

## Code Style

- Follow the standard Rust style guide (`rustfmt`). All code is formatted with the default configuration.
- Use `camelCase` only for acronyms that are pronounced as words (e.g., `to_f64`, not `to_f_64`).
- Generic type parameters use single uppercase letters: `T` for element type, `N` for numeric type, `U` for unit type.

## Naming

- **Types**: UpperCamelCase — `Num`, `VecN`, `Percentage`, `Unit`, `MatN`
- **Traits**: UpperCamelCase — `Numeric`, `Signed`, `Meta`, `Dimension`
- **Functions**: snake_case — `simplify()`, `evaluate()`, `polygon_area()`
- **Methods**: snake_case — `to_f64()`, `from_f64()`, `dot_units()`
- **Constants**: UPPER_SNAKE_CASE — `PI`, `TAU`, `EULER_MASCHERONI`
- **Enum variants**: UpperCamelCase — `Expr::Add`, `Token::Number`, `BinaryOp::Mul`
- **Macros**: snake_case with trailing `!` — `declare_family!`, `declare_units!`

## Module Structure

Each module follows a consistent structure:

```
mod.rs          — re-exports, top-level public API functions
sub_module.rs   — related functionality grouped into sub-modules
```

Sub-modules are re-exported via `pub use` in `mod.rs` so users can access items at the module level (e.g., `rvmath::algebra::simplify` rather than `rvmath::algebra::simplify::simplify`).

## Re-exports

- Frequently-used items are re-exported at the crate root in `lib.rs` for ergonomic access.
- The `prelude` module re-exports the most common types for `use rvmath::prelude::*`.
- Internal implementation details are kept private. Only the public API surface is re-exported.
- Wildcard re-exports (`pub use module::*`) are used sparingly — only when all public items are intended for direct use.

## Error Handling

- Functions that can fail return `Result<T, String>` with descriptive error messages.
- Use `String` rather than a custom error type to keep the API simple and avoid dependency overhead.
- Panics are reserved for programming errors (e.g., unit dimension mismatches, which indicate a bug in the caller's logic).
- Invalid mathematical input (division by zero, sqrt of negative) returns `NaN` rather than an error.

## Testing

- All new tests use [`rvtest`](https://crates.io/crates/rvtest) — write specs with `describe`/`it`/`run`/`assert_all_pass()`.
- Prefer integration tests in `crates/rvmath/tests/` directory.
- Inline tests (`#[cfg(test)]`) and doctests are permitted but should be used sparingly.
- Use descriptive spec names that describe the expected behavior (e.g., `"computes volume correctly"`).
- Edge cases (division by zero, overflow, domain errors) are explicitly tested.
- Floating-point comparisons use `assert!((a - b).abs() < EPSILON)` rather than `assert_eq!`.

## Documentation

- All documentation is written in full English.
- Public items have doc comments (`///`) explaining what they do.
- Module-level docs (`//!`) explain the module's purpose and key design decisions.
- Do not include runnable code examples in doc comments.
- Cross-references to other modules use relative markdown links in `.md` files and backtick syntax in doc comments.

## Serde Support

- Serialization is optional (behind the `serde` feature).
- When enabled, `Unit` and `VecN` derive `Serialize`/`Deserialize`.
- Custom serialization logic is avoided — standard derive is used wherever possible.

## Macro Patterns

- `declare_family!` and `declare_units!` use a declarative syntax that mirrors Rust's `struct` and `enum` declarations.
- Internal macros (like `impl_numeric!`, `impl_vec_alias!`) are private and used to reduce boilerplate for repetitive implementations.
- Macros generate no additional runtime cost — they expand to the equivalent hand-written code.
