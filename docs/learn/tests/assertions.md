# Assertions

Standard and enhanced assertion macros — making failures tell you exactly what went wrong.

## Prerequisites

- [Why Test](why-test.md) — `#[test]`, `assert!`, the Three A's


## Standard Assertions

```rust
#[test]
fn standard() {
    assert!(true);                                    // boolean condition
    assert_eq!(42, 42);                               // equality
    assert_ne!("hello", "world");                     // inequality
    assert_eq!(2 + 2, 4, "math is broken");           // custom message
}
```

## rvtest Assertions

`rvtest` provides assertion macros with enhanced failure output — structural diffs, source locations, and descriptive messages:

```rust
use rvtest::{assert_eq, assert_ok, assert_err, assert_matches, assert_delta};

#[test]
fn enhanced() {
    // Diff output on failure — shows which fields differ
    rvtest::assert_eq!(vec![1, 2, 3], vec![1, 2, 4]);

    // Result assertions
    rvtest::assert_ok!(Ok::<_, String>(42));
    rvtest::assert_err!(Err::<(), _>("error"));

    // Pattern matching
    rvtest::assert_matches!(Some(1), Some(1));

    // Float comparisons with tolerance
    rvtest::assert_delta!(1.0, 1.0001, 0.001);
}
```

## When Assertions Fail

Standard `assert_eq!`:
```
thread 'standard' panicked at 'assertion failed: `(left == right)`
  left: `[1, 2, 3]`,
 right: `[1, 2, 4]`'
```

rvtest's `assert_eq!` adds structural diff for nested types, making it easy to spot which fields differ.

## Glossarium

| Term | Definition |
|------|------------|
| `assert_eq!` | Assert two values are equal. On failure, both values are printed. |
| `assert_ne!` | Assert two values are not equal. |
| `assert_ok!` | Assert a `Result` is `Ok`, returning the inner value. |
| `assert_err!` | Assert a `Result` is `Err`. |
| `assert_matches!` | Assert a value matches a pattern. |
| `assert_delta!` | Assert two floats are within a tolerance. |


## Next Steps

- [BDD Specs](bdd-specs.md) — organising tests with `describe`/`it`
- [Parametrized Tests](parametrized-tests.md) — running the same logic with multiple inputs
