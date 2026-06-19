# Result and Option

Rust's two core error handling types — recoverable errors with `Result` and optional values with `Option`.

## Prerequisites

- [Traits](../../rust/basics/traits.md) — trait basics
- [Error Handling](../../rust/basics/error-handling.md) — Rust error handling foundations

## Option

```rust
fn find_user(id: u32) -> Option<String> {
    if id == 42 {
        Some("Alice".to_string())
    } else {
        None
    }
}

// Safe access:
let name = find_user(42).unwrap_or("Guest".to_string());
```

## Result

```rust
fn read_config(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

// Propagating with ?:
fn load_config(path: &str) -> Result<Config, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("cannot read {path}: {e}"))?;
    // ... parse config ...
    Ok(config)
}
```

## Glossarium

| Term | Definition |
|------|------------|
| `Option<T>` | Either `Some(T)` (a value) or `None` (no value). |
| `Result<T, E>` | Either `Ok(T)` (success) or `Err(E)` (failure). |
| `?` Operator | Unwraps `Ok`/`Some` or returns `Err`/`None` early. |

## Next Steps

- [Custom Errors](custom-errors.md) — defining error types with `thiserror`
- [Rust Book: Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
