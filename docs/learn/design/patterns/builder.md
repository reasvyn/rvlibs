# Builder

The builder pattern for constructing complex objects with many optional parameters.

## Prerequisites

- Basic Rust — structs, methods, generics

## The Problem

When a struct has many optional fields, constructors become unwieldy:

```rust
let config = Config::new("localhost", 8080, true, None, Some(30), false);
// What does each parameter mean?
```

## The Builder Solution

```rust
#[derive(Debug)]
pub struct Config {
    host: String,
    port: u16,
    tls: bool,
    cert_path: Option<String>,
    timeout_secs: u64,
}

pub struct ConfigBuilder {
    host: String,
    port: u16,
    tls: bool,
    cert_path: Option<String>,
    timeout_secs: u64,
}

impl ConfigBuilder {
    pub fn new(host: impl Into<String>) -> Self {
        Self {
            host: host.into(),
            port: 8080,
            tls: false,
            cert_path: None,
            timeout_secs: 30,
        }
    }

    pub fn port(mut self, port: u16) -> Self { self.port = port; self }
    pub fn tls(mut self, enable: bool) -> Self { self.tls = enable; self }
    pub fn cert_path(mut self, path: impl Into<String>) -> Self {
        self.cert_path = Some(path.into());
        self
    }
    pub fn timeout(mut self, secs: u64) -> Self { self.timeout_secs = secs; self }

    pub fn build(self) -> Result<Config, String> {
        if self.tls && self.cert_path.is_none() {
            return Err("TLS requires a certificate path".into());
        }
        Ok(Config {
            host: self.host,
            port: self.port,
            tls: self.tls,
            cert_path: self.cert_path,
            timeout_secs: self.timeout_secs,
        })
    }
}
```

Usage:

```rust
let config = ConfigBuilder::new("localhost")
    .port(443)
    .tls(true)
    .cert_path("/etc/certs/server.pem")
    .build()
    .unwrap();
```

## Glossarium

| Term | Definition |
|------|------------|
| Builder | A creational pattern that constructs objects step by step. |
| Consuming Builder | A builder whose methods take and return `self` (ownership). |
| Validation | Builders can validate parameters in `build()`, returning `Result`. |

## Next Steps

- [Newtype](newtype.md) — type-safe wrappers
- [Rust API Guidelines: Builder](https://rust-lang.github.io/api-guidelines/type-safety.html#builders)
