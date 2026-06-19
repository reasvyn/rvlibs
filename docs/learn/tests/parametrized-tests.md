# Parametrized Tests

Running the same test logic against multiple input sets without boilerplate.

## Prerequisites

- [Why Test](why-test.md) — `#[test]`, basic test structure


## Basic Parametrization

```rust
use rvtest::param::parametrize;

#[test]
fn addition_cases() {
    let cases = parametrize(
        "add",
        [(1, 1, 2), (0, 0, 0), (-1, 1, 0), (-2, -2, -4)],
        |(a, b, expected)| {
            assert_eq!(a + b, *expected);
        },
    );
    assert!(cases.iter().all(|c| c.status.is_passed()));
}
```

## Named Parametrization

```rust
use rvtest::param::parametrize_named;

#[test]
fn parse_cases() {
    let results = parametrize_named(
        "parse",
        [("empty", ""), ("valid_number", "42")],
        |input| {
            if !input.is_empty() {
                assert!(input.parse::<i32>().is_ok());
            }
        },
    );
    assert!(results.iter().all(|c| c.status.is_passed()));
}
```

## Parametrization in BDD Specs

`parametrize` can be composed inside `describe`/`it` blocks:

```rust
describe("Calculator")
    .it("handles parametrized cases", || {
        for case in parametrize(
            "add",
            [(1, 1, 2), (0, 0, 0), (-1, 1, 0)],
            |(a, b, exp)| assert_eq!(a + b, *exp),
        ) {
            assert!(case.status.is_passed(), "{} failed", case.name);
        }
    })
    .run()
    .assert_all_pass();
```

## Glossarium

| Term | Definition |
|------|------------|
| `parametrize` | Run a closure against a list of input tuples. Returns collected results. |
| `parametrize_named` | Like `parametrize` but each case has a descriptive name. |


## Next Steps

- [Property-Based Testing](property-based-testing.md) — testing invariants over many random inputs
- [Mocking](mocking.md) — spies, stubs, and scoped function replacement
