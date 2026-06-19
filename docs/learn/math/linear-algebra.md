# Linear Algebra

Vectors, matrices, and tensors — the foundation of computational mathematics.

## Prerequisites

- [Numeric Types](numeric-types.md) — `Num<T>`, `Numeric` trait


## Vectors

```rust
use rvmath::la::vector::{VecN, Vec3};

let v1 = Vec3::new(1.0, 2.0, 3.0);
let v2 = Vec3::new(4.0, 5.0, 6.0);
let dot = v1.dot(&v2); // 1*4 + 2*5 + 3*6 = 32.0
```

Unit-aware dot products are also supported.

## Matrices

```rust
use rvmath::la::matrix::MatN;

let m = MatN::<f64, 3, 3>::identity();
let transposed = m.transpose();
```

Supports: addition, subtraction, multiplication (element-wise and matrix), transpose, row/column access.

## Tensors

```rust
use rvmath::la::tensor::Tensor;

let t = Tensor::<f64>::zeros(&[2, 3, 4]);
let ones = Tensor::<f64>::ones(&[5, 5]);
let reshaped = t.reshape(&[3, 8]);
```

Tensors are dynamic — shape and strides are managed at runtime.

## Glossarium

| Term | Definition |
|------|------------|
| `VecN<T, N>` | A fixed-size, compile-time dimensional vector. |
| `MatN<T, R, C>` | A fixed-size matrix with rows and columns specified at compile time. |
| `Tensor<T>` | A dynamic N-dimensional array with flexible shape. |
| Dot Product | The sum of element-wise products of two vectors. |


## Next Steps

- [Calculus](../math/calculus.md) — derivatives, integrals, numerical methods
- [Geometry](../math/geometry.md) — 2D and 3D shape formulas
