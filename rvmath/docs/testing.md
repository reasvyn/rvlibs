# Testing

## Framework

rvmath uses [`rvtest`](https://crates.io/crates/rvtest) — a BDD-style testing framework for Rust that provides spec blocks, property-based checks, parametrized cases, rich reporting, and code coverage.

Add `rvtest` as a dev-dependency in `Cargo.toml`:

```toml
[dev-dependencies]
rvtest = "0.1"
```

## Policy

Tests should be placed in the `crates/rvmath/tests/` directory as integration test files whenever possible. This is the recommended approach because:

1. **Keeps source files smaller and focused** on the implementation.
2. **Encourages testing through the public API** — integration tests can only use exported items, ensuring the public interface is well-designed and sufficient.
3. **Simplifies build caching** — changes to tests do not trigger recompilation of library code.
4. **Provides a single location** for developers to find and understand how each module is used.

Inline unit tests (`#[cfg(test)]`) and doctests in doc comments (`///` and `//!`) are permitted but should be used sparingly — prefer `tests/` integration tests for comprehensive coverage.


## Writing Tests with rvtest

### BDD-Style Specs

Build a spec with `describe` / `it`, call `run()`, then verify with `assert_all_pass()`:

```rust
use rvtest::spec::describe;

#[test]
fn geometry_tests() {
    describe("Sphere")
        .it("computes volume correctly", || {
            let v = sphere_volume(1.0);
            assert!((v - 4.188_790_204_786_390_5).abs() < 1e-10);
        })
        .it("surface area of unit sphere is 4π", || {
            let s = sphere_surface(1.0);
            assert!((s - 12.566_370_614_359_172).abs() < 1e-10);
        })
        .tag("3d")
        .run();

    describe("Triangle")
        .it("area via Heron's formula", || {
            let a = triangle_area_heron(3.0, 4.0, 5.0);
            assert!((a - 6.0).abs() < 1e-10);
        })
        .tag("2d")
        .run()
        .assert_all_pass();
}
```

### Property-Based Testing

Use `check` with `any::<T>()` to verify invariants over randomly generated inputs:

```rust
use rvtest::property::{check, any};

#[test]
fn numeric_properties() {
    check("addition is commutative", any::<i32>(), |a: &i32| {
        let b: i32 = 42;
        a + b == b + *a
    });
}
```

### Parametrized Tests

Use `parametrize` to run the same logic against multiple input sets:

```rust
use rvtest::param::parametrize;

#[test]
fn operation_cases() {
    for case in parametrize(
        "addition",
        [(1, 1, 2), (0, 0, 0), (-1, 1, 0)],
        |(a, b, exp)| {
            assert_eq!(a + b, *exp);
        },
    ) {
        assert!(case.status.is_passed(), "{} failed", case.name);
    }
}
```

### Tags

Use `.tag("name")` to categorize specs. Tags can be used for filtering with `cargo rvtest`:

```rust
describe("Vector")
    .it("dot product", || { ... }).tag("la").tag("core")
    .it("length", || { ... }).tag("la")
    .run();
```

## Test Organization

Each major module gets its own test file, mirroring the module structure:

| Module | Test File |
|--------|-----------|
| `algebra` | `algebra_tests.rs` |
| `calculus` | `calculus_tests.rs` |
| `geometry` | `geometry_tests.rs` |
| `la` (matrix) | `matrix_basic.rs` |
| `la` (tensor) | `tensor_basic.rs` |
| `la` (vector) | `vector_basic.rs`, `vector_units.rs` |
| `num` (basic) | `num_basic.rs` |
| `num` (Complex) | `num_complex_tests.rs` |
| `num` (Fraction) | `num_fraction_tests.rs` |
| `num` (Percentage) | `percentage_basic.rs` |
| `num` (Set) | `set_tests.rs` |
| `ops` | `ops_tests.rs` |
| `unit` | `unit_basic.rs`, `unit_ops.rs` |
| `utils` (evaluator) | `evaluator_tests.rs` |

## Conventions

- Use descriptive spec names that describe the expected behavior (e.g., `"computes volume correctly"` rather than `"test_sphere_volume"`).
- All documentation must be written in full English.
- Test edge cases explicitly: division by zero, domain errors, negative inputs, overflow.
- Use `assert!((a - b).abs() < EPSILON)` for floating-point comparisons instead of `assert_eq!`.
- Organize specs by related behavior — group them under a shared `describe` block.
- Import items through the crate root (`use rvmath::...`) rather than through internal paths.
- Tag specs with module names (`.tag("la")`, `.tag("num")`) to enable filtered runs.

## Running Tests

```bash
cargo test                         # Run all tests (standard)
cargo rvtest                       # Run all tests with rvtest runner & reporting
cargo rvtest -- --tag la           # Filter by tag
cargo rvtest --coverage            # Run with code coverage (LLVM)
cargo rvtest --format tap          # TAP output format
cargo rvtest --format junit        # JUnit XML output (CI)
cargo test -- --nocapture          # Show println! output (standard)
```
