# Lints and Diagnostics

Compiler warnings, errors, and clippy — keeping your code clean and idiomatic.

## Prerequisites

- [rustc Basics](rustc-basics.md) — compiler flags

## Lint Levels

```rust
// Allow a lint (suppress warning)
#[allow(unused_variables)]
fn example() {
    let x = 42;
}

// Deny a lint (treat warning as error)
#[deny(unsafe_code)]
fn safe_only() {
    // compiler error if unsafe is used
}

// Forbid a lint (cannot be overridden)
#[forbid(unsafe_code)]
```

## Clippy

```bash
# Run all lints
cargo clippy

# Deny warnings in CI
cargo clippy -- -D warnings

# Run specific lints
cargo clippy -- -W clippy::pedantic

# Suppress a clippy lint
#[allow(clippy::needless_return)]
fn example() {
    42
}
```

## Common Lints

| Lint | What It Detects |
|------|-----------------|
| `unused_variables` | Variables that are declared but never read |
| `unused_imports` | Imports that are never used |
| `dead_code` | Functions, structs, or enums that are never used |
| `unsafe_code` | Any use of `unsafe` blocks |
| `clippy::pedantic` | Strict style and correctness lints |
| `clippy::nursery` | Experimental lints still in development |

## Glossarium

| Term | Definition |
|------|------------|
| Lint | A tool that analyses source code for potential errors or style issues. |
| Clippy | Rust's built-in lint collection (`cargo clippy`). |
| `allow` | Suppresses a lint warning. |
| `deny` | Treats a lint warning as a compiler error. |

## Next Steps

- [rustfmt and clippy](../ide/rustfmt-clippy.md) — integrating lints into your workflow
- [Clippy Book](https://doc.rust-lang.org/clippy/)
