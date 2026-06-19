# Error Handling

Rust's approach: recoverable errors with `Result`, unrecoverable with `panic!`.

## Prerequisites

- [Traits](traits.md) — trait definitions, impl, derive


## `panic!`

```rust
fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("division by zero");
    }
    a / b
}
```

Panics are for bugs, not for expected errors. Use `Result` for recoverable failures.

## `Option<T>`

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

## `Result<T, E>`

```rust
fn read_file(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}
```

## The `?` Operator

The `?` operator is syntactic sugar for "unwrap or return early":

```rust
fn process_config(path: &str) -> Result<Config, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("cannot read {path}: {e}"))?;

    let parsed: Config = serde_json::from_str(&content)
        .map_err(|e| format!("invalid JSON: {e}"))?;

    Ok(parsed)
}
```

## Custom Error Types

```rust
use std::fmt;

#[derive(Debug)]
enum AppError {
    NotFound(String),
    PermissionDenied,
    Io(std::io::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(name) => write!(f, "not found: {name}"),
            AppError::PermissionDenied => write!(f, "permission denied"),
            AppError::Io(e) => write!(f, "I/O error: {e}"),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Io(e)
    }
}
```

With the custom error and `From` impl, `?` works automatically:

```rust
fn load_user(id: u32) -> Result<User, AppError> {
    let data = std::fs::read_to_string("users.db")?; // auto-converts via From
    // ...
}
```

## Idiomatic Error Handling

| Scenario | Approach |
|----------|----------|
| Bug / invariant violation | `panic!`, `unreachable!`, `unimplemented!` |
| Optional value | `Option<T>` with `?`, `unwrap_or`, `map` |
| Expected failure | `Result<T, E>` with `?` |
| Prototyping | `.unwrap()`, `.expect("message")` |
| Library code | Custom error type implementing `std::error::Error` |
| Binary / app code | `anyhow::Result` for simplicity |
| Library error details | `thiserror` derive macro |

## Glossarium

| Term | Definition |
|------|------------|
| `panic!` | Unrecoverable error — unwinds the stack, prints message, exits. |
| `Result<T, E>` | Recoverable error — either `Ok(T)` or `Err(E)`. |
| `Option<T>` | Optional value — either `Some(T)` or `None`. |
| `?` Operator | Propagate errors: unwraps `Ok` or returns `Err` early. |
| `unwrap` / `expect` | Extract the value or panic. Use sparingly — only when failure is impossible. |
| Error Trait | `std::error::Error` — the trait all error types should implement. |
| Type Alias | `type Result<T> = std::result::Result<T, MyError>` to avoid duplicating error types. |


## Next Steps

- [Collections](collections.md) — `Vec`, `HashMap`, `String`, and other std containers
- The Rust Book: [Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
