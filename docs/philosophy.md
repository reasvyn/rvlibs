# Philosophy

## Purpose & Direction

rvlibs is not a collection of libraries without purpose. It is an **ecosystem with gravity** — every crate, whether atomic or composite, has a concrete role and a clear destination: **Rveco**.

Rveco is the estuary. The main binary that unifies all crates into one creative development suite. Without Rveco, we have a pile of libraries waiting for a purpose. With Rveco, every crate knows where it is heading.

This direction does not compromise modularity. Each crate remains independent — usable alone, publishable to crates.io — but together they share a gravitational pull toward a complete application.

## Modular, Atomic, Composite

These three principles guide every design decision:

**Modular** — Each crate is focused and independent. rvmath does mathematics. rvtest does testing. rvnx is the brain. rvfx is the body. They have minimal coupling and can be developed separately.

**Atomic** — Within each crate, features are atomic building blocks. `Num<T>`, `Unit<N, T>`, `VecN<T, N>` in rvmath. `describe`/`it`, `check`, `parametrize`, `Spy` in rvtest. `Entity`, `Component`, `World` in rvnx. Each does one thing well.

**Composite** — Blocks compose into powerful combinations. `VecN<Unit<Num<f64>, Meter>, 3>` combines types. `check()` inside `describe().it()` combines testing patterns. rvnx + rvfx combine into rveco.

## Brain & Body

The ecosystem architecture is built on a biological metaphor:

- **rvnx (brain)** — Intelligence, logic, structure. Defines what the system needs (ports) without knowing how they are implemented. May use external dependencies. Not artificially constrained.
- **rvfx (body)** — Physical implementation. Renders, draws, plays audio, handles input, manages windows. Implements the ports the brain defines. Depends on rvnx, never the other way.
- **rveco (estuary)** — The living organism. Binds brain and body into a functioning application.

## Not Preemptive

New crates are only created when a concrete implementation need arises. We do not create `rvstat`, `rvphysic`, `rvui`, or any `rv*` crate speculatively. Wait until the code demands extraction. This keeps the ecosystem lean, focused, and free of unused abstractions.

Foundation crates (rvlibs, rvmath, rvtest) are the exception — they are fundamental building blocks proven by existing code.

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

Foundation crates (rvlibs, rvmath) have zero or minimal external dependencies. Higher-level crates (rvnx, rvfx) may use external crates as needed — the brain should not be artificially constrained, and the body must interface with real hardware and APIs.

## Easy to Adopt, Easy to Remove

Adding any rvlibs crate is a single `cargo add`. Removing it is equally simple — dev-only dependencies, no proc-macro magic, no framework lock-in.

## Dogfooding

rvtest tests itself with its own BDD API. Every crate in the ecosystem uses rvtest for testing. This ensures the API is ergonomic in practice and that breaking changes are caught during development.
