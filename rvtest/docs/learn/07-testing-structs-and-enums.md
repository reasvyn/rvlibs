# Chapter 07 — Testing Structs and Enums

[← Previous](06-testing-errors.md) • [Index](00-index.md) • [Next →](08-parametrized-tests.md)

---

Most real-world Rust code revolves around structs and enums.  This chapter
covers how to test them effectively — from basic equality checks to custom
comparison logic.

---

## The `PartialEq` and `Debug` Requirements

To use `assert_eq!` (standard or `rvtest`) with a custom type, the type must
implement two traits:

- **`PartialEq`** — defines how two values are compared
- **`Debug`** — defines how values are displayed on failure

The easiest way to implement both is to derive them:

```rust
#[derive(Debug, PartialEq)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

#[test]
 fn compare_users() {
     let user1 = User { id: 1, name: "Alice".into(), email: "alice@example.com".into() };
     let user2 = User { id: 1, name: "Alice".into(), email: "alice@example.com".into() };
     rvtest::assert_eq!(user1, user2);
 }
```

---

## Struct Comparison with `rvtest::assert_eq!`

When the assertion fails, `rvtest::assert_eq!` shows a diff of the struct
fields, making it easy to spot which field differs:

```rust
#[test]
 fn user_email_mismatch() {
     let actual = User { id: 1, name: "Alice".into(), email: "alice@work.com".into() };
     let expected = User { id: 1, name: "Alice".into(), email: "alice@home.com".into() };
     rvtest::assert_eq!(actual, expected);
     // Failure output highlights: email differs: "alice@work.com" vs "alice@home.com"
 }
```

---

## Custom `PartialEq` Implementations

Sometimes the default field-by-field comparison is wrong:

```rust
pub struct Temperature {
    pub celsius: f64,
}

impl PartialEq for Temperature {
    fn eq(&self, other: &Self) -> bool {
        // Compare with 0.1 degree tolerance
        (self.celsius - other.celsius).abs() < 0.1
    }
}

impl std::fmt::Debug for Temperature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}°C", self.celsius)
    }
}

#[test]
 fn temperatures_within_tolerance() {
     let t1 = Temperature { celsius: 36.5 };
     let t2 = Temperature { celsius: 36.6 };
     assert_eq!(t1, t2); // Uses custom PartialEq
 }
```

---

## Testing Enum Variants

Enums with data are common in Rust.  Test each variant independently:

```rust
#[derive(Debug, PartialEq)]
pub enum ApiResponse {
    Success { data: String, timestamp: u64 },
    NotFound { resource: String },
    Unauthorized { message: String },
    RateLimited { retry_after_secs: u64 },
}

fn fetch_resource(id: u32) -> ApiResponse {
    match id {
        1 => ApiResponse::Success { data: "content".into(), timestamp: 1000 },
        2 => ApiResponse::NotFound { resource: "item_2".into() },
        _ => ApiResponse::Unauthorized { message: "access denied".into() },
    }
}

#[test]
 fn fetch_existing_resource() {
    let response = fetch_resource(1);
    assert_eq!(response, ApiResponse::Success {
        data: "content".into(),
        timestamp: 1000,
    });
 }

 #[test]
 fn fetch_nonexistent_resource() {
     let response = fetch_resource(2);
     assert!(matches!(response, ApiResponse::NotFound { .. }));
 }

 #[test]
 fn fetch_without_permission() {
     let response = fetch_resource(99);
     assert!(matches!(response, ApiResponse::Unauthorized { .. }));
 }
```

---

## Pattern Matching in Assertions

Use `matches!` macro for variants where you only care about the variant, not
the inner data:

```rust
#[test]
 fn response_is_not_found() {
     let response = fetch_resource(2);
     assert!(matches!(response, ApiResponse::NotFound { .. }));
 }
```

Or use `assert_matches!` from `rvtest` for a better error message on failure:

```rust
#[test]
 fn response_is_success() {
     let response = fetch_resource(1);
     rvtest::assert_matches!(response, ApiResponse::Success { .. });
 }
```

When the pattern does not match, `rvtest::assert_matches!` shows both the
expected pattern and the actual value:

```
assertion failed: expected `Success { .. }`, got `NotFound { resource: "item_2" }`
```

---

## Testing Large Structs with Defaults

For structs with many fields, provide a builder or default to avoid
repeating all fields in every test:

```rust
#[derive(Debug, PartialEq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub timeout_secs: u64,
    pub retries: u32,
    pub tls: bool,
    pub log_level: String,
}

impl Config {
    pub fn test_default() -> Self {
        Config {
            host: "localhost".into(),
            port: 8080,
            timeout_secs: 30,
            retries: 3,
            tls: false,
            log_level: "debug".into(),
        }
    }
}

#[test]
 fn config_uses_default_port() {
     let config = Config::test_default();
     assert_eq!(config.port, 8080);
 }
```

This keeps tests concise when only a few fields matter for the scenario being
tested.

---

## Partial Comparison with Helper Functions

When you only care about a subset of fields, write a comparison helper:

```rust
// Only compare fields relevant to this test
fn assert_user_id_and_name(actual: &User, expected_id: u32, expected_name: &str) {
    assert_eq!(actual.id, expected_id, "id mismatch");
    assert_eq!(actual.name, expected_name, "name mismatch");
    // Email and other fields are ignored
}

#[test]
 fn user_has_correct_id_and_name() {
     let user = create_user("Alice", "alice@example.com");
     assert_user_id_and_name(&user, 1, "Alice");
 }
```

---

## Dealing with Non-Comparable Fields

Some fields cannot or should not be compared:

```rust
pub struct Event {
    pub id: u32,
    pub name: String,
    pub timestamp: std::time::SystemTime,     // Not comparable
    pub uuid: uuid::Uuid,                      // Generated randomly
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name
        // timestamp and uuid are ignored in comparison
    }
}

#[test]
 fn events_with_same_id_and_name_are_equal() {
     let event1 = Event {
         id: 1, name: "login".into(),
         timestamp: std::time::SystemTime::now(),
         uuid: uuid::Uuid::new_v4(),
     };
     let event2 = Event {
         id: 1, name: "login".into(),
         timestamp: std::time::SystemTime::now(),
         uuid: uuid::Uuid::new_v4(),
     };
     assert_eq!(event1, event2); // Passes despite different timestamps and UUIDs
 }
```

---

## Summary

- Derive `PartialEq` and `Debug` for most structs — it enables `assert_eq!`
- Implement `PartialEq` manually when you need custom comparison logic
- Use `matches!` or `rvtest::assert_matches!` for enum variant assertions
- Provide test defaults or builder methods to keep test code concise
- Write comparison helpers when only a subset of fields matter
- Omit non-deterministic fields (timestamps, UUIDs) from equality checks

In the next chapter, we will see how to avoid repeating the same test logic
for multiple inputs using parametrized tests.

---

[← Previous](06-testing-errors.md) • [Index](00.index.md) • [Next →](08-parametrized-tests.md)
