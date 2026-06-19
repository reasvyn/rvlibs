# Chapter 15 — Flaky Tests

[← Previous](14-coverage.md) • [Index](00-index.md) • [Next →](16-benchmark-and-regression.md)

---

A flaky test is one that sometimes passes and sometimes fails without any
code change.  Flaky tests destroy trust in the test suite.  When developers
start ignoring failing tests because "it might be flaky," the test suite has
lost its value.  This chapter covers how to detect, diagnose, and fix flaky
tests.

---

## What Makes a Test Flaky?

Flaky tests are caused by non-determinism.  Common sources:

| Cause | Example |
|-------|---------|
| **Race conditions** | Tests that depend on thread scheduling order |
| **Network timeouts** | Tests that call external APIs |
| **Random data** | Tests using `rand::random()` without a fixed seed |
| **Timing dependencies** | Tests that sleep or depend on wall-clock time |
| **Filesystem races** | Tests that write to the same file path |
| **Hash order** | Tests depending on `HashMap` or `HashSet` iteration order |
| **Environment leaks** | Tests that modify global state without restoring it |

---

## Detecting Flaky Tests with `rvtest`

`rvtest` has a built-in flaky test detector:

```bash
cargo rvtest --detect-flaky
```

This runs the entire test suite 10 times (by default) and records the
pass/fail history of each test:

```
  🔍 Running test suite 10 times to detect flaky tests...

  ⚠  concurrent_write_test               8/10 passes (80%)
  ⚠  network_timeout_test                 7/10 passes (70%)
```

Configure the number of runs:

```bash
cargo rvtest --detect-flaky=50   # Run 50 times
```

---

## How to Fix Common Causes of Flakiness

### Race Conditions

```rust
// ❌ Flaky: depends on thread scheduling
#[test]
 fn test_concurrent_counter() {
     let counter = std::sync::Arc::new(std::sync::atomic::AtomicI32::new(0));
     let mut handles = Vec::new();
     for _ in 0..10 {
         let c = counter.clone();
         handles.push(std::thread::spawn(move || {
             c.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
         }));
     }
     for h in handles { h.join().unwrap(); }
     assert_eq!(counter.load(std::sync::atomic::Ordering::SeqCst), 10);
 }
```

```rust
// ✅ Deterministic: use a barrier to control execution order
use std::sync::{Arc, Barrier};

#[test]
 fn test_concurrent_counter_deterministic() {
     let counter = Arc::new(std::sync::atomic::AtomicI32::new(0));
     let barrier = Arc::new(Barrier::new(11));
     let mut handles = Vec::new();
     for _ in 0..10 {
         let c = counter.clone();
         let b = barrier.clone();
         handles.push(std::thread::spawn(move || {
             b.wait(); // Wait for all threads to be ready
             c.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
         }));
     }
     barrier.wait(); // Signal all threads to start
     for h in handles { h.join().unwrap(); }
     assert_eq!(counter.load(std::sync::atomic::Ordering::SeqCst), 10);
 }
```

### Network Dependencies

```rust
// ❌ Flaky: depends on network availability
#[test]
 fn test_api_call() {
     let response = reqwest::blocking::get("https://api.example.com/data").unwrap();
     assert!(response.status().is_success());
 }
```

```rust
// ✅ Deterministic: mock the HTTP client
#[test]
 fn test_api_call_with_mock() {
     let client = StubHttpClient::new(200, r#"{"status": "ok"}"#);
     let result = fetch_data(&client);
     assert_eq!(result.status, "ok");
 }
```

### Random Data Without Fixed Seed

```rust
// ❌ Flaky: different seed every time
#[test]
 fn test_random_operation() {
     let value = rand::random::<u32>();
     let result = process(value);
     assert!(result.is_ok());
 }
```

```rust
// ✅ Deterministic: use a fixed seed
use rand::SeedableRng;

#[test]
 fn test_random_operation_with_seed() {
     let mut rng = rand::rngs::StdRng::seed_from_u64(42);
     let value: u32 = rng.random();
     let result = process(value);
     assert!(result.is_ok());
 }
```

Or use `rvtest`'s `--seed` CLI flag:

```bash
cargo rvtest --seed 42
```

