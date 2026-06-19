# Executors

How futures are driven to completion — executors, tasks, and spawn.

## Prerequisites

- [Async/Await](async-await.md) — async functions and `.await`

## What an Executor Does

An executor polls futures to completion. It manages a queue of tasks and uses wakers to know when to poll each one.

```rust
// Pseudocode of a minimal executor
fn block_on<F: Future>(mut future: F) -> F::Output {
    let waker = create_waker(); // knows how to wake the executor
    let mut cx = Context::from_waker(&waker);

    loop {
        match unsafe { Pin::new_unchecked(&mut future) }.poll(&mut cx) {
            Poll::Ready(value) => return value,
            Poll::Pending => park(), // sleep until woken
        }
    }
}
```

## Spawning Tasks

```rust
#[tokio::main]
async fn main() {
    // Spawn a task — runs concurrently on the executor
    let handle = tokio::spawn(async {
        do_slow_work().await
    });

    // Other work happens here while the spawned task runs

    handle.await.unwrap();
}
```

## Glossarium

| Term | Definition |
|------|------------|
| Executor | A runtime that polls futures to completion. |
| Task | A spawned future that runs independently on the executor. |
| Blocking | A synchronous call that prevents the executor from making progress. |
| Cooperative | Async tasks yield control via `.await`, never preempted. |

## Next Steps

- [Tokio](../runtimes/tokio.md) — the most popular async runtime
- [Async Book: Executors](https://rust-lang.github.io/async-book/02_execution/01_chapter.html)
