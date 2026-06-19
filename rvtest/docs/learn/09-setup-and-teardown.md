# Chapter 09 — Setup and Teardown

[← Previous](08-parametrized-tests.md) • [Index](00-index.md) • [Next →](10-test-doubles.md)

---

Many tests need the same preparation steps: create a database connection, set
environment variables, write temporary files, or initialise a service.  Repeating
this setup in every test is wasteful and error-prone.  This chapter covers
patterns for managing shared setup and teardown logic in Rust, using `rvtest`'s
lifecycle hooks.

---

## The Problem: Repeated Setup Code

```rust
#[test]
 fn test_insert_user() {
     let mut db = Database::new_in_memory();
     db.run_migrations().unwrap();
     db.insert_user("Alice").unwrap();
     assert!(db.user_exists("Alice"));
 }

 #[test]
 fn test_delete_user() {
     let mut db = Database::new_in_memory();
     db.run_migrations().unwrap();
     db.insert_user("Alice").unwrap();
     db.insert_user("Bob").unwrap();
     db.delete_user("Alice").unwrap();
     assert!(!db.user_exists("Alice"));
 }
```

The database setup is duplicated in every test.  If the setup changes (for
example, a new migration is added), every test must be updated.

---

## Helper Functions for Setup

The simplest solution is a helper function:

```rust
fn setup_db() -> Database {
    let mut db = Database::new_in_memory();
    db.run_migrations().unwrap();
    db
}

#[test]
 fn test_insert_user() {
     let mut db = setup_db();
     db.insert_user("Alice").unwrap();
     assert!(db.user_exists("Alice"));
 }

 #[test]
 fn test_delete_user() {
     let mut db = setup_db();
     db.insert_user("Alice").unwrap();
     db.insert_user("Bob").unwrap();
     db.delete_user("Alice").unwrap();
     assert!(!db.user_exists("Alice"));
 }
```

This is better, but each test still repeats `setup_db()` at the top.

---

## Introducing `rvtest` `describe` Blocks

`rvtest` provides `describe` blocks with lifecycle hooks.  A `before_all` hook
runs once before any test in the block, and an `after_all` hook runs after all
tests complete:

```rust
use rvtest::spec::describe;

#[test]
 fn database_tests() {
     let db = std::sync::Arc::new(std::sync::Mutex::new(Database::new_in_memory()));

     describe("Database")
         .before_all(move || {
             let mut db = db.lock().unwrap();
             db.run_migrations().unwrap();
         })
         .it("inserts a user", || {
             let mut db = db.lock().unwrap();
             db.insert_user("Alice").unwrap();
             assert!(db.user_exists("Alice"));
         })
         .it("deletes a user", || {
             let mut db = db.lock().unwrap();
             db.insert_user("Alice").unwrap();
             db.insert_user("Bob").unwrap();
             db.delete_user("Alice").unwrap();
             assert!(!db.user_exists("Alice"));
         })
         .run()
         .assert_all_pass();
 }
```

The `before_all` hook runs once.  The `after_all` hook (not shown here) is
guaranteed to run even if one of the tests panics.

---

## `before_each` and `after_each`

For setup that must run before *every* test, use `before_each` and
`after_each`:

```rust
#[test]
 fn counter_tests() {
     let counter = std::sync::Arc::new(std::sync::atomic::AtomicI32::new(0));

     describe("Counter")
         .before_each(move || {
             counter.store(0, std::sync::atomic::Ordering::SeqCst);
         })
         .it("starts at zero", move || {
             assert_eq!(counter.load(std::sync::atomic::Ordering::SeqCst), 0);
         })
         .it("increments", move || {
             counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
             assert_eq!(counter.load(std::sync::atomic::Ordering::SeqCst), 1);
         })
         .run()
         .assert_all_pass();
 }
```

This guarantees a clean state for each test, regardless of what the previous
test did.

---

## Nested Hooks

Hooks compose across nested `describe` blocks.  Parent hooks run before child
hooks:

```rust
#[test]
 fn nested_hooks() {
     let log = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));

     describe("Outer")
         .before_each({
             let log = log.clone();
             move || log.lock().unwrap().push("outer")
         })
         .describe("Inner")
             .before_each({
                 let log = log.clone();
                 move || log.lock().unwrap().push("inner")
             })
             .it("runs both hooks", move || {
                 let log = log.lock().unwrap();
                 assert_eq!(*log, vec!["outer", "inner"]);
             })
             .run()
         .run()
         .assert_all_pass();
 }
```

Hooks run from outermost to innermost before the test, and from innermost to
outermost after the test.

---

## Environment Variables

Tests often need to set environment variables.  Because `std::env::set_var`
affects the entire process, it must be paired with a restore:

```rust
fn with_env_var(name: &str, value: &str, f: impl FnOnce()) {
    let old = std::env::var(name).ok();
    std::env::set_var(name, value);
    f();
    match old {
        Some(v) => std::env::set_var(name, v),
        None => std::env::remove_var(name),
    }
}

#[test]
 fn test_config_from_env() {
     with_env_var("APP_HOST", "example.com", || {
         let config = load_config();
         assert_eq!(config.host, "example.com");
     });
     // After the closure, env var is restored
 }
```

