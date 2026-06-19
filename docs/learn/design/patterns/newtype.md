# Newtype

Type-safe wrappers with zero runtime cost — the newtype pattern.

## Prerequisites

- Basic Rust — tuples, generics, traits

## The Pattern

Wrap a primitive type in a tuple struct to add type safety:

```rust
struct Meters(f64);
struct Seconds(f64);

// ⚠️ Without newtypes, this compiles and is wrong:
fn travel(distance: f64, time: f64) -> f64 { distance / time }
let speed = travel(10.0, 5.0); // What are the units?

// ✅ With newtypes, the compiler catches mistakes:
fn speed(distance: Meters, time: Seconds) -> f64 {
    distance.0 / time.0
}
// speed(Seconds(10.0), Meters(5.0)); // ❌ Type error!
```

## Deriving Traits

Newtypes can delegate trait implementations to their inner type:

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
struct Meters(f64);

impl std::ops::Add for Meters {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Meters(self.0 + other.0)
    }
}
```

Or use the `derive_more` crate for automatic delegation.

## Glossarium

| Term | Definition |
|------|------------|
| Newtype | A tuple struct with a single field, used for type safety. |
| Zero-cost Abstraction | An abstraction that has no runtime overhead — compiles to the same code as the raw type. |

## Next Steps

- [RAII and Drop](raii-drop.md) — resource management
- [Rust Book: Newtype](https://doc.rust-lang.org/book/ch19-04-advanced-types.html#using-the-newtype-pattern-for-type-safety-and-abstraction)
