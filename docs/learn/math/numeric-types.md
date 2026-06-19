# Numeric Types

rvmath's core type system — `Num<T>`, the `Numeric` trait, `Percentage`, and comparison utilities.

## Prerequisites

- Basic Rust — generics, traits


## Using `Num<T>`

```rust
use rvmath::num::Num;

let a = Num::new(3.0);
let b = Num::new(4.0);
let c = a + b;
assert!((c.value - 7.0).abs() < 1e-10);
```

## The `Numeric` Trait

```rust
use rvmath::num::Numeric;

fn double<T: Numeric>(x: T) -> T {
    x + x
}

assert_eq!(double(5.0), 10.0);
```

Built-in operations:
- Arithmetic: `+`, `-`, `*`, `/`, `%`
- Roots: `sqrt()`, `cbrt()`
- Trigonometry: `sin()`, `cos()`, `tan()`, `asin()`, `acos()`, `atan()`
- Logarithms: `ln()`, `log10()`, `log(base)`
- Rounding: `round()`, `floor()`, `ceil()`, `abs()`

## `Percentage`

```rust
use rvmath::Percentage;

let base = Num::new(100.0);
let pct = Percentage::new(10.0);
let result = base + pct; // 100 + 10% = 110
```

## Comparison

```rust
use rvmath::num::Num;

let a = Num::new(3.14);
let b = Num::new(3.14);
assert!(a.approx_eq(&b, 1e-6));
```

## Glossarium

| Term | Definition |
|------|------------|
| `Num<T>` | A wrapper around any numeric type that adds math operations and `Numeric` trait support. |
| `Numeric` | The core trait that all numeric types must implement. Provides `add`, `sub`, `mul`, `div`, `sqrt`, trigonometric functions, and more. |
| `Percentage` | A dedicated type for percentage-based arithmetic (e.g., `100 m + 10% = 110 m`). |
| `Meta` | An enum describing the metadata/flavour of a numeric type (real, complex, percentage, etc.). |


## Next Steps

- [Units](units.md) — dimensional analysis with type-safe units
- [Algebra](algebra.md) — symbolic expressions and simplification
