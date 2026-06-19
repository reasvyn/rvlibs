# Async/Await

Writing asynchronous functions — `async fn`, `.await`, and the state machine.

## Prerequisites

- [Futures](futures.md) — the `Future` trait and polling

## Async Functions

```rust
// An async function returns a Future — it doesn't run immediately
async fn fetch_data(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}
```

## Awaiting

`.await` yields control back to the executor until the future is ready:

```rust
async fn process() {
    // Task 1: runs → yields → resumes → yields → completes
    let data = fetch_data("https://api.example.com").await;

    // Task 2: runs after task 1 completes
    println!("{data}");
}
```

## The State Machine

Every `async fn` is compiled into a state machine:

```rust
// This async fn:
async fn example() -> i32 {
    let a = foo().await;
    let b = bar().await;
    a + b
}

// Becomes something like this:
enum ExampleStateMachine {
    Start,
    WaitingForFoo { /* saved state */ },
    WaitingForBar { saved_a: i32, /* saved state */ },
    Done,
}
```

## Glossarium

| Term | Definition |
|------|------------|
| `.await` | A suspension point — yields control until the future is ready. |
| State Machine | The internal enum that the compiler generates for each `async fn`. |
| Suspension | The act of pausing execution of an async function at a `.await` point. |

## Next Steps

- [Executors](executors.md) — how futures are driven to completion
- [Async Book](https://rust-lang.github.io/async-book/)
