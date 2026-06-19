# Chapter 17 — Legacy and Refactoring

[← Previous](16-benchmark-and-regression.md) • [Index](00-index.md) • [Next →](18-ci-integration.md)

---

Most real-world projects are not greenfield.  You will often need to add tests
to code that was never designed for testing — functions that call global state,
mix concerns, or have no dependency injection.  This chapter covers strategies
for bringing existing code under test.

---

## The Problem: Untestable Code

```rust
// A typical piece of untestable code:
pub fn process_order(order_id: u32) -> Result<String, String> {
    let db = Database::connect("postgres://localhost:5432/production");
    let order = db.query_order(order_id).map_err(|e| e.to_string())?;
    if order.total > 1000.0 {
        let email = EmailSender::new("smtp.example.com");
        email.send("admin@example.com", &format!("Large order: {order_id}"));
    }
    format!("Order {order_id} processed")
}
```

Problems:
- Hard-coded database connection string
- Direct call to `EmailSender::new` — cannot mock
- No way to test the "large order" path without a real database
- Tests would require network access to PostgreSQL and SMTP

---

## Strategy 1: Characterisation Tests

Before refactoring, write **characterisation tests** that capture the current
behaviour.  These tests document what the code actually does, not what it
should do.  They serve as a safety net during refactoring.

```rust
// Step 1: Test the function through its current interface
#[test]
 fn process_order_returns_ok_format() {
     // This test documents the current return format
     // It will fail if the return format changes
     let result = process_order(1);
     assert!(result.is_ok());
     assert!(result.unwrap().contains("processed"));
 }
```

The purpose is not to validate correctness — it is to detect unintended
behaviour changes during refactoring.

---

## Strategy 2: Dependency Injection via Wrapper

Add a thin wrapper that accepts dependencies:

```rust
// Step 2: Extract the logic into a testable function

// Original function (keep for backwards compatibility)
pub fn process_order(order_id: u32) -> Result<String, String> {
    let db = Database::connect("postgres://localhost:5432/production");
    let email = EmailSender::new("smtp.example.com");
    process_order_inner(order_id, &db, &email)
}

// New testable version
fn process_order_inner(
    order_id: u32,
    db: &impl Database,
    email: &impl EmailSender,
) -> Result<String, String> {
    let order = db.query_order(order_id).map_err(|e| e.to_string())?;
    if order.total > 1000.0 {
        email.send("admin@example.com", &format!("Large order: {order_id}"));
    }
    Ok(format!("Order {order_id} processed"))
}

// Now testable!
#[test]
 fn test_process_order_inner() {
     let db = FakeDatabase::new();
     let email = SpyEmailSender::new();

     let result = process_order_inner(1, &db, &email);

     assert!(result.is_ok());
     assert!(result.unwrap().contains("processed"));
 }
```

This is the **Extract and Override** pattern — you extract the logic into a
function that accepts testable dependencies, and the original function becomes
a thin wrapper.

---

## Strategy 3: Feature Flag for Testability

When you cannot change the function signature, use conditional compilation:

```rust
#[cfg(test)]
use mockable::get_current_time;

#[cfg(not(test))]
fn get_current_time() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[cfg(test)]
mod mockable {
    use std::sync::atomic::{AtomicU64, Ordering};

    static MOCK_TIME: AtomicU64 = AtomicU64::new(0);

    pub fn set_time(t: u64) {
        MOCK_TIME.store(t, Ordering::SeqCst);
    }

    pub fn get_current_time() -> u64 {
        MOCK_TIME.load(Ordering::SeqCst)
    }
}

#[test]
 fn test_with_mocked_time() {
     mockable::set_time(1000);
     assert_eq!(calculate_expiry(3600), 4600);
 }
```

---

## Strategy 4: The Sprout Method

When you need to add a new behaviour to existing code, do not modify the
untestable code directly.  Instead, "sprout" a new testable function and call
it from the existing code:

