# Chapter 20 — Testing Concurrent Code

[← Previous](19-architecture-tests.md) • [Index](00-index.md) • [Next →](21-faster-feedback-loops.md)

---

Concurrent code is notoriously difficult to test.  Race conditions, deadlocks,
and non-deterministic behaviour can hide in your code for months before
manifesting in production.  This chapter covers techniques for testing
multi-threaded and async code in Rust.

---

## The Challenge of Concurrent Testing

Concurrent tests fail intermittently because they depend on thread scheduling,
which the operating system controls.  A test that passes 99 times out of 100
on your machine might fail 50% of the time on a CI runner with different
hardware.

```rust
// This test looks correct but is flaky
#[test]
 fn test_concurrent_increment() {
     let counter = std::sync::Arc::new(std::sync::Mutex::new(0));
     let mut handles = vec![];

     for _ in 0..10 {
         let c = counter.clone();
         handles.push(std::thread::spawn(move || {
             let mut val = c.lock().unwrap();
             *val += 1;
         }));
     }

     for h in handles { h.join().unwrap(); }

     assert_eq!(*counter.lock().unwrap(), 10);
 }
```

This test is actually correct — `Mutex` ensures exclusive access.  But a test
that depends on timing (calling `thread::sleep` and expecting something to
happen) would be flaky.

---

## Testing Thread Safety

Rust's type system prevents data races at compile time.  If your code
compiles without `unsafe`, it is free of data races.  But it can still have
**logic races** — incorrect behaviour under concurrent execution.

### Test with `loom`

The `loom` crate is a permutation-testing tool for concurrent Rust code.  It
systematically explores all possible thread interleavings:

```toml
[dev-dependencies]
loom = "0.7"
```

```rust
use loom::sync::Arc;
use loom::thread;

#[test]
 fn test_concurrent_increment_with_loom() {
     loom::model(|| {
         let counter = Arc::new(std::sync::Mutex::new(0));
         let mut handles = vec![];

         for _ in 0..2 {
             let c = counter.clone();
             handles.push(thread::spawn(move || {
                 let mut val = c.lock().unwrap();
                 *val += 1;
             }));
         }

         for h in handles { h.join().unwrap(); }

         assert_eq!(*counter.lock().unwrap(), 2);
     });
 }
```

`loom` explores all possible thread interleavings, ensuring the test is
correct under all schedules.

---

## Testing Channels

When using channels, test both the send and receive paths:

```rust
use std::sync::mpsc;

#[test]
 fn channel_sends_and_receives() {
     let (tx, rx) = mpsc::channel();

     std::thread::spawn(move || {
         tx.send(42).unwrap();
     });

     assert_eq!(rx.recv().unwrap(), 42);
 }

 #[test]
 fn channel_multiple_messages() {
     let (tx, rx) = mpsc::channel();

     std::thread::spawn(move || {
         for i in 0..5 {
             tx.send(i).unwrap();
         }
     });

     let received: Vec<_> = rx.iter().take(5).collect();
     assert_eq!(received, vec![0, 1, 2, 3, 4]);
 }

 #[test]
 fn channel_closes_when_sender_drops() {
     let (tx, rx) = mpsc::channel::<i32>();

     std::thread::spawn(move || {
         tx.send(1).unwrap();
         // tx is dropped here
     });

     assert_eq!(rx.recv().unwrap(), 1);
     assert_eq!(rx.recv(), Err(mpsc::RecvError)); // Channel closed
 }
```

---

## Testing `Arc<Mutex<T>>` Patterns

When sharing state with `Arc<Mutex<T>>`, test that:

1. The shared state is correctly initialised
2. Operations on the shared state are atomic
3. No deadlocks occur

