# Modules

Organising code with modules — `mod`, `use`, `pub`, visibility, and file hierarchy.

## Prerequisites

- [Structs](structs.md) — custom types
- [Enums](enums.md) — enum types

## Module Declaration

```rust
// src/lib.rs
mod math;          // looks for src/math.rs or src/math/mod.rs
pub use math::add; // re-export

// src/math.rs
pub fn add(a: i32, b: i32) -> i32 { a + b }
fn internal_helper() {} // private — not accessible outside math
```

## Visibility

| Keyword | Accessible From |
|---------|-----------------|
| (none) | Current module and descendants |
| `pub` | Any code that can see the module |
| `pub(crate)` | Within the same crate |
| `pub(super)` | Parent module |
| `pub(in path)` | Specific module path |

## The `use` Keyword

```rust
use std::collections::HashMap;       // absolute path
use crate::math::add;                // crate-relative path
use super::config::load;             // parent-relative path

use std::collections::{HashMap, HashSet}; // nested
use std::io::{self, Read};               // self = std::io

// Renaming
use std::io::Result as IoResult;
```

## File Hierarchy

```
src/
├── lib.rs              // crate root
├── main.rs             // binary root
├── math/
│   ├── mod.rs          // math module root
│   ├── algebra.rs      // math::algebra
│   └── geometry.rs     // math::geometry
└── utils.rs            // utils module
```

## Glossarium

| Term | Definition |
|------|------------|
| Module | A namespace that organises code within a crate. |
| Crate | The smallest compilation unit — a library or binary. |
| Re-export | Making an item available at a different path with `pub use`. |
| Path | A way to refer to an item — `crate::module::fn`. |

## Next Steps

- [Ownership](ownership.md) — Rust's core memory management model
- [Rust Book: Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
