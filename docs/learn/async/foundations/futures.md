# Futures

The foundation of async Rust — what futures are and how they work.

## Prerequisites

- [Traits](../../rust/basics/traits.md) — trait definitions and implementations
- Basic threading concepts

## The Future Trait

```rust
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

A future represents a value that may not be ready yet. Calling `poll` advances it:

- `Poll::Ready(value)` — the future is complete
- `Poll::Pending` — not ready yet, wake me when I can make progress

## How Polling Works

```rust
let mut future = read_file("data.txt");

match Pin::new(&mut future).poll(&mut cx) {
    Poll::Ready(content) => println!("{content}"),
    Poll::Pending => {
        // The future registered a waker; we'll be called again
    }
}
```

## Combining Futures

```rust
use std::future::Future;

// Sequential — wait for one, then the other
async fn sequential() {
    let a = fetch_user(1).await;
    let b = fetch_user(2).await;
}

// Concurrent — both start, then wait for both
async fn concurrent() {
    let (a, b) = futures::join!(
        fetch_user(1),
        fetch_user(2),
    );
}
```

## Glossarium

| Term | Definition |
|------|------------|
| Future | A value that represents an asynchronous computation. |
| Poll | The method that advances a future's state machine. |
| Waker | A callback that notifies the executor when a future can make progress. |
| Pin | A pinned reference that guarantees a value will not move in memory. |

## Next Steps

- [Async/Await](async-await.md) — writing async functions
- [Rust Book: Async](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
