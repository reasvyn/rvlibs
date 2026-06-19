# Testing

## Framework

rvlibs uses [`rvtest`](https://github.com/reasvyn/rvlibs/tree/main/crates/rvtest) — the monorepo's own BDD-style testing library. All crates are tested with it.

## Organisation

Each crate has its own `tests/` directory with integration tests:

| Crate | Test Files |
|-------|-----------|
| `rvmath` | `algebra_tests.rs`, `calculus_tests.rs`, `evaluator_tests.rs`, `geometry_tests.rs`, `matrix_basic.rs`, `num_basic.rs`, `num_complex_tests.rs`, `num_fraction_tests.rs`, `ops_tests.rs`, `percentage_basic.rs`, `set_tests.rs`, `tensor_basic.rs`, `unit_basic.rs`, `unit_ops.rs`, `vector_basic.rs`, `vector_units.rs` |
| `rvtest` | `arch.rs`, `assert.rs`, `capture.rs`, `cli.rs`, `coverage.rs`, `mock.rs`, `param.rs`, `property.rs`, `report.rs`, `runner.rs`, `snapshot.rs`, `spec.rs` |
| `rvtest-macros` | `integration.rs` |

Writing tests with rvtest uses the BDD API:
```rust
use rvtest::spec::describe;

#[test]
fn calculator_tests() {
    describe("Calculator")
        .it("adds two numbers", || assert_eq!(2 + 2, 4))
        .it("subtracts", || assert_eq!(5 - 3, 2))
        .run()
        .assert_all_pass();
}
```

## Dogfooding Policy (rvtest)

> **rvtest tests itself with rvtest.**

Every test MUST use `describe`/`it`/`run`/`assert_all_pass()` whenever the feature being tested is part of rvtest's own public API. Raw `#[test]` functions are reserved for bootstrapping and doc-tests only.

### Rules
1. **Dogfood everything possible** — All rvtest integration tests use the BDD API.
2. **Every API must be dogfooded** — New public API requires a corresponding dogfooded test in the same PR.
3. **One behaviour per `.it()` block** — Each block tests exactly one scenario.
4. **Test names use `rvtest_` prefix** — e.g., `rvtest_spec`, `rvtest_property`.
5. **Keep tests independent** — No shared mutable state between tests.
6. **Prefer real data** — Construct `TestRun` instances with realistic data.

### Why Dogfooding
1. **Quality signal** — If the API is awkward in our own tests, it will be awkward for users.
2. **Regression detection** — Breaking changes are caught immediately.
3. **Living documentation** — The test suite serves as canonical usage examples.
4. **Edge case coverage** — Retries, timeouts, hooks, nesting, tags all exercised in the dogfooded suite.

### Current Coverage
Every public rvtest feature has at least one dogfooded test: specs, nesting, tags, timeouts, retries, hooks, parametrized, property-based, runner config, reporters, architecture, snapshots, assertion macros, proc-macros.

## Running Tests

```bash
# Full workspace
cargo test --workspace

# Specific crate
cargo test -p rvmath
cargo test -p rvtest

# With rvtest runner
cargo rvtest

# With coverage
cargo rvtest --coverage

# With format output
cargo rvtest -F tap
cargo rvtest -F junit
```

## Test Conventions

- Use descriptive spec names describing expected behaviour (e.g., `"computes volume correctly"`).
- Test edge cases explicitly: division by zero, domain errors, negative inputs, overflow.
- Use `assert!((a - b).abs() < EPSILON)` for floating-point comparisons.
- Prefer integration tests in `tests/` over inline `#[cfg(test)]`.
- Tag specs with module names (`.tag("la")`, `.tag("num")`) for filtered runs.
