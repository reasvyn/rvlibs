# Algebra

Symbolic algebra — expression parsing, simplification, differentiation, and evaluation.

## Prerequisites

- [Numeric Types](numeric-types.md) — `Num<T>`, basic operations


## Parsing and Simplifying

```rust
use rvmath::algebra;

let expr = algebra::simplify("2*x + 3*x + 4").unwrap();
// Result: "5*x+4"
```

## Symbolic Differentiation

```rust
let derivative = algebra::derivative("x^3 + 2*x^2", "x").unwrap();
// Result: "(3*x^2+4*x)"
```

Supports power, product, quotient, and chain rules. Multi-variable: automatically differentiates with respect to the specified variable.

## Evaluation

```rust
use rvmath::num::Num;

let result = algebra::resolve("2*x + 5*y", &[
    ("x", Num::new(4.0)),
    ("y", Num::new(6.0)),
]).unwrap();
assert!((result.value - 38.0).abs() < 1e-10);
```

## Batch Evaluation

```rust
let inputs = vec![Num::new(1.0), Num::new(2.0), Num::new(3.0)];
let results = algebra::map_resolve("x^2 + 1", "x", &inputs).unwrap();
// Returns [2.0, 5.0, 10.0]
```

## Supported Functions

The expression evaluator supports 30+ math functions: `sqrt`, `cbrt`, `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `atan2`, `sinh`, `cosh`, `tanh`, `ln`, `log10`, `log`, `exp`, `abs`, `round`, `floor`, `ceil`, `min`, `max`, and more.

## Glossarium

| Term | Definition |
|------|------------|
| Expr | The symbolic expression enum representing mathematical expressions as trees. |
| simplify | Reduce an expression to its simplest form. |
| derivative | Compute the symbolic derivative of an expression. |
| rationalize | Rationalise the denominator of a fractional expression. |
| resolve | Evaluate an expression by substituting variable values. |
| map_resolve | Batch-evaluate an expression across multiple input values. |


## Next Steps

- [Linear Algebra](linear-algebra.md) — vectors, matrices, and tensors
- [Calculus](calculus.md) — derivatives and integrals
