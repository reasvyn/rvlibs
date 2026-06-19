# Custom Errors

Defining, deriving, and composing error types — `thiserror`, `Display`, and `From`.

## Prerequisites

- [Result and Option](result-option.md) — `Result`, `Option`, `?`

## Manual Error Type

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
    fn from(e: std::io::Error) -> Self { AppError::Io(e) }
}
```

## With thiserror

The `thiserror` crate reduces the boilerplate:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum AppError {
    #[error("not found: {0}")]
    NotFound(String),

    #[error("permission denied")]
    PermissionDenied,

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
```

## Glossarium

| Term | Definition |
|------|------------|
| `std::error::Error` | The trait all error types should implement. |
| `thiserror` | A derive macro for implementing `Error` with minimal boilerplate. |
| `#[error("...")]` | `thiserror` attribute for customising the `Display` message. |

## Next Steps

- [Builder](../patterns/builder.md) — validating input in constructors
- [thiserror Documentation](https://docs.rs/thiserror/latest/thiserror/)