```rust
use std::sync::{Arc, Mutex};

struct SharedCounter {
    inner: Arc<Mutex<u64>>,
}

impl SharedCounter {
    fn new() -> Self {
        SharedCounter { inner: Arc::new(Mutex::new(0)) }
    }

    fn increment(&self) {
        let mut val = self.inner.lock().unwrap();
        *val += 1;
    }

    fn value(&self) -> u64 {
        *self.inner.lock().unwrap()
    }
}

#[test]
 fn shared_counter_starts_at_zero() {
     let counter = SharedCounter::new();
     assert_eq!(counter.value(), 0);
 }

 #[test]
 fn shared_counter_increments() {
     let counter = SharedCounter::new();
     counter.increment();
     counter.increment();
     counter.increment();
     assert_eq!(counter.value(), 3);
 }
```

---

## Testing `RwLock<T>` Patterns

`RwLock` allows concurrent reads but exclusive writes.  Test both paths:

```rust
use std::sync::{Arc, RwLock};

#[test]
 fn rwlock_allows_concurrent_reads() {
     let data = Arc::new(RwLock::new(42));
     let mut handles = vec![];

     for _ in 0..10 {
         let d = data.clone();
         handles.push(std::thread::spawn(move || {
             let read = d.read().unwrap();
             assert_eq!(*read, 42);
         }));
     }

     for h in handles { h.join().unwrap(); }
 }

 #[test]
 fn rwlock_exclusive_write() {
     let data = Arc::new(RwLock::new(0));
     let d = data.clone();

     std::thread::spawn(move || {
         let mut write = d.write().unwrap();
         *write = 99;
     }).join().unwrap();

     assert_eq!(*data.read().unwrap(), 99);
 }
```

---

## Using Barriers for Deterministic Concurrency

`Barrier` synchronises threads at a specific point, making concurrent tests
deterministic:

```rust
use std::sync::{Arc, Barrier};

#[test]
 fn barrier_synchronises_threads() {
     let barrier = Arc::new(Barrier::new(3));
     let mut handles = vec![];

     for id in 0..3 {
         let b = barrier.clone();
         handles.push(std::thread::spawn(move || {
             // Simulate some work
             std::thread::sleep(std::time::Duration::from_millis(id * 10));
             // Wait for all threads to reach this point
             b.wait();
             // All threads are now guaranteed to be here
         }));
     }

     for h in handles { h.join().unwrap(); }
 }
```

---

## Testing `Condvar` (Condition Variables)

`Condvar` allows threads to wait for a condition.  Test the notification
pattern:

```rust
use std::sync::{Arc, Condvar, Mutex};

#[test]
 fn condvar_notifies_waiting_thread() {
     let pair = Arc::new((Mutex::new(false), Condvar::new()));
     let pair2 = pair.clone();

     let waiter = std::thread::spawn(move || {
         let (lock, cvar) = &*pair2;
         let mut ready = lock.lock().unwrap();
         while !*ready {
             ready = cvar.wait(ready).unwrap();
         }
     });

     std::thread::sleep(std::time::Duration::from_millis(10));

     let (lock, cvar) = &*pair;
     let mut ready = lock.lock().unwrap();
     *ready = true;
     cvar.notify_one();

     waiter.join().unwrap();
 }
```

---

## Testing `Atomic` Types

Atomic operations are lock-free and inherently thread-safe.  Test the
expected values after concurrent access:

```rust
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

#[test]
 fn atomic_increment() {
     let counter = Arc::new(AtomicU64::new(0));
     let mut handles = vec![];

     for _ in 0..10 {
         let c = counter.clone();
         handles.push(std::thread::spawn(move || {
             c.fetch_add(1, Ordering::SeqCst);
         }));
     }

     for h in handles { h.join().unwrap(); }
     assert_eq!(counter.load(Ordering::SeqCst), 10);
 }
```

---

## Testing Async Code

Async Rust introduces additional testing challenges.  Use `tokio::test` or
`futures::executor` to run async tests:

```toml
[dev-dependencies]
tokio = { version = "1", features = ["rt", "macros"] }
```