### Time-Dependent Tests

```rust
// ❌ Flaky: depends on current time
#[test]
 fn test_greeting() {
     let greeting = greet("Alice");
     assert_eq!(greeting, "Good morning, Alice!"); // Fails in the afternoon
 }
```

```rust
// ✅ Deterministic: inject the time
fn greet(name: &str, hour: u8) -> String {
     if hour < 12 {
         format!("Good morning, {name}!")
     } else {
         format!("Good evening, {name}!")
     }
 }

 #[test]
 fn test_greeting_morning() {
     assert_eq!(greet("Alice", 9), "Good morning, Alice!");
 }

 #[test]
 fn test_greeting_evening() {
     assert_eq!(greet("Alice", 20), "Good evening, Alice!");
 }
```

### Tests Depending on HashMap Order

```rust
// ❌ Flaky: HashMap iteration order is not guaranteed
#[test]
 fn test_hash_map_order() {
     let mut map = std::collections::HashMap::new();
     map.insert("a", 1);
     map.insert("b", 2);
     let values: Vec<_> = map.values().collect();
     assert_eq!(values, vec![&1, &2]); // May fail on different Rust versions
 }
```

```rust
// ✅ Deterministic: sort before comparing
#[test]
 fn test_hash_map_content() {
     let mut map = std::collections::HashMap::new();
     map.insert("a", 1);
     map.insert("b", 2);
     let mut values: Vec<_> = map.values().copied().collect();
     values.sort();
     assert_eq!(values, vec![1, 2]);
 }
```

---

## Quarantining Flaky Tests

### Automatic Retries

When a test is flaky but you cannot fix it immediately, use retries as a
temporary workaround. `rvtest` supports per-suite retries:

```rust
use rvtest::spec::describe;

#[test]
fn potentially_flaky() {
    describe("Network")
        .it("calls unreliable API", || {
            let result = call_api();
            assert!(result.is_ok());
        })
        .retries(3)   // Retry up to 3 times on failure
        .run()
        .assert_all_pass();
}
```

Or use `--retries` globally or `--auto-retry` for a single retry:

```bash
cargo rvtest --retries 3
cargo rvtest --auto-retry    # Retry failed tests once
```

### Flaky Quarantine

`rvtest` provides a full quarantine workflow for managing flaky tests.
After detecting flaky tests with `--detect-flaky`:

```bash
# Detect flaky tests (saves to target/.rvtest-cache/flaky.json)
cargo rvtest --detect-flaky

# Skip known-flaky tests in subsequent runs
cargo rvtest --quarantine

# Run normally but include quarantined tests
cargo rvtest --quarantine --include-flaky

# List currently quarantined tests
cargo rvtest --flaky-report

# Clear the quarantine list
cargo rvtest --unquarantine
```

The quarantine workflow lets you:
1. **Detect** flaky tests with `--detect-flaky` (runs the suite N times)
2. **Quarantine** them with `--quarantine` so they don't break your build
3. **Review** the list with `--flaky-report`
4. **Fix** the underlying flakiness
5. **Unquarantine** with `--unquarantine` when fixed

---

## The Cost of Flaky Tests

Flaky tests have a real cost:

- **Wasted CI time** — Restarting failed CI pipelines
- **Lost trust** — Developers stop believing test failures are real
- **Delayed releases** — Cannot merge because of "random" failures
- **Noise blindness** — Real failures are ignored because they look flaky

Aim for **zero flaky tests**.  Any flaky test is a bug in the test, not in
the code.

---

## Summary

- Flaky tests are non-deterministic: they pass or fail inconsistently
- Common causes: race conditions, network, time, randomness, hash order
- Use `cargo rvtest --detect-flaky` to identify flaky tests
- Fix flaky tests by injecting determinism (fixed seed, mocked time,
  synchronous execution)
- Use `--retries` or `.retries(N)` as a temporary workaround
- Quarantine flaky tests until they can be fixed
- Zero flaky tests should be the goal

In the next chapter, we will explore benchmark and regression testing —
measuring performance and catching regressions.

---

[← Previous](14-coverage.md) • [Index](00-index.md) • [Next →](16-benchmark-and-regression.md)
