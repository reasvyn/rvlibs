# Chapter 02 — Rust Basics for Testing

[← Previous](01-why-test.md) • [Index](00-index.md) • [Next →](03-test-organization.md)

---

Rust has first-class support for testing built directly into the language and
compiler.  You do not need any external libraries to write and run tests.  This
chapter covers the fundamentals.

---

## The `#[test]` Attribute

Any function annotated with `#[test]` becomes a test.  When you run
`cargo test`, the compiler builds a test runner that discovers and executes
every `#[test]` function in your project.

```rust
#[test]
fn one_plus_one_equals_two() {
    assert_eq!(1 + 1, 2);
}
```

Save this in `src/lib.rs` and run:

```bash
cargo test
```

You should see output similar to:

```
running 1 test
test one_plus_one_equals_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

## The `assert!` Macro

`assert!` takes a boolean expression and panics if it is `false`:

```rust
#[test]
fn truth() {
    assert!(true);          // passes
    assert!(1 == 1);        // passes
    assert!(1 == 2);        // panics — test fails
}
```

You can add a custom message:

```rust
#[test]
fn with_message() {
    let result = 42;
    assert!(
        result > 100,
        "expected result > 100, got {result}"   // shown on failure
    );
}
```

---

## `assert_eq!` and `assert_ne!`

`assert_eq!` compares two values for equality and prints a diff on failure:

```rust
#[test]
fn string_equality() {
    let greeting = format!("hello {}", "world");
    assert_eq!(greeting, "hello world");
}
```

`assert_ne!` asserts that two values are **not** equal:

```rust
#[test]
 fn not_equal() {
     assert_ne!(1, 2);
 }
```

Both require the values to implement `PartialEq` and `Debug`.  The `Debug`
trait is what enables the diff output on failure.

---

## Test Modules

Tests can be organised into modules.  The conventional pattern is:

```rust
// In src/lib.rs or src/calculator.rs

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

    #[test]
    fn add_negative_numbers() {
        assert_eq!(add(-2, -3), -5);
    }
}
```

Key points:
- `#[cfg(test)]` means this module is only compiled during testing — it is
  excluded from the release build
- `use super::*;` brings the parent module's items into scope
- Each `#[test]` function is a separate test case

---

## Running Tests

| Command | Behaviour |
|---------|-----------|
| `cargo test` | Run all tests |
| `cargo test -- test_name` | Run only tests containing `test_name` |
| `cargo test -- --nocapture` | Show stdout/stderr output |
| `cargo test -- --test-threads=1` | Run tests sequentially |
| `cargo test -- --ignored` | Run only ignored tests |

Examples:

```bash
# Run all tests
cargo test

# Run tests with "add" in their name
cargo test add

# Run tests in a specific module
cargo test tests::

# Run with verbose output
cargo test -- --nocapture
```

---

## Ignoring Tests

Use `#[ignore]` to skip a test by default:

```rust
#[test]
#[ignore]
fn expensive_test() {
    // This test is skipped unless you run with --ignored
}
```

Run ignored tests with:

```bash
cargo test -- --ignored
```

This is useful for tests that are slow or require external resources.

---

## Testing `Result<T, E>`

Tests can return `Result` instead of panicking:

```rust
#[test]
fn returns_result() -> Result<(), String> {
    if 1 + 1 == 2 {
        Ok(())
    } else {
        Err("math is broken".to_string())
    }
}
```

This is handy when the code you are testing already returns `Result`:

```rust
fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse()
}

#[test]
fn parse_valid_number() -> Result<(), std::num::ParseIntError> {
    assert_eq!(parse_number("42")?, 42);
    Ok(())
}
```

We will explore this pattern in depth in [Chapter 6](06-testing-errors.md).

---

## Doc Tests

Rust also tests code examples in documentation comments:

```rust
/// Adds two numbers.
///
/// ```
/// use my_crate::add;
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Run doc tests with:

```bash
cargo test --doc
```

Or they run automatically as part of `cargo test`.

---

## Common Pitfalls

### Tests That Pass for the Wrong Reason

```rust
#[test]
fn test_addition() {
    assert!(1 + 1 == 3);  // This fails — good
}
```

But what about:

```rust
#[test]
fn test_reverse() {
    let v = vec![1, 2, 3];
    let rev: Vec<_> = v.into_iter().rev().collect();
    // Missing assertion — test always passes!
}
```

Always include an assertion.  A test with no assertions is worse than no test
at all — it gives a false sense of security.

### Tests That Depend on Other Tests

```rust
static mut COUNTER: i32 = 0;

#[test]
fn increment() {
    unsafe { COUNTER += 1; }
    assert!(unsafe { COUNTER } == 1);  // Passes only if run first
}
```

Tests should be **isolated** — they must not share mutable state.

---

> **rvtest:** The `cargo rvtest` CLI (install via `cargo install cargo-rvtest`) enhances `cargo test` with formatted output (Pretty, TAP, JUnit, JSON, Compact, GitHub, Agent), code coverage, flaky detection, and watch/daemon modes — all without changing your `#[test]` functions.

## Summary

- `#[test]` makes any function a test
- `assert!`, `assert_eq!`, `assert_ne!` are the basic assertion macros
- `#[cfg(test)]` gates test modules so they are excluded from release builds
- `cargo test` discovers and runs all tests
- Tests can return `Result` or panic on failure
- Doc tests keep examples in documentation up to date

In the next chapter, we will look at how to organise tests across a project.

---

[← Previous](01-why-test.md) • [Index](00-index.md) • [Next →](03-test-organization.md)
