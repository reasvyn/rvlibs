# Data Types

Rust's type system — primitives, type inference, and type aliases.

## Prerequisites

- [Variables](variables.md) — `let`, `mut`, `const`

## Integers

| Length | Signed | Unsigned |
|--------|--------|----------|
| 8-bit | `i8` | `u8` |
| 16-bit | `i16` | `u16` |
| 32-bit | `i32` | `u32` |
| 64-bit | `i64` | `u64` |
| 128-bit | `i128` | `u128` |
| arch | `isize` | `usize` |

Default integer type is `i32`.

```rust
let decimal = 98_222;        // i32
let hex = 0xff;              // u8
let octal = 0o77;            // u16
let binary = 0b1111_0000;    // u8
let byte = b'A';             // u8
```

## Floats

```rust
let x = 2.0;      // f64 (default)
let y: f32 = 3.0; // f32
```

## Booleans

```rust
let t = true;
let f: bool = false;
```

## Characters

```rust
let c = 'z';
let emoji = '🦀';
```

`char` is 4 bytes and represents a Unicode scalar value.

## Type Inference

Rust can often infer types based on usage:

```rust
let x = 42;                // i32 (inferred)
let y = 3.14;              // f64 (inferred)
let mut v = Vec::new();    // Vec<T> — T inferred later
v.push(1);                  // now v is Vec<i32>
```

## Type Aliases

```rust
type Kilometres = i32;
let distance: Kilometres = 100;
```

## Glossarium

| Term | Definition |
|------|------------|
| Primitive | A built-in type provided by the language itself. |
| Type Inference | The compiler's ability to deduce types from usage context. |
| Type Alias | An alternative name for an existing type, created with `type`. |

## Next Steps

- [Functions](functions.md) — defining and calling functions
- [Rust Book: Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)
