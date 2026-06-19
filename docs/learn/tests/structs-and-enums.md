# Testing Structs and Enums

Testing custom types — deriving `PartialEq` and `Debug`, custom comparison logic, and testing enum variants.

## Prerequisites

- [Testing Errors](testing-errors.md) — testing panics and results
- Rust traits — `PartialEq`, `Debug`, `Display`

## Glossarium

| Term | Definition |
|------|------------|
| `#[derive(PartialEq)]` | Auto-implements equality comparison for a struct or enum. Required for `assert_eq!`. |
| `#[derive(Debug)]` | Auto-implements debug formatting (`{:?}`). Required for assertion failure output. |
| Custom `PartialEq` | Manual implementation when the default field-by-field comparison is insufficient. |
| Snapshot | Stored output that is compared against future test runs. |

## Derive for Assertions

```rust
#[derive(Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

#[test]
fn point_equality() {
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = Point { x: 1.0, y: 2.0 };
    assert_eq!(p1, p2);
}
```

Without `PartialEq`, `assert_eq!` won't compile. Without `Debug`, the compiler will complain about formatting on failure.

## Custom PartialEq

When field-by-field comparison isn't appropriate (e.g., floating-point tolerance):

```rust
struct Measurement {
    value: f64,
    unit: String,
}

impl PartialEq for Measurement {
    fn eq(&self, other: &Self) -> bool {
        self.unit == other.unit && (self.value - other.value).abs() < 1e-6
    }
}
```

## Testing Enum Variants

```rust
#[derive(Debug, PartialEq)]
enum HttpStatus {
    Ok(u16),
    NotFound,
    Error { code: u16, message: String },
}

#[test]
fn match_ok_variant() {
    let status = HttpStatus::Ok(200);
    assert_eq!(status, HttpStatus::Ok(200));
}

#[test]
fn match_error_variant() {
    let status = HttpStatus::Error {
        code: 500,
        message: "Server Error".into(),
    };
    match status {
        HttpStatus::Error { code, .. } => assert_eq!(code, 500),
        _ => panic!("expected Error variant"),
    }
}
```

## With rvtest

```rust
use rvtest::assert_eq as assert_eq;

#[derive(Debug, PartialEq)]
struct Config {
    host: String,
    port: u16,
}

#[test]
fn config_comparison() {
    let a = Config { host: "localhost".into(), port: 8080 };
    let b = Config { host: "localhost".into(), port: 8080 };
    rvtest::assert_eq!(a, b); // shows diff on failure
}
```

## Next Steps

- [Hooks and Setup](hooks-and-setup.md) — setup/teardown with lifecycle hooks
- [Parametrized Tests](parametrized-tests.md) — running the same logic with multiple inputs
