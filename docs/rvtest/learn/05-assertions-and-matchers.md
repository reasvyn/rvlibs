# Chapter 05 — Assertions and Matchers

[← Previous](04-writing-effective-tests.md) • [Index](00-index.md) • [Next →](06-testing-errors.md)

---

Assertions are the heart of every test.  They are how you communicate what
correct behaviour looks like.  This chapter covers the assertion tools Rust
provides, their limitations, and how `rvtest` improves upon them.

---

## Standard Assertions: What Rust Gives You

Rust's standard library provides three assertion macros:

```rust
assert!(condition);              // Panics if condition is false
assert_eq!(left, right);          // Panics if left != right
assert_ne!(left, right);          // Panics if left == right
```

Each accepts an optional custom message:

```rust
assert!(value > 0, "value must be positive, got {value}");
assert_eq!(result, expected, "test failed for input {input}");
```

---

## The Problem with Standard Assertions

When a standard `assert_eq!` fails, the output is often hard to read:

```
thread 'tests::test_example' panicked at src/lib.rs:10:5:
assertion `left == right` failed
  left: "hello\nworld\nfoo"
  right: "hello\nworld\nbar"
```

For complex types, this becomes even worse.  Consider a struct with many
fields — the output is a single line of Debug output that is impossible to
parse visually.

---

## Introducing `rvtest::assert_eq!`

`rvtest` provides an `assert_eq!` macro that produces structured diff output
when a comparison fails:

```rust
use rvtest::assert_eq as assert_eq;

#[test]
fn compare_strings() {
    let actual = format!("{} {}", "hello", "world");
    let expected = "hello world";
    rvtest::assert_eq!(actual, expected);
}
```

On failure, it shows a side-by-side diff of the two values, making it
immediately obvious what differs.

---

## Other `rvtest` Assertion Macros

```rust
use rvtest::{assert_ok, assert_err, assert_matches, assert_delta};
```

### `assert_ok!`

Unwraps a `Result` and returns the inner value, or panics with a formatted
error message:

```rust
#[test]
fn parse_succeeds() {
    let result: Result<i32, _> = "42".parse();
    let value = rvtest::assert_ok!(result);
    assert_eq!(value, 42);
}
```

This is cleaner than `result.unwrap()` because the failure message includes
the actual error:

```
assertion failed: expected Ok, got Err(ParseIntError { kind: InvalidDigit })
```

### `assert_err!`

The inverse — asserts that a `Result` is `Err`:

```rust
#[test]
 fn parse_fails_for_invalid_input() {
     let result: Result<i32, _> = "not a number".parse();
     rvtest::assert_err!(result);
 }
```

### `assert_matches!`

Asserts that a value matches a pattern:

```rust
#[test]
fn option_is_some() {
    let value = Some(42);
    rvtest::assert_matches!(value, Some(42));
}
```

When the pattern does not match, the output shows both the expected pattern
and the actual value:

```
assertion failed: expected `Some(42)`, got `Some(43)`
```

### `assert_delta!`

Compares floating-point values within an epsilon:

```rust
#[test]
fn float_approximation() {
    let actual = 0.1 + 0.2;
    let expected = 0.3;
    rvtest::assert_delta!(actual, expected, 0.0001);
}
```

Floating-point arithmetic is inherently imprecise.  Never compare floats with
`assert_eq!` — use `assert_delta!` instead.

---

## Custom Failure Messages

All `rvtest` assertion macros support custom messages:

```rust
rvtest::assert_eq!(
    result,
    expected,
    "test case {input} failed: expected {expected}, got {result}"
);
```

The custom message is appended to the default diff output.

---

## When to Use Which

| Situation | Macro |
|-----------|-------|
| Two values should be equal | `rvtest::assert_eq!` |
| Two values should differ | `assert_ne!` (standard is fine) |
| A `Result` should be `Ok` | `rvtest::assert_ok!` |
| A `Result` should be `Err` | `rvtest::assert_err!` |
| A value matches a pattern | `rvtest::assert_matches!` |
| Floats are approximately equal | `rvtest::assert_delta!` |
| A boolean condition | `assert!` (standard is fine) |

---

## Testing Equality for Custom Types

To use any `assert_eq!` with your own types, they must implement:

- **`PartialEq`** — so Rust knows how to compare two values
- **`Debug`** — so the assertion can display the values on failure

```rust
#[derive(Debug, PartialEq)]
struct User {
    name: String,
    age: u8,
}

#[test]
 fn user_equality() {
     let alice = User { name: "Alice".into(), age: 30 };
     let bob = User { name: "Bob".into(), age: 25 };
     assert_eq!(alice, bob); // Compiles because of PartialEq + Debug
 }
```

We will explore this in detail in [Chapter 7](07-testing-structs-and-enums.md).

---

## Common Mistakes

### Comparing Floats Directly

```rust
// ❌ May fail due to floating-point precision
assert_eq!(0.1 + 0.2, 0.3);
```

```rust
// ✅ Use delta comparison
rvtest::assert_delta!(0.1 + 0.2, 0.3, 0.0001);
```

### Forgetting to Assert

```rust
#[test]
 fn parse_test() {
     let result = "42".parse::<i32>();
     // Missing assertion!  Test passes even if parse() panics or returns wrong value
 }
```

### Over-Asserting

```rust
// ❌ This test is fragile — it breaks when unrelated details change
#[test]
 fn test_user_display() {
     let user = User::new("Alice", 30);
     let output = format!("{user}");
     assert_eq!(output, "User { name: \"Alice\", age: 30 }");
 }
```

---

## Summary

- Standard `assert_eq!` works but produces hard-to-read output for complex types
- `rvtest::assert_eq!` adds structured diff output
- `assert_ok!`, `assert_err!`, `assert_matches!`, `assert_delta!` cover common
  assertion patterns
- Always use `assert_delta!` for floating-point comparisons
- Custom types need `PartialEq` + `Debug` for assertion macros

---

[← Previous](04-writing-effective-tests.md) • [Index](00-index.md) • [Next →](06-testing-errors.md)
