# Units

Type-safe dimensional analysis — declare unit families and perform dimensionally-consistent arithmetic.

## Prerequisites

- [Numeric Types](numeric-types.md) — `Num<T>`, `Numeric` trait


## Declaring a Unit Family

```rust
use rvmath::unit::{declare_family, declare_units};

declare_family!(Length, "m");
declare_units! {
    Length: Meter => 1.0,
    Kilometer => 1000.0,
    Centimeter => 0.01,
    Millimeter => 0.001,
}
```

## Using Units

```rust
use rvmath::unit::{Unit, Meter, Kilometer};

let distance = Unit::<f64, Meter>::new(100.0);
let km_distance = distance.convert_to::<Kilometer>();
assert!((km_distance.value() - 0.1).abs() < 1e-10);
```

## Dimensional Safety

```rust
// ❌ Compile error: cannot add Length and Time
let length = Unit::<f64, Meter>::new(5.0);
let time = Unit::<f64, Second>::new(3.0);
// let result = length + time; // type error!

// ✅ Same family: automatically handled
let m1 = Unit::<f64, Meter>::new(5.0);
let m2 = Unit::<f64, Meter>::new(3.0);
let sum = m1 + m2; // 8.0 meters
```

## Glossarium

| Term | Definition |
|------|------------|
| Family | A physical dimension (e.g., Length, Time, Mass). |
| Unit | A specific unit within a family (e.g., Meter, Kilometer within Length). |
| `declare_family!` | Macro to define a new dimension family. |
| `declare_units!` | Macro to define specific units within a family. |
| Dimensional Consistency | The compiler prevents adding units of different families (e.g., Meters + Seconds). |


## Next Steps

- [Algebra](algebra.md) — symbolic expressions
- [Linear Algebra](linear-algebra.md) — vectors and matrices with unit support
