# Modules and Packages

Organizing Rust code — crates, modules, visibility, and workspaces.

## Prerequisites

- Basic Rust syntax and project structure


## Module System

```rust
// src/lib.rs
mod math;            // declares module, looks for src/math.rs or src/math/mod.rs
pub use math::add;   // re-export

// src/math.rs
pub fn add(a: i32, b: i32) -> i32 { a + b }
fn internal_helper() {} // private — not accessible outside math module
```

Sub-modules:

```rust
// src/math/mod.rs
mod algebra;         // src/math/algebra.rs
mod geometry;        // src/math/geometry.rs
pub use algebra::simplify;
```

## Visibility Levels

| Keyword | Accessible From |
|---------|-----------------|
| (none) | Current module and descendants |
| `pub` | Any code that can see the module |
| `pub(crate)` | Within the same crate |
| `pub(super)` | Parent module |
| `pub(in path)` | Specific module path |

```rust
pub struct Config {
    pub name: String,       // accessible everywhere
    pub(crate) version: u32, // only within this crate
    path: String,           // only within this module
}
```

## File Layout Conventions

```
src/
├── lib.rs              // crate root
├── main.rs             // binary root (if binary crate)
├── math/
│   ├── mod.rs          // math module root
│   ├── algebra.rs      // math::algebra
│   └── geometry.rs     // math::geometry
└── utils.rs            // utils module (single file)
```

## Workspaces

A workspace groups multiple packages that share one `Cargo.lock`:

```toml
# Cargo.toml (root)
[workspace]
members = [
    "crates/rvmath",
    "crates/rvtest",
    "crates/rvtest-macros",
    "crates/cargo-rvtest",
]
```

```bash
cargo build              # builds all members
cargo test --workspace   # tests all members
cargo test -p rvmath     # test specific package
```

## Paths

```rust
// Absolute path from crate root:
crate::math::algebra::simplify

// Relative path (within the same module):
self::helper::format
super::parent_fn

// External dependency:
// In Cargo.toml: serde = "1.0"
// In code:       serde::Serialize  (via the `use` keyword)
```

## Glossarium

| Term | Definition |
|------|------------|
| Crate | A compilation unit. Either a binary (has `main`) or a library (`lib.rs`). |
| Package | A `Cargo.toml` + one or more crates. Exactly one library crate, zero or more binary crates. |
| Module | A namespace within a crate. Defined by `mod` keyword. Can be in a file or inline. |
| Path | A way to refer to an item: `crate::module::fn`, `self::`, `super::`. |
| Visibility | `pub` makes an item accessible outside its module. Private by default. |
| Workspace | A set of packages that share a common `Cargo.lock` and output directory. |
| Re-export | `pub use` — makes an item available at a different path. |


## Next Steps

- [Tests](../../tests/index.md) — testing in Rust with rvtest
- Rust By Example: [Modules](https://doc.rust-lang.org/stable/rust-by-example/mod.html)