```rust
#[tokio::test]
 async fn test_async_function() {
     let result = async_add(2, 3).await;
     assert_eq!(result, 5);
 }

 async fn async_add(a: i32, b: i32) -> i32 {
     tokio::time::sleep(std::time::Duration::from_millis(10)).await;
     a + b
 }
```

### Testing Timeouts

```rust
#[tokio::test]
 async fn test_timeout() {
     let result = tokio::time::timeout(
         std::time::Duration::from_millis(100),
         slow_operation(),
     ).await;

     assert!(result.is_ok());
 }

 async fn slow_operation() -> &'static str {
     tokio::time::sleep(std::time::Duration::from_millis(50)).await;
     "done"
 }
```

### Testing Async Cancellation

```rust
use tokio::select;

#[tokio::test]
 async fn test_cancellation() {
     let work = tokio::spawn(async {
         loop {
             // Simulate infinite work
             tokio::time::sleep(std::time::Duration::from_millis(100)).await;
         }
     });

     // Cancel after 50ms
     tokio::time::sleep(std::time::Duration::from_millis(50)).await;
     work.abort();

     assert!(work.await.unwrap_err().is_cancelled());
 }
```

---

## Deterministic Async Testing with `tokio::test`

Control time in async tests:

```rust
#[tokio::test]
 async fn test_with_controlled_time() {
     // Start the clock
     let start = tokio::time::Instant::now();

     tokio::time::sleep(std::time::Duration::from_millis(100)).await;

     let elapsed = start.elapsed();
     assert!(elapsed >= std::time::Duration::from_millis(100));
 }
```

---

## Common Concurrent Testing Mistakes

### Sleeping Instead of Synchronising

```rust
// ❌ Flaky: depends on timing
#[test]
 fn test_race_condition() {
     let flag = Arc::new(AtomicBool::new(false));
     let f = flag.clone();
     std::thread::spawn(move || {
         std::thread::sleep(Duration::from_millis(10));
         f.store(true, Ordering::SeqCst);
     });
     std::thread::sleep(Duration::from_millis(5)); // May not be enough
     assert!(flag.load(Ordering::SeqCst)); // Flaky!
 }
```

```rust
// ✅ Deterministic: use a Barrier
#[test]
 fn test_synchronised() {
     let barrier = Arc::new(Barrier::new(2));
     let flag = Arc::new(AtomicBool::new(false));
     let b = barrier.clone();
     let f = flag.clone();
     std::thread::spawn(move || {
         f.store(true, Ordering::SeqCst);
         b.wait();
     });
     b.wait();
     assert!(flag.load(Ordering::SeqCst)); // Always correct
 }
```

### Not Joining Threads

```rust
// ❌ Test may complete before the thread runs
#[test]
 fn test_thread_not_joined() {
     std::thread::spawn(|| {
         // This may never execute
         panic!("this might not cause test failure");
     });
     // Thread is detached — test passes immediately
 }
```

```rust
// ✅ Always join threads
#[test]
 fn test_thread_joined() {
     let handle = std::thread::spawn(|| {
         // This always executes
     });
     handle.join().unwrap();
 }
```

### Assuming Thread Execution Order

Never assume threads will execute in any particular order.  If you need
ordered execution, use synchronisation primitives.

---

## Summary

- Use `loom` for exhaustive exploration of thread interleavings
- Test `Mutex`, `RwLock`, `Condvar`, `Barrier`, and `Atomic` types
- Use `Barrier` for deterministic concurrent tests
- Use `tokio::test` for async Rust tests
- Test async timeouts with `tokio::time::timeout`
- Always join threads — detached threads may not complete
- Never assume thread execution order
- Sleep-based synchronisation is always flaky — use proper primitives

> **rvtest:** Use `rvtest::assert_delta!` for concurrent code where floating-point results may vary slightly between runs, and `--shuffle` to randomise test execution order and detect hidden ordering dependencies.

In the next and final chapter, we will look at techniques for getting faster
feedback from your test suite.

---

[← Previous](19-architecture-tests.md) • [Index](00-index.md) • [Next →](21-faster-feedback-loops.md)
