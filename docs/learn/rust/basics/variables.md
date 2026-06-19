# Variables

Binding values to names — `let`, `mut`, `const`, `static`, and shadowing.

## Prerequisites

- Basic Rust syntax — `fn main()`, `println!`

## let Bindings

```rust
let x = 5;          // immutable (default)
let mut y = 10;     // mutable
y += 5;             // ✅
// x += 1;          // ❌ ERROR: cannot mutate immutable variable
```

## Mutability

Variables are immutable by default — one of Rust's core safety guarantees.

```rust
let mut counter = 0;
counter += 1;       // ✅ explicit mut
```

## Constants

```rust
const MAX_POINTS: u32 = 100_000;
const PI: f64 = 3.14159265359;
```

- Always immutable
- Type annotation is required
- Can be declared at any scope, including global
- Inlined at compile time

## Shadowing

```rust
let x = 5;
let x = x + 1;    // shadows previous x
let x = "hello";  // type changed! — reuse name with different type
```

Shadowing is different from `mut`:
- Creates a new binding, doesn't mutate the original
- Can change type
- Original binding is still valid until shadowed

## Glossarium

| Term | Definition |
|------|------------|
| Binding | Associating a value with a name using `let`. |
| Mutability | Whether a variable's value can change after creation. |
| Shadowing | Creating a new variable with the same name as a previous one. |
| Constant | A value bound with `const` that is inlined at compile time. |

## Next Steps

- [Data Types](data-types.md) — integers, floats, booleans, characters
- [Rust Book: Variables](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
