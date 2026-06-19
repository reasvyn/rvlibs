# Chapter 06 — Testing Errors

[← Previous](05-assertions-and-matchers.md) • [Index](00-index.md) • [Next →](07-testing-structs-and-enums.md)

---

Errors are a fact of life in software.  A well-tested codebase verifies not
only that code works when it should, but also that it fails correctly when it
should.  This chapter covers every technique Rust offers for testing error
paths.

---

## The Two Error Models in Rust

Rust has two ways to represent failure:

| Mechanism | Use Case |
|-----------|----------|
| **Panic** | Programming bugs: index out of bounds, division by zero, unwrap on `None` |
| **`Result<T, E>`** | Recoverable errors: file not found, invalid input, network timeout |

Your testing strategy must handle both.

---

## Testing for Panics with `#[should_panic]`

When a function is supposed to panic, annotate the test with
`#[should_panic]`:

```rust
/// Returns the element at index, panics if index is out of bounds.
fn first_element<T>(v: &[T]) -> &T {
    &v[0]
}

#[test]
#[should_panic]
fn first_element_of_empty_slice_panics() {
    let empty: &[i32] = &[];
    first_element(empty);
}
```

The test passes only if the code inside it panics.

---

## Testing Specific Panic Messages

A bare `#[should_panic]]` passes if *any* panic occurs anywhere in the test.
Use `expected` to be more precise:

```rust
#[test]
#[should_panic(expected = "index out of bounds")]
fn first_element_of_empty_slice_panics_with_message() {
    let empty: &[i32] = &[];
    first_element(empty);
}
```

The expected string is a substring match — the panic message just needs to
*contain* it.  This prevents false positives from unrelated panics.

---

## When `#[should_panic]]` Is Not Enough

Sometimes you need to verify something *after* the panic, or you need to
assert on the panic value itself.  In those cases, use `std::panic::catch_unwind`:

```rust
use std::panic::catch_unwind;

#[test]
fn catch_panic_and_inspect_message() {
    let result = catch_unwind(|| {
        let empty: &[i32] = &[];
        first_element(empty);
    });

    assert!(result.is_err(), "expected a panic");

    // You can inspect the panic message
    let panic_msg = result.unwrap_err();
    let msg = panic_msg.downcast_ref::<&str>().unwrap();
    assert!(msg.contains("index out of bounds"), "unexpected message: {msg}");
}
```

This pattern is useful when you need to assert on the panic message exactly,
or when you want to test that a specific operation panics while other
operations in the same test do not.

---

## Testing `Result` Types

For functions that return `Result`, write tests that verify both the `Ok` and
`Err` paths:

```rust
fn divide(numerator: i32, denominator: i32) -> Result<i32, String> {
    if denominator == 0 {
        Err("division by zero".to_string())
    } else {
        Ok(numerator / denominator)
    }
}

#[test]
 fn divide_success() {
     let result = divide(10, 2);
     assert!(result.is_ok());
     assert_eq!(result.unwrap(), 5);
 }

 #[test]
 fn divide_by_zero() {
     let result = divide(10, 0);
     assert!(result.is_err());
     assert_eq!(result.unwrap_err(), "division by zero");
 }
```

---

## Tests That Return `Result`

A test function itself can return `Result`.  This is the cleanest way to test
`Result`-returning code:

```rust
#[test]
 fn divide_success() -> Result<(), String> {
     let result = divide(10, 2)?;
     assert_eq!(result, 5);
     Ok(())
 }

 #[test]
 fn divide_by_zero() -> Result<(), String> {
     let result = divide(10, 0);
     assert!(result.is_err());
     Ok(())
 }
```

When a test returns `Result`, the `?` operator propagates errors — if
`divide` returns `Err`, the test fails with that error.  No need for
`unwrap()`.

---

## Testing Multiple Error Variants

When a function returns different error variants for different conditions,
test each variant:

```rust
enum ValidationError {
    TooShort { min: usize, actual: usize },
    ContainsInvalidChar(char),
    Empty,
}

fn validate_username(name: &str) -> Result<&str, ValidationError> {
    if name.is_empty() {
        return Err(ValidationError::Empty);
    }
    if name.len() < 3 {
        return Err(ValidationError::TooShort { min: 3, actual: name.len() });
    }
    for c in name.chars() {
        if !c.is_alphanumeric() {
            return Err(ValidationError::ContainsInvalidChar(c));
        }
    }
    Ok(name)
}

#[test]
 fn rejects_empty_username() {
     let result = validate_username("");
     assert!(matches!(result, Err(ValidationError::Empty)));
 }

 #[test]
 fn rejects_short_username() {
     let result = validate_username("ab");
     assert!(matches!(result, Err(ValidationError::TooShort { min: 3, .. })));
 }

 #[test]
 fn rejects_invalid_characters() {
     let result = validate_username("user name");
     assert!(matches!(result, Err(ValidationError::ContainsInvalidChar(' '))));
 }

 #[test]
 fn accepts_valid_username() {
     assert!(validate_username("alice").is_ok());
 }
```

---

## Testing Error Messages

When errors implement `Display`, test the formatted message:

```rust
impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::Empty => write!(f, "username cannot be empty"),
            ValidationError::TooShort { min, actual } => {
                write!(f, "username too short: minimum {min} characters, got {actual}")
            }
            ValidationError::ContainsInvalidChar(c) => {
                write!(f, "username contains invalid character: '{c}'")
            }
        }
    }
}

#[test]
 fn error_message_for_short_username() {
     let err = validate_username("ab").unwrap_err();
     assert_eq!(
         err.to_string(),
         "username too short: minimum 3 characters, got 2"
     );
 }
```

Error messages are part of your public API.  If other code parses your error
messages, changing them will break that code.  Test them.

---

## Testing `Option` and `unwrap`

`None` values from `Option` are similar to `Err` from `Result`:

```rust
fn find_user(id: u32) -> Option<String> {
    match id {
        1 => Some("Alice".to_string()),
        2 => Some("Bob".to_string()),
        _ => None,
    }
}

#[test]
 fn finds_existing_user() {
     assert_eq!(find_user(1), Some("Alice".into()));
 }

 #[test]
 fn returns_none_for_missing_user() {
     assert_eq!(find_user(99), None);
 }
```

---

## Testing Invariants Under Error Conditions

Sometimes you need to verify that your system remains in a consistent state
even after an error:

```rust
#[test]
 fn failed_operation_does_not_corrupt_state() {
     let mut system = System::new();

     // Perform an operation that will fail
     let result = system.process_invalid_data();

     assert!(result.is_err(), "expected failure");

     // Verify the system is still usable
     assert!(system.is_healthy());
     assert!(system.process_valid_data().is_ok());
 }
```

---

> **rvtest:** The `rvtest::assert_err!` and `rvtest::assert_matches!` macros provide concise, readable assertions for error handling and pattern matching in tests.

## Summary

| Technique | When to Use |
|-----------|-------------|
| `#[should_panic]` | A function should panic |
| `#[should_panic(expected = "...")]` | A function should panic with a specific message |
| `catch_unwind` | Need to inspect the panic value or continue after panic |
| `Result<T, E>` in tests | Testing `Result`-returning code cleanly |
| `matches!()` | Testing specific enum variants |
| Error message assertions | Error messages are part of your contract |

In the next chapter, we will see how to test structs, enums, and complex data
types effectively.

---

[← Previous](05-assertions-and-matchers.md) • [Index](00-index.md) • [Next →](07-testing-structs-and-enums.md)
