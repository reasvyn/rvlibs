# RAII and Drop

Resource Acquisition Is Initialisation — Rust's deterministic cleanup mechanism.

## Prerequisites

- [Ownership](../../rust/basics/ownership.md) — ownership and scope
- [Newtype](newtype.md) — type wrappers

## The Pattern

Resources (memory, file handles, locks) are acquired during initialisation and released automatically when the value goes out of scope:

```rust
struct DatabaseConnection {
    connected: bool,
}

impl DatabaseConnection {
    fn open(url: &str) -> Self {
        println!("connecting to {url}...");
        DatabaseConnection { connected: true }
    }
}

impl Drop for DatabaseConnection {
    fn drop(&mut self) {
        println!("closing connection...");
        self.connected = false;
    }
}

fn main() {
    let db = DatabaseConnection::open("postgres://localhost/db");
    // ... use db ...
} // db.drop() is called automatically here
```

## The Drop Trait

```rust
impl Drop for MyType {
    fn drop(&mut self) {
        // cleanup code — runs when the value goes out of scope
    }
}
```

## RAII Guards

A common pattern: return a guard value that performs cleanup on drop:

```rust
struct TempFile {
    path: std::path::PathBuf,
}

impl TempFile {
    fn new(name: &str) -> std::io::Result<Self> {
        let path = std::env::temp_dir().join(name);
        std::fs::write(&path, b"")?;
        Ok(TempFile { path })
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}
```

## Glossarium

| Term | Definition |
|------|------------|
| RAII | Resource Acquisition Is Initialisation — bind resource lifetime to object lifetime. |
| Drop | Rust's trait for cleanup code run when a value goes out of scope. |
| Guard | An object that performs an action when dropped (e.g., unlocking a mutex). |

## Next Steps

- [Result and Option](../error-handling/result-option.md) — error handling patterns
- [Rust Book: Drop](https://doc.rust-lang.org/book/ch15-03-drop.html)
