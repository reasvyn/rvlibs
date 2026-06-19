# Tokio

The Tokio runtime — asynchronous I/O, timers, synchronisation, and task management.

## Prerequisites

- [Executors](../foundations/executors.md) — how executors work

## The Runtime

```rust
#[tokio::main]
async fn main() {
    // Your async code here
}
```

This macro expands to:

```rust
fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        // Your async code here
    });
}
```

## Async I/O

```rust
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

async fn fetch() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("example.com:80").await?;
    stream.write_all(b"GET / HTTP/1.1\r\nHost: example.com\r\n\r\n").await?;

    let mut buf = vec![0u8; 4096];
    let n = stream.read(&mut buf).await?;
    println!("received {n} bytes");
    Ok(())
}
```

## Sync Primitives

```rust
use tokio::sync::{Mutex, mpsc, Semaphore};
use std::sync::Arc;

async fn shared_state() {
    let data = Arc::new(Mutex::new(0));

    let handle = tokio::spawn({
        let data = Arc::clone(&data);
        async move {
            let mut val = data.lock().await;
            *val += 1;
        }
    });

    handle.await.unwrap();
}
```

## Glossarium

| Term | Definition |
|------|------------|
| Tokio | An asynchronous runtime for Rust, providing I/O, timers, and sync. |
| `#[tokio::main]` | Macro that sets up the Tokio runtime and enters the async context. |
| `tokio::spawn` | Spawn a new task on the Tokio runtime. |

## Next Steps

- [Futures](../foundations/futures.md) — the foundation of async Rust
- [Tokio Documentation](https://docs.rs/tokio/latest/tokio/)
