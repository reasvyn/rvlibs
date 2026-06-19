# Ergonomics

Designing Rust APIs that are discoverable, consistent, and pleasant to use.

## Prerequisites

- [Naming](naming.md) — naming conventions

## Principle: Readable Over Terse

```rust
// ✅ Clear — intent is obvious
let result = config.merge(other_config)?;

// ❌ Cryptic — what does merge_into do?
let result = config.merge_into(other_config)?;
```

## Consume or Borrow?

| Pattern | When to Use |
|---------|-------------|
| `fn foo(&self)` | Read-only access |
| `fn foo(&mut self)` | Mutation |
| `fn foo(self)` | Consuming — the value is no longer usable after |
| `fn foo(self) -> Self` | Builder chain |

## Into Parameters

Accept `impl Into<T>` for owned parameters to give callers flexibility:

```rust
// ✅ Callers can pass &str or String
fn set_host(&mut self, host: impl Into<String>) {
    self.host = host.into();
}

// Both work:
config.set_host("localhost");
config.set_host(String::from("localhost"));
```

## Defaults

Provide sensible defaults with `Default`:

```rust
#[derive(Default)]
struct Config {
    host: String,
    port: u16,
    timeout: u64,
}

// Users only override what they need
let config = Config {
    port: 443,
    ..Default::default()
};
```

## Glossarium

| Term | Definition |
|------|------------|
| Ergonomics | The quality of being easy and pleasant to use. |
| `impl Into<T>` | A generic parameter that accepts any type that can convert into `T`. |
| Default | A trait providing a default value for a type. |

## Next Steps

- [Result and Option](../error-handling/result-option.md) — idiomatic error handling
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
