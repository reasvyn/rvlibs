# Chapter 11 — Mocking External Dependencies

[← Previous](10-test-doubles.md) • [Index](00-index.md) • [Next →](12-property-based-testing.md)

---

The previous chapter covered test doubles in theory.  This chapter focuses on
practical techniques for replacing external dependencies in Rust code using
`rvtest`'s built-in `Spy`, `Stub`, and `patch!` tools — with zero proc-macro
overhead.

---

## When to Mock

You need mocking when your code calls:

| Dependency | Example |
|-----------|---------|
| **Network** | HTTP APIs, gRPC, WebSocket |
| **File system** | Reading/writing files, creating directories |
| **System clock** | `SystemTime::now()`, timers |
| **Randomness** | `rand::random()`, UUID generation |
| **External processes** | `Command::new("git")`, shell commands |

In each case, the real dependency makes your test slow, non-deterministic, or
dependent on external infrastructure.

---

## `rvtest::mock::Stub` — Fixed Return Values

`Stub` is the simplest test double.  It wraps a closure that maps input to
output — the closure is called each time the stub is invoked:

```rust
use rvtest::mock::Stub;

#[test]
 fn stub_returns_fixed_value() {
     let stub = Stub::new(|x: i32| x * 2);
     assert_eq!(stub.call(21), 42);
     assert_eq!(stub.call(0), 0);
 }
```

`Stub` is useful when you need a dependency to return a specific value and
you do not care about tracking how many times it was called.

For functions that take references:

```rust
let stub = Stub::new(|name: &str| format!("Hello, {name}!"));
assert_eq!(stub.call("Alice"), "Hello, Alice!");
```

---

## `rvtest::mock::Spy` — Call Recording and Assertions

`Spy` records every invocation and lets you assert on the call history:

```rust
use rvtest::mock::Spy;

#[test]
 fn spy_records_calls() {
     let spy = Spy::new(|x: i32| x * 2);

     spy.call(1);
     spy.call(2);
     spy.call(3);

     assert_eq!(spy.call_count(), 3);
     spy.assert_called_with(&[1, 2, 3]);
 }
```

### Asserting Call Patterns

```rust
#[test]
 fn spy_assertions() {
     let spy = Spy::new(|x: i32| x * 2);

     spy.call(10);
     spy.call(20);

     // Assert that the function was called at least once
     spy.assert_called();

     // Assert the exact call history
     spy.assert_called_with(&[10, 20]);

     // Check individual calls
     let calls = spy.calls();
     assert_eq!(calls[0], 10);
     assert_eq!(calls[1], 20);
 }
```

### Resetting a Spy

```rust
#[test]
 fn spy_reset() {
     let spy = Spy::new(|x: i32| x);
     spy.call(1);
     assert_eq!(spy.call_count(), 1);

     spy.reset();
     assert_eq!(spy.call_count(), 0);
 }
```

### Sharing a Spy Between Tests

`Spy` implements `Clone`, so you can share it between hooks and test
closures:

```rust
use std::sync::Arc;
use rvtest::spec::describe;
use rvtest::mock::Spy;

#[test]
 fn spy_across_hooks() {
     let spy = Spy::new(|x: i32| x);

     describe("Spy in describe")
         .before_each({
             let spy = spy.clone();
             move || { spy.call(0); }
         })
         .it("records before_each call", {
             let spy = spy.clone();
             move || {
                 spy.call(1);
                 assert_eq!(spy.call_count(), 2); // before_each + it
             }
         })
         .run()
         .assert_all_pass();
 }
```

---

## `rvtest::mock::patch!` — Replacing Static Functions

The `Stub` and `Spy` patterns require dependency injection.  But what if
you are testing code that calls a function directly without going through a
trait?

The `patch!` macro lets you temporarily replace a function for the duration
of a test:

```rust
use rvtest::mock::{make_patchable, patch};

// Step 1: declare the function as patchable
make_patchable!(fn get_current_time() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
});

fn calculate_expiry() -> u64 {
    get_current_time() + 3600 // expires in 1 hour
}

#[test]
 fn test_with_frozen_time() {
     // Step 2: patch the function for the duration of this scope
     let _guard = patch!("my_crate::get_current_time", || 1000);
     assert_eq!(calculate_expiry(), 1000 + 3600);
     // The guard is dropped here, restoring the original function
 }
```

### How `patch!` Works

1. `make_patchable!` wraps the function in a static that can be swapped at
   runtime
2. `patch!` saves the original function, installs the replacement, and returns
   a `PatchGuard`
3. When the `PatchGuard` is dropped (at the end of the scope), the original
   function is restored

This is safe even with parallel tests because each test gets its own
`PatchGuard`.

### Patching with Spies

Combine `patch!` with `Spy` for call-recording:

