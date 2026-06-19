# Concurrency

Fearless concurrency — Rust's type system catches data races at compile time.

## Prerequisites

- [Ownership](../basics/ownership.md) — ownership rules and moves
- [Traits](../basics/traits.md) — understanding trait bounds
- [Collections](../collections/collections.md) — `Vec`, `HashMap`


## Spawning Threads

```rust
use std::thread;
use std::time::Duration;

let handle = thread::spawn(|| {
    for i in 1..10 {
        println!("thread: {i}");
        thread::sleep(Duration::from_millis(1));
    }
});

for i in 1..5 {
    println!("main: {i}");
    thread::sleep(Duration::from_millis(1));
}

handle.join().unwrap(); // wait for thread to finish
```

## Shared State with `Arc<Mutex<T>>`

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    handles.push(thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    }));
}

for handle in handles {
    handle.join().unwrap();
}

println!("result: {}", *counter.lock().unwrap()); // 10
```

## Channels (`mpsc`)

```rust
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    let vals = vec![1, 2, 3];
    for v in vals {
        tx.send(v).unwrap();
    }
});

for received in rx {
    println!("got: {received}");
}
```

Multiple producers:

```rust
let (tx, rx) = mpsc::channel();

for i in 0..3 {
    let tx = tx.clone();
    thread::spawn(move || {
        tx.send(i).unwrap();
    });
}
// rx collects from all producers
```

## `Send` and `Sync`

```rust
// Most types are Send:
//   i32 ✅, String ✅, Arc<T> ✅
//   Rc<T> ❌ (not thread-safe)

// Most types are Sync:
//   i32 ✅, Mutex<T> ✅
//   RefCell<T> ❌ (not thread-safe)

// These traits are automatically derived by the compiler.
```

## Atomic Types

For simple shared counters without a mutex:

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

let counter = AtomicUsize::new(0);
let mut handles = vec![];

for _ in 0..10 {
    let counter = &counter;
    handles.push(thread::spawn(move || {
        counter.fetch_add(1, Ordering::SeqCst);
    }));
}

// ...
```

## Which Concurrency Primitive?

| Scenario | Solution |
|----------|----------|
| One-time computation | `thread::spawn` + `join` |
| Shared mutable state | `Arc<Mutex<T>>` |
| Many readers, few writers | `Arc<RwLock<T>>` |
| Message passing | `mpsc::channel` |
| Simple counter / flag | `AtomicUsize`, `AtomicBool`, etc. |
| Data-parallel iteration | `rayon` crate (`.par_iter()`) |

## Glossarium

| Term | Definition |
|------|------------|
| Thread | An OS thread (`std::thread::spawn`). Rust threads are 1:1 with OS threads. |
| `Send` | A trait for types that can be transferred across threads. Most types are `Send`. |
| `Sync` | A trait for types that can be shared across threads via reference. |
| `Arc<T>` | Atomically Reference Counted — thread-safe shared ownership. |
| `Mutex<T>` | Mutual exclusion — ensures only one thread accesses data at a time. |
| `RwLock<T>` | Reader-writer lock — multiple readers or one writer. |
| `mpsc` | Multiple Producer, Single Consumer channel. |
| `Atomic*` | Lock-free concurrent primitives (`AtomicBool`, `AtomicUsize`, etc.). |


## Next Steps

- [Modules and Packages](../project-structure/modules-and-packages.md) — organizing Rust projects
- The Rust Book: [Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
