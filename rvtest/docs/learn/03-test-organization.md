# Chapter 03 — Test Organization

[← Previous](02-rust-basics-for-test.md) • [Index](00-index.md) • [Next →](04-writing-effective-tests.md)

---

As a project grows, where you put your tests and how you structure them
matters.  Rust gives you two main options: inline tests and integration tests.
This chapter explains both and when to use each.

---

## Inline Tests (Unit Tests)

The most common pattern is to put tests in a `tests` module inside the same
file as the code being tested:

```rust
// src/calculator.rs

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_positive_numbers() {
        assert_eq!(add(2, 3), 5);
    }
}
```

**Advantages:**
- Tests are next to the code — easy to find and update
- `use super::*;` gives access to private functions and structs
- The `#[cfg(test)]` attribute ensures tests are not compiled into the release
- Compilation is faster because there are fewer separate compilation units

**When to use:** Always.  Every module should have a `tests` module with unit
tests for its public (and private) API.

---

## Testing Private Functions

Unlike many languages, Rust allows tests to access private items:

```rust
fn internal_helper(x: i32) -> i32 {
    x * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_internal_helper() {
        assert_eq!(internal_helper(21), 42);
    }
}
```

This is intentional — testing internal functions directly often leads to more
thorough coverage than testing only through the public API.

However, if a private function is so complex that it needs extensive tests,
consider whether it should be extracted into its own module with a public API.

---

## Integration Tests

Integration tests go in the `tests/` directory at the project root.  Each file
in `tests/` is compiled as a separate crate.

```
my-project/
├── Cargo.toml
├── src/
│   └── lib.rs
└── tests/
    ├── integration_test.rs
    └── api_test.rs
```

```rust
// tests/integration_test.rs

// Your crate is imported as an external dependency
use my_project::add;

#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
}
```

**Advantages:**
- Tests use the crate's public API only — just like real users
- Catch API design issues early (if something is awkward to test, it is
  probably awkward to use)
- Each file is a separate crate, so there is no accidental sharing of state

**Limitations:**
- Can only test public API
- Slower to compile (each file is a separate crate)
- No access to private functions

---

## When to Use Each

| Scenario | Unit Test | Integration Test |
|----------|-----------|-----------------|
| Testing a single function | ✅ Best | ❌ Overkill |
| Testing private helper | ✅ Best | ❌ Not possible |
| Testing crate public API | ✅ Good | ✅ Also good |
| Testing multiple modules together | ❌ Not suitable | ✅ Best |
| Testing error paths in internal code | ✅ Best | ❌ May not be exposed |
| Catching API design issues | ❌ Not suitable | ✅ Best |

**General rule of thumb:** Write unit tests for every function and module.
Write integration tests for the main user-facing workflows.

---

## Organizing Unit Tests

### Per-Module Tests

Each module has its own `tests` submodule:

```rust
// src/parser.rs
pub fn parse(input: &str) -> i32 {
    input.trim().parse().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_number() {
        assert_eq!(parse("42"), 42);
    }

    #[test]
    fn parse_whitespace() {
        assert_eq!(parse("  42  "), 42);
    }

    #[test]
    fn parse_invalid_returns_zero() {
        assert_eq!(parse("abc"), 0);
    }
}
```

```rust
// src/formatter.rs
pub fn format(value: i32) -> String {
    format!("value: {value}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_includes_value() {
        assert_eq!(format(42), "value: 42");
    }
}
```

### Nested Test Modules

For larger modules, use nested test modules to group related tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    mod add_tests {
        use super::*;

        #[test]
        fn add_positive() { assert_eq!(add(2, 3), 5); }
        #[test]
        fn add_negative() { assert_eq!(add(-2, -3), -5); }
        #[test]
        fn add_zero() { assert_eq!(add(0, 5), 5); }
    }

    mod subtract_tests {
        use super::*;

        #[test]
        fn sub_positive() { assert_eq!(sub(5, 3), 2); }
        #[test]
        fn sub_negative_result() { assert_eq!(sub(3, 5), -2); }
    }
}
```

---

## Organizing Integration Tests

### Feature-Based Files

Name integration test files after the feature they test:

```
tests/
├── user_registration.rs
├── authentication.rs
├── data_export.rs
└── api_errors.rs
```

### Common Modules

If multiple integration tests need shared setup code, use a `common` module:

```rust
// tests/common/mod.rs
pub fn setup_test_db() -> Database {
    Database::new_in_memory()
}
```

```rust
// tests/user_registration.rs
mod common;

#[test]
fn register_new_user() {
    let db = common::setup_test_db();
    // ...
}
```

Files in `tests/common/` are not treated as test files — only `tests/*.rs`
files are.

---

## Doc Tests Location

Doc tests live inside documentation comments in your source code:

```rust
/// Adds two numbers.
///
/// # Examples
///
/// ```
/// use my_lib::add;
/// assert_eq!(add(2, 3), 5);
/// ```
///
/// # Edge Cases
///
/// ```
/// use my_lib::add;
/// assert_eq!(add(-1, 1), 0);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Run only doc tests:
```bash
cargo test --doc
```

Doc tests serve dual purpose: documentation and testing.  They are the first
thing new users see and the last thing that gets updated.  Keep them simple.

---

## The Standard Layout

A well-organised Rust project follows this convention:

```
my-project/
├── Cargo.toml
├── src/
│   ├── lib.rs            ← Crate root, public exports
│   ├── main.rs           ← Binary entry point (if applicable)
│   ├── parser.rs         ← Module with inline unit tests
│   ├── formatter.rs      ← Module with inline unit tests
│   └── database.rs       ← Module with inline unit tests
├── tests/
│   ├── integration.rs    ← Integration tests
│   └── common/
│       └── mod.rs        ← Shared test helpers
└── examples/
    └── basic_usage.rs    ← Runnable examples (tested via cargo test --examples)
```

---

> **rvtest:** Use `cargo rvtest --workspace` to run tests across all workspace members with a single command, aggregated into one report.

## Summary

- **Unit tests** (`#[cfg(test)] mod tests`) live alongside the code they test
- **Integration tests** (`tests/` directory) test the public API from the
  outside
- **Doc tests** keep code examples in documentation up to date
- Use unit tests for thorough coverage of every function
- Use integration tests for user-facing workflows
- Keep tests organised by feature or module

In the next chapter, we will learn how to write tests that are clear, focused,
and maintainable.

---

[← Previous](02-rust-basics-for-test.md) • [Index](00-index.md) • [Next →](04-writing-effective-tests.md)
