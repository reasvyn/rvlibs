# Philosophy

## Modular, Atomic, Composite

These three principles guide every design decision in rvlibs.

**Modular** — Each crate is focused and independent. rvmath does mathematics. rvtest does testing. They have no coupling and can be used separately.

**Atomic** — Within each crate, features are atomic building blocks. `Num<T>`, `Unit<N, T>`, `VecN<T, N>` in rvmath. `describe`/`it`, `check`, `parametrize`, `Spy` in rvtest. Each does one thing well.

**Composite** — Blocks compose into powerful combinations. `VecN<Unit<Num<f64>, Meter>, 3>` combines types. `check()` inside `describe().it()` combines testing patterns.

## Type Safety Without Sacrificing Ergonomics

Catch errors at compile time rather than runtime. The rvmath unit system makes adding incompatible dimensions a type error. The rvtest API is explicit about what it does — no global state, no hidden registration.

At the same time, APIs are designed to be ergonomic. Preludes provide one-line access. Operators work naturally. Builder chains are readable.

## Generic by Default

Functions are written generically over traits (`Numeric`, `TestReporter`) unless there is a specific reason not to. This means:
- A single `sphere_volume` function works with `f32`, `f64`, `Num<f64>`, `i32`, and custom types.
- rvtest reporters work on any `TestRun`, regardless of how it was produced.
- Users can extend the system by implementing traits on their own types.

## Zero-Cost Abstractions

Rust's type system enforces invariants without runtime overhead:
- `VecN<T, N>` uses const generics for fixed-size stack storage.
- The `Numeric` trait uses static dispatch (no `dyn` trait objects).
- Unit conversions are simple multiplications — no lookup tables.

## No Unsafe Code

All crates are written in safe Rust. No `unsafe` blocks, no raw pointer manipulation, no transmutes. This maintains safety guarantees and makes the libraries suitable for security-sensitive applications.

## Minimal Dependencies

rvmath has one optional dependency (`serde`). rvtest keeps its dependency tree small and optional. All core functionality is implemented directly rather than delegating to external crates.

## Easy to Adopt, Easy to Remove

Adding any rvlibs crate is a single `cargo add`. Removing it is equally simple — dev-only dependencies, no proc-macro magic, no framework lock-in.

## Dogfooding

rvtest tests itself with its own BDD API. This ensures the API is ergonomic in practice and that breaking changes are caught during development. If it's awkward to use in our own tests, it will be awkward for users.