```rust
// Existing untestable code — do not modify
pub fn save_user(name: &str) -> Result<(), String> {
    let db = Database::connect("postgres://localhost:5432/production");
    // ... existing logic ...

    // Sprout: call the new testable function
    validate_name(name).map_err(|e| e.to_string())?;

    // ...
    Ok(())
}

// New sprouted function — testable
pub fn validate_name(name: &str) -> Result<(), ValidationError> {
    if name.is_empty() {
        return Err(ValidationError::Empty);
    }
    if name.len() < 2 {
        return Err(ValidationError::TooShort);
    }
    Ok(())
}

// Tests for the new function
#[test]
 fn validate_name_rejects_empty() {
     assert!(validate_name("").is_err());
 }

 #[test]
 fn validate_name_rejects_short() {
     assert!(validate_name("a").is_err());
 }

 #[test]
 fn validate_name_accepts_valid() {
     assert!(validate_name("Alice").is_ok());
 }
```

---

## The Legacy Code Change Algorithm

When adding tests to legacy code, follow this order:

```
1. Identify the code that needs changing
2. Write characterisation tests (capture current behaviour)
3. Make the change (refactor, extract, add DI)
4. Run characterisation tests — they should pass (behaviour preserved)
5. Write new tests for the extracted/testable code
6. Remove characterisation tests if they overlap with new tests
```

Never skip step 2.  Without characterisation tests, you cannot tell whether
your refactoring changed behaviour.

---

## Example: Refactoring Legacy Code

```rust
// --- Original code (untestable) ---
pub fn generate_report() -> String {
    let content = std::fs::read_to_string("/etc/config/data.csv").unwrap();
    let mut lines: Vec<&str> = content.lines().collect();
    lines.sort();
    lines.join("\n")
}

// --- Step 2: Characterisation test ---
#[test]
 fn generate_report_returns_sorted_lines() {
     // This test captures current behaviour
     // It will need mock data after refactoring
     let result = generate_report();
     assert!(!result.is_empty()); // Just check it returns something
 }

// --- Step 3: Extract testable function ---
fn process_csv_content(content: &str) -> String {
    let mut lines: Vec<&str> = content.lines().collect();
    lines.sort();
    lines.join("\n")
}

pub fn generate_report() -> String {
    let content = std::fs::read_to_string("/etc/config/data.csv").unwrap();
    process_csv_content(&content)
}

// --- Step 5: Test the extracted function ---
#[test]
 fn process_csv_content_sorts_lines() {
     let input = "b\na\nc";
     assert_eq!(process_csv_content(input), "a\nb\nc");
 }

 #[test]
 fn process_csv_content_handles_empty() {
     assert_eq!(process_csv_content(""), "");
 }

 #[test]
 fn process_csv_content_handles_single_line() {
     assert_eq!(process_csv_content("only"), "only");
 }
```

---

## When Refactoring Is Too Expensive

Sometimes the legacy code is too entangled to refactor safely.  In those
cases, write **integration-level tests** that test through the public API,
even if they are slower:

```rust
// Integration-level test for legacy code
// Tests through the public API, no refactoring needed
#[test]
 fn generate_report_integration() {
     // This test uses the real filesystem
     // It is slower but requires no code changes
     let result = generate_report();
     assert!(result.contains("expected_content"));
 }
```

These tests are not ideal, but they are better than no tests.  Over time, as
you refactor the code, you can replace them with focused unit tests.

---

## Summary

| Strategy | When to Use |
|----------|-------------|
| **Characterisation tests** | Before any refactoring — capture current behaviour |
| **Extract and Override** | Code that calls hard-coded dependencies |
| **Conditional compilation** | When signatures cannot change |
| **Sprout method** | Adding new behaviour to existing code |
| **Integration tests** | When refactoring is too expensive |

> **rvtest:** Use `cargo rvtest --retest` to re-run only previously failed tests during refactoring iterations, and `--diff` to compare results against the previous run.

The goal is not to achieve perfect testability overnight.  Every test you add
— even a characterisation test — makes the codebase safer to change.

In the next chapter, we will look at integrating tests into CI pipelines and
configuring them for different environments.

---

[← Previous](16-benchmark-and-regression.md) • [Index](00-index.md) • [Next →](18-ci-integration.md)