```rust
make_patchable!(fn send_email(to: &str, subject: &str) {
    // Real implementation: connects to SMTP server
});

#[test]
 fn test_email_is_sent() {
     let spy = Spy::new(|(to, subject): (&str, &str)| {
         // Stub implementation — does not actually send
     });
     let _guard = patch!("my_crate::send_email", spy.clone());

     register_user("Alice");

     spy.assert_called();
 }
```

---

## Mocking External Libraries

When the dependency comes from an external crate, you have three options:

### Option 1: Wrap in a Trait (Recommended)

```rust
// Wrap the external dependency in your own trait
trait HttpClient {
    fn get(&self, url: &str) -> Result<String, String>;
}

struct RealHttpClient;
impl HttpClient for RealHttpClient {
    fn get(&self, url: &str) -> Result<String, String> {
        reqwest::blocking::get(url)
            .map_err(|e| e.to_string())?
            .text()
            .map_err(|e| e.to_string())
    }
}

fn fetch_user_data(client: &impl HttpClient, user_id: u32) -> Result<String, String> {
    client.get(&format!("https://api.example.com/users/{user_id}"))
}

// In tests, use a stub
struct StubHttpClient;
impl HttpClient for StubHttpClient {
    fn get(&self, _url: &str) -> Result<String, String> {
        Ok(r#"{"id": 1, "name": "Alice"}"#.into())
    }
}

#[test]
 fn test_fetch_user() {
     let client = StubHttpClient;
     let data = fetch_user_data(&client, 1).unwrap();
     assert!(data.contains("Alice"));
 }
```

### Option 2: Use `make_patchable!` on Your Wrapper

```rust
make_patchable!(fn http_get(url: &str) -> Result<String, String> {
    reqwest::blocking::get(url)
        .map_err(|e| e.to_string())?
        .text()
        .map_err(|e| e.to_string())
});

fn fetch_user_data(user_id: u32) -> Result<String, String> {
    http_get(&format!("https://api.example.com/users/{user_id}"))
}

#[test]
 fn test_with_stubbed_http() {
     let _guard = patch!("my_crate::http_get", |_url| {
         Ok(r#"{"name": "Alice"}"#.into())
     });
     let data = fetch_user_data(1).unwrap();
     assert!(data.contains("Alice"));
 }
```

### Option 3: Conditional Compilation (Last Resort)

```rust
#[cfg(test)]
fn get_current_time() -> u64 {
    // Return fixed time during tests
    1000000
}

#[cfg(not(test))]
fn get_current_time() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
```

This approach is simple but makes it impossible to test different time values
in different tests.

---

## Mocking Filesystem Operations

For mocking filesystem operations, use `TempDir` (or `tempfile` crate) to
create an isolated directory:

```rust
use tempfile::TempDir;

fn count_files(dir: &std::path::Path) -> std::io::Result<usize> {
    Ok(std::fs::read_dir(dir)?.count())
}

#[test]
 fn count_files_in_directory() {
     let tmp = TempDir::new().unwrap();

     // Create test files
     std::fs::write(tmp.path().join("a.txt"), "").unwrap();
     std::fs::write(tmp.path().join("b.txt"), "").unwrap();

     assert_eq!(count_files(tmp.path()).unwrap(), 2);
     // tmp is deleted automatically when it goes out of scope
 }
```

---

## Mocking System Time

When your code uses `SystemTime::now()`, inject the time as a parameter
instead of calling it directly:

```rust
// ❌ Hard to test
fn is_expired(expiry: u64) -> bool {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    now > expiry
}

// ✅ Easy to test
fn is_expired(expiry: u64, now: u64) -> bool {
    now > expiry
}

#[test]
 fn test_expiry() {
     assert!(!is_expired(100, 50));  // Not expired yet
     assert!(is_expired(50, 100));   // Expired
 }
```

---

## Mocking Randomness

For functions that use random data, pass a seedable RNG as a parameter:

```rust
use rand::{Rng, SeedableRng};

// ❌ Hard to test
fn generate_id() -> u64 {
    rand::random()
}

// ✅ Easy to test
fn generate_id_with(rng: &mut impl Rng) -> u64 {
    rng.random()
}

#[test]
 fn test_generate_id_is_deterministic_with_seed() {
     let mut rng1 = rand::rngs::StdRng::seed_from_u64(42);
     let mut rng2 = rand::rngs::StdRng::seed_from_u64(42);
     assert_eq!(generate_id_with(&mut rng1), generate_id_with(&mut rng2));
 }
```

---

## Summary

| Tool | Use Case |
|------|----------|
| `Stub` | Fixed return values, no call tracking |
| `Spy` | Call recording and assertions |
| `patch!` | Temporarily replace a function (no DI required) |
| Trait + stub | Mock external library dependencies |
| `TempDir` | Isolated filesystem access |
| Injected RNG | Deterministic random values |
| Injected time | Deterministic clock |

Mocking is a means to an end — test behaviour, not implementation.  If you
find yourself writing overly complex mocks, the code under test may need to be
simplified.

---

[← Previous](10-test-doubles.md) • [Index](00-index.md) • [Next →](12-property-based-testing.md)
