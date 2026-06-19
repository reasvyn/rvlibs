# Geometry

Mathematical constants and formulas for 2D and 3D shapes — with unit-aware support.

## Prerequisites

- [Numeric Types](../foundations/numeric-types.md) — basic numeric operations
- [Units](../foundations/units.md) (recommended) — dimensional analysis


## Constants

```rust
use rvmath::consts::{PI, EULER_NUMBER, GOLDEN_RATIO, SQRT_2, TAU};
```

## 2D Shapes

```rust
use rvmath::geometry::shapes_2d;

let area = shapes_2d::circle_area(5.0);
let circumference = shapes_2d::circle_circumference(5.0);

let triangle = shapes_2d::triangle_area_heron(3.0, 4.0, 5.0);
```

## 3D Shapes

```rust
use rvmath::geometry::shapes_3d;

let volume = shapes_3d::sphere_volume(1.0);
let surface = shapes_3d::sphere_surface(1.0);

let cylinder = shapes_3d::cylinder_volume(2.0, 5.0);
```

All functions work with any `Numeric` type, including `Num<f64>` and unit-aware types.

## Glossarium

| Term | Definition |
|------|------------|
| PI | π — 3.141592653589793 |
| GOLDEN_RATIO | φ — 1.618033988749895 |
| Euler's Number | e — 2.718281828459045 |


## Next Steps

- [Algebra](../algebra/algebra.md) — symbolic expressions
- [Linear Algebra](../linear-algebra/linear-algebra.md) — vectors and matrices
