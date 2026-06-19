# Chapter 01 — Why Test?

[Index](00-index.md) • [Next →](02-rust-basics-for-test.md)

---

Testing is the practice of writing code that verifies other code behaves
correctly.  If you have ever written a `println!` to check whether a variable
has the value you expect, you have already done a manual test.  This chapter
explains why automating that process is worth the effort.

---

## The Cost of Not Testing

Consider a simple function:

```rust
fn divide(a: f64, b: f64) -> f64 {
    a / b
}
```

It looks correct.  But what happens when `b` is `0.0`?  In Rust, floating-point
division by zero returns `inf` instead of panicking — which might be wrong for
your application.  A test would catch this.

Without tests, you rely on:
- Manual `println!` debugging — slow and easily forgotten
- Code reviews — good but cannot catch every edge case
- Production bug reports — the most expensive way to find bugs

---

## What a Test Looks Like in Rust

A test in Rust is just a function annotated with `#[test]`:

```rust
#[test]
fn divide_by_zero_returns_inf() {
    let result = divide(1.0, 0.0);
    assert!(result.is_infinite(), "expected infinite, got {result}");
}
```

Run it with:

```bash
cargo test
```

If the assertion fails, the test fails, and you know immediately that
something is wrong.

---

## Kinds of Tests

### Unit Tests

Test a single function or module in isolation.  Fast, reliable, and easy to
write.  These make up the bulk of a good test suite.

```rust
#[test]
fn add_two_positive_numbers() {
    assert_eq!(add(2, 3), 5);
}
```

### Integration Tests

Test how multiple modules or external services work together.  Slower but
catch interface mismatches.

```rust
// tests/api_test.rs
#[test]
fn create_user_and_fetch() {
    let user = create_user("alice");
    let fetched = fetch_user(user.id);
    assert_eq!(user, fetched);
}
```

### Documentation Tests

Code examples in doc comments are tested by `cargo test`.  This keeps
documentation from going stale.

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

### Property-Based Tests

Instead of writing individual test cases, you describe an invariant that must
hold for all inputs, and the computer generates random inputs to verify it.

```rust
fn add_is_commutative(a: i32, b: i32) -> bool {
    add(a, b) == add(b, a)
}
```

We will explore these in [Chapter 12](12-property-based-testing.md).

---

## The Testing Trophy

Not all tests are created equal.  A good test suite has:

```
         ╱╲
        ╱  ╲       Static analysis (types, lints)
       ╱    ╲
      ╱──────╲     
     ╱        ╲    Unit tests (fast, many)
    ╱          ╲
   ╱────────────╲  Integration tests (fewer, slower)
  ╱              ╲
 ╱────────────────╲ End-to-end tests (few, slow)
```

- **Static analysis** (compiler, Clippy) catches type errors and common mistakes
- **Unit tests** catch logic errors in individual functions
- **Integration tests** catch interface mismatches between components
- **End-to-end tests** verify the full system works

Invest most of your effort in unit tests — they give the best return on
investment.

---

## What Makes a Good Test Suite?

| Property | Why It Matters |
|----------|---------------|
| **Fast** | Slow tests discourage running them |
| **Deterministic** | A test should pass or fail consistently |
| **Isolated** | Tests should not depend on each other |
| **Readable** | The test name and body should explain the expected behaviour |
| **Focused** | Each test should verify one behaviour |
| **Automated** | Tests should run without manual intervention |

---

## Summary

- Tests are code that verifies other code
- Rust has built-in support for unit, integration, and doc tests
- A good test suite is fast, deterministic, isolated, and focused
- Start with unit tests — they give the most value for the least effort
- `rvtest` extends Rust's built-in test harness with BDD specs, property
  testing, parametrized tests, rich reporting, and code coverage — all
  without proc-macro dependencies

In the next chapter, we will write our first Rust test and explore the testing
tools that Rust provides out of the box.

---

[Index](00-index.md) • [Next →](02-rust-basics-for-test.md)
