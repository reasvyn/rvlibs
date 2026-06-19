# Testing Errors

Testing code that fails — `#[should_panic]`, `Result<T, E>` in tests, and testing error variants.

## Prerequisites

- [Writing Tests](writing-tests.md) — AAA pattern, test structure
- Rust error handling — `panic!`, `Result`, `Option`

## Glossarium

| Term | Definition |
|------|------------|
| `#[should_panic]` | An attribute that asserts a test function panics. |
| `#[should_panic(expected = "...")]` | Asserts the panic message contains the given string. |
| `Result<T, E>` in tests | Tests can return `Result` — `Ok` means pass, `Err` means fail. |
| `catch_unwind` | Captures a panic without aborting the test. |

## Testing Panics

```rust
#[test]
#[should_panic]
fn division_by_zero() {
    divide(1, 0); // should panic
}
```

With expected message:

```rust
#[test]
#[should_panic(expected = "division by zero")]
fn division_by_zero_message() {
    divide(1, 0);
}
```

## Testing `Result` Return Types

Tests can return `Result` — `Ok(())` passes, `Err` fails:

```rust
#[test]
fn parse_number() -> Result<(), String> {
    let n: i32 = "42".parse().map_err(|e| format!("parse failed: {e}"))?;
    assert_eq!(n, 42);
    Ok(())
}
```

This works naturally with the `?` operator:

```rust
#[test]
fn read_and_parse_config() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&content)?;
    assert_eq!(config.version, 1);
    Ok(())
}
```

## Testing Error Variants

```rust
#[test]
fn returns_not_found() {
    let result = find_user(999);
    assert!(result.is_err());

    match result {
        Err(AppError::NotFound(id)) => assert_eq!(id, 999),
        _ => panic!("expected NotFound, got {:?}", result),
    }
}
```

With rvtest:

```rust
use rvtest::assert_err;

#[test]
fn returns_not_found() {
    let result: Result<i32, _> = Err("not found");
    rvtest::assert_err!(result);
}
```

## `catch_unwind` for Advanced Scenarios

```rust
use std::panic::catch_unwind;

#[test]
fn verify_panic_behavior() {
    let result = catch_unwind(|| {
        divide(1, 0);
    });
    assert!(result.is_err(), "expected a panic");
}
```

## When to Use What

| Scenario | Approach |
|----------|----------|
| Function panics on invalid input | `#[should_panic]` |
| Function returns `Result` | Return `Result` from test + `?` |
| Assert specific error variant | Pattern match on the error |
| Test both success and failure | Separate `#[test]` functions |
| Verify panic message content | `#[should_panic(expected = "...")]` |

## Next Steps

- [Structs and Enums](structs-and-enums.md) — testing custom types with `PartialEq`, `Debug`
- [BDD Specs](../patterns/bdd-specs.md) — organising tests with `describe`/`it`
