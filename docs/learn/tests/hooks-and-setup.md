# Hooks and Setup

Lifecycle hooks for test setup and teardown — `before_all`, `after_each`, and RAII guards.

## Prerequisites

- [BDD Specs](bdd-specs.md) — `describe`/`it` blocks

## Glossarium

| Term | Definition |
|------|------------|
| `before_all` | Runs once before any child test in the suite. |
| `after_all` | Runs once after all child tests complete. |
| `before_each` | Runs before each individual test. |
| `after_each` | Runs after each individual test. |
| RAII Guards | Rust values that perform cleanup when dropped (e.g., temp dir, env var guard). |

## Basic Hooks

```rust
use rvtest::spec::describe;

#[test]
fn database_tests() {
    describe("Database")
        .before_all(|| {
            // runs once before any test
            eprintln!("connecting to database...");
        })
        .after_all(|| {
            // runs once after all tests
            eprintln!("closing connection...");
        })
        .it("inserts a record", || { /* ... */ })
        .it("queries records", || { /* ... */ })
        .run()
        .assert_all_pass();
}
```

## Per-Test Hooks

```rust
use std::sync::{Arc, Mutex};

#[test]
fn isolated_tests() {
    let counter = Arc::new(Mutex::new(0));

    describe("Counter")
        .before_each({
            let counter = Arc::clone(&counter);
            move || { *counter.lock().unwrap() = 0; }
        })
        .after_each({
            let counter = Arc::clone(&counter);
            move || { eprintln!("counter was: {}", counter.lock().unwrap()); }
        })
        .it("increments", || { /* starts at 0 */ })
        .it("decrements", || { /* also starts at 0 */ })
        .run()
        .assert_all_pass();
}
```

## RAII Guards

For Rustic resource management without explicit hooks:

```rust
use rvtest::env::set_var;
use rvtest::fs::temp_dir;

#[test]
fn with_temp_environment() {
    let _dir = temp_dir(); // deleted on drop
    let _guard = set_var("DATABASE_URL", "sqlite::memory:");

    // test uses a temporary directory and env var
}
```

## Nested Hooks

```rust
describe("API")
    .before_all(|| { /* outer setup */ })
    .describe("users")
        .before_each(|| { /* inner setup */ })
        .it("creates user", || { /* ... */ })
        .it("deletes user", || { /* ... */ })
    .describe("posts")
        .before_each(|| { /* inner setup */ })
        .it("creates post", || { /* ... */ })
    .run()
    .assert_all_pass();
```

## When to Use What

| Hook | Use Case |
|------|----------|
| `before_all` | Expensive setup shared across tests (db connection, file creation) |
| `after_all` | Cleanup that must run even on failure |
| `before_each` | Reset state between tests |
| `after_each` | Per-test cleanup, logging |
| RAII Guards | Simple, local resource management (temp files, env vars) |

## Next Steps

- [Flaky Tests](flaky-tests.md) — detecting and managing flaky tests
- [Benchmark](benchmark.md) — performance regression testing