---

## Temporary Directories

Tests that write files need isolated temporary directories:

```rust
fn with_temp_dir(f: impl FnOnce(&std::path::Path)) {
    let dir = std::env::temp_dir().join(format!("rvtest_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    f(&dir);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
 fn test_file_creation() {
     with_temp_dir(|dir| {
         let file_path = dir.join("test.txt");
         std::fs::write(&file_path, "hello").unwrap();
         assert!(file_path.exists());
         assert_eq!(std::fs::read_to_string(&file_path).unwrap(), "hello");
     });
 }
```

The `tempfile` crate is a popular choice that handles this pattern with
automatic cleanup:

```toml
[dev-dependencies]
tempfile = "3"
```

```rust
use tempfile::TempDir;

#[test]
 fn test_with_tempdir_crate() {
     let tmp = TempDir::new().unwrap();
     let file_path = tmp.path().join("test.txt");
     std::fs::write(&file_path, "data").unwrap();
     assert!(file_path.exists());
     // TempDir is deleted when it goes out of scope
 }
```

---

## Shared State Between Hooks and Tests

Use `Arc` to share state between hooks and tests:

```rust
use std::sync::Arc;

#[test]
 fn shared_counter() {
     let counter = Arc::new(std::sync::Mutex::new(0));
     let setup_counter = counter.clone();

     describe("Counter")
         .before_all(move || {
             let mut c = setup_counter.lock().unwrap();
             *c = 42;
         })
         .it("has value from before_all", move || {
             let c = counter.lock().unwrap();
             assert_eq!(*c, 42);
         })
         .run()
         .assert_all_pass();
 }
```

---

## What Not to Do

### Global Mutable State

```rust
// ❌ Never use global mutability in tests
static DB: std::sync::Mutex<Option<Database>> = std::sync::Mutex::new(None);
```

Global state makes tests depend on each other and breaks parallel execution.

### Tests That Modify Shared Files

```rust
// ❌ Tests that write to the same file will conflict
#[test]
 fn write_to_shared_file() {
     std::fs::write("/tmp/shared.txt", "data").unwrap();
 }
```

Always use temporary directories for file operations.

### rvtest `TempDir` — Guaranteed Cleanup

`rvtest` provides `temp_dir()` and `temp_dir_with_prefix()` that clean up
automatically, even after panics:

```rust
use rvtest::fs::temp_dir;

#[test]
fn test_with_rvtest_tempdir() {
    let dir = temp_dir();
    let path = dir.path().join("test.txt");
    std::fs::write(&path, b"data").unwrap();
    assert!(path.exists());
    // dir is deleted on drop, even if a panic occurs before that
}
```

Use `temp_dir_with_prefix("myapp")` for descriptive directory names that
help with debugging failed test artifacts.

---

## Programmatic Runner Configuration

When you use `rvtest`'s library API (not the CLI), you can configure the
test runner programmatically with `RunnerConfig`:

```rust
use rvtest::core::RunnerConfig;
use rvtest::spec::describe;

#[test]
fn with_custom_config() {
    let config = RunnerConfig::default()
        .with_filter("arithmetic")
        .with_verbose(true)
        .with_fail_fast(true)
        .with_shuffle(true)
        .with_seed(42);

    let run = describe("Calculator")
        .it("adds", || assert_eq!(2 + 2, 4))
        .run_with_config(&config);
    // ...
}
```

### Config Presets

Use built-in presets for common scenarios:

```rust
// CI preset: JUnit output, fail-fast
let ci_config = RunnerConfig::ci();

// Development preset: pretty output, verbose
let dev_config = RunnerConfig::dev();
```

### Config from TOML File

Load configuration from a file:

```rust
let config = RunnerConfig::default()
    .with_config_file("rvtest.toml")
    .expect("failed to load config");
```

Example `rvtest.toml`:

```toml
filter = "auth"
format = "compact"
verbose = true
fail_fast = true
shuffle = true
```

The file is **opt-in** (not auto-discovered) — you explicitly call
`with_config_file()` to load it.

### Ignoring Cleanup on Failure

```rust
// ❌ This temp file is never cleaned up if the assertion fails
#[test]
 fn leaky_test() {
     let path = "/tmp/test.txt";
     std::fs::write(path, "data").unwrap();
     assert!(false); // Panics -> file is never deleted
 }
```

Use `TempDir` or RAII guards that clean up on drop regardless of panics.

---

## Summary

| Pattern | Best For |
|---------|----------|
| Helper function | Simple, stateless setup |
| `before_all` / `after_all` | One-time setup (database migrations) |
| `before_each` / `after_each` | Per-test reset (clear state) |
| `with_env_var` | Temporary environment variable changes |
| `TempDir` | Isolated file system access |
| `Arc` + hooks | Shared state between hooks and tests |
| RAII guards | Guaranteed cleanup on test failure |

In the next chapter, we will move from setup logic into test doubles — stubs,
spies, and mocks — and understand when each is appropriate.

---

[← Previous](08-parametrized-tests.md) • [Index](00-index.md) • [Next →](10-test-doubles.md)
