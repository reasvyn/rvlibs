# Testing Concurrent Code

Testing multithreaded code, shared state, and async runtimes.

## Prerequisites

- [Architecture Tests](architecture-tests.md) — test structure
- Rust concurrency basics — threads, `Arc<Mutex<T>>`, channels

## Glossarium

| Term | Definition |
|------|------------|
| Thread Safety | Code that works correctly when accessed from multiple threads. |
| Data Race | Two threads accessing the same memory without synchronisation, at least one writing. |
| Race Condition | A bug where the outcome depends on the timing of events. |
| `loom` | A tool for testing concurrent Rust code with model checking (external crate). |

## Testing Thread Safety

Rust prevents data races at compile time through `Send` and `Sync`. But race conditions (logic bugs) still need testing:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

#[test]
fn concurrent_counter() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut num = c.lock().unwrap();
            *num += 1;
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    assert_eq!(*counter.lock().unwrap(), 10);
}
```

## Testing Channels

```rust
use std::sync::mpsc;
use std::thread;

#[test]
fn channel_message_passing() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(42).unwrap();
    });

    let received = rx.recv().unwrap();
    assert_eq!(received, 42);
}
```

## Testing with rvtest

```rust
use rvtest::spec::describe;
use std::sync::{Arc, Mutex};

#[test]
fn concurrent_specs() {
    describe("Concurrent Counter")
        .it("handles concurrent increments", || {
            let counter = Arc::new(Mutex::new(0));
            let mut handles = vec![];
            for _ in 0..10 {
                let c = Arc::clone(&counter);
                handles.push(std::thread::spawn(move || {
                    *c.lock().unwrap() += 1;
                }));
            }
            for h in handles { h.join().unwrap(); }
            assert_eq!(*counter.lock().unwrap(), 10);
        })
        .tag("concurrent")
        .run()
        .assert_all_pass();
}
```

## Common Pitfalls

| Issue | Symptom | Fix |
|-------|---------|-----|
| No synchronisation | Intermittent failures | Use `Mutex`, `RwLock`, or atomics |
| Deadlock | Tests hang | Consistent lock ordering, timeout |
| Thread leak | Process doesn't exit | `.join()` all threads, use scoped threads |
| Shared state mutation | Non-deterministic results | Isolate state per test, reset in hooks |

## Next Steps

- [Faster Feedback](../workflow/faster-feedback.md) — speeding up the test cycle
- [CI Integration](../workflow/ci-integration.md) — running tests in CI pipelines
