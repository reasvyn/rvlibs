# Design Philosophy

## Type Safety Without Sacrificing Ergonomics

rvmath's primary goal is catching errors at compile time rather than runtime. This is most visible in the unit system, where adding incompatible dimensions is a type error, and in the `Numeric` trait, which ensures all mathematical operations are available on any numeric type.

At the same time, the API is designed to be ergonomic. The `prelude` provides one-line access to common types. Operators (`+`, `-`, `*`, `/`) work naturally. The `from_*` constructors and `From` trait implementations mean you rarely need explicit conversions.

## Generic by Default

Functions are written generically over the `Numeric` trait unless there is a specific reason not to. This means:
- A single `sphere_volume` function works with `f32`, `f64`, `Num<f64>`, `i32`, and custom numeric types.
- Calculus operations are not limited to `f64` — they work with any type supporting the required math.
- Users can extend the system by implementing `Numeric` on their own types.

The cost is slightly more complex type signatures, but the gain in reusability is substantial.

## Composability Over Monoliths

Rather than providing a single "number" type that does everything, rvmath provides composable building blocks:
- `Num<T>` adds math operations to any primitive
- `Unit<N, T>` adds dimensions to any numeric type
- `Percentage<T>` adds percentage semantics
- `VecN<T, N>` adds spatial semantics

These can be combined: `VecN<Unit<Num<f64>, Meter>, 3>` is a 3D vector of unit-aware, generic numeric values.

## Zero-Cost Abstractions

rvmath uses Rust's type system to enforce invariants without runtime overhead:
- `VecN<T, N>` uses const generics for fixed-size storage on the stack
- `MatN<T, R, C>` is a fixed-size 2D array — no heap allocation
- The `Numeric` trait uses static dispatch through generics (no `dyn` trait objects)
- Unit conversions are simple multiplications — no lookup tables or string matching

## Predictable Behavior

Functions follow the principle of least surprise:
- Invalid mathematical operations return `NaN` rather than panicking
- Unit dimension mismatches panic at runtime with a clear message (type system can't track all cases)
- All algebraic functions return `Result` with descriptive error messages
- Operator precedence follows standard mathematical conventions

## No Unsafe Code

The entire library is written in safe Rust. No `unsafe` blocks, no raw pointer manipulation, no transmutes. This is a deliberate choice to maintain safety guarantees and make the library suitable for security-sensitive applications.

## Minimal Dependencies

rvmath has a single optional dependency (`serde` for serialization). All mathematical operations are implemented directly rather than delegating to external crates. This keeps compile times fast and avoids version conflicts.

## Why Not Use `num-traits` / `nalgebra` / `symbolic-expressions`?

rvmath is designed as a cohesive, opinionated library rather than a wrapper around existing crates:

- The `Numeric` trait includes methods (`pi()`, `lerp()`, `map_range()`) that go beyond `num-traits`.
- The unit system is deeply integrated with the numeric type system, not an add-on.
- The symbolic algebra module is tailored for educational and scientific use cases with a focus on string-based input/output.
- Having all functionality in one crate ensures consistent design patterns and compatible versioning.
