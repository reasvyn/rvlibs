# Naming Conventions

Rust API naming conventions — consistent, predictable, and idiomatic naming.

## Prerequisites

- Basic Rust — functions, types, traits

## Core Rules

| Category | Convention | Example |
|----------|------------|---------|
| Types | `PascalCase` | `Config`, `HttpResponse` |
| Traits | `PascalCase` | `Display`, `Iterator` |
| Functions | `snake_case` | `parse_config`, `read_file` |
| Methods | `snake_case` | `.to_string()`, `.as_ref()` |
| Constants | `SCREAMING_SNAKE` | `MAX_SIZE`, `PI` |
| Statics | `SCREAMING_SNAKE` | `GLOBAL_CONFIG` |
| Modules | `snake_case` | `std::collections` |
| Macros | `snake_case!` | `println!`, `vec!` |

## Conversion Methods

| Prefix | Semantics | Example |
|--------|-----------|---------|
| `from_` | Convert from another type (consuming) | `String::from_utf8_lossy()` |
| `to_` | Convert to another type (costly copy) | `.to_string()` |
| `as_` | Convert to another type (borrowed) | `.as_str()` |
| `into_` | Convert to another type (consuming) | `.into_bytes()` |

## Getter/Setter

Rust does **not** use `get_`/`set_` prefixes:

```rust
// ✅ Idiomatic
impl Config {
    pub fn host(&self) -> &str { &self.host }
    pub fn set_host(&mut self, host: String) { self.host = host; }
}

// ❌ Not idiomatic
impl Config {
    pub fn get_host(&self) -> &str { &self.host }
    pub fn set_host(&mut self, host: String) { self.host = host; }
}
```

## Glossarium

| Term | Definition |
|------|------------|
| Conversion | A method that changes a value from one type to another. |
| Getter | A method that returns a reference to a field. |
| Setter | A method that modifies a field. |

## Next Steps

- [Ergonomics](ergonomics.md) — designing discoverable APIs
- [Rust API Guidelines: Naming](https://rust-lang.github.io/api-guidelines/naming.html)
