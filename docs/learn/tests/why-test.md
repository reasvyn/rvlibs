# Why Test

Understanding testing — what it is, the types of tests, and the cost-benefit trade-offs.

## Prerequisites

- Basic Rust syntax — functions, `let`, `assert!`


## Why Test?

Tests serve three purposes:

1. **Correctness** — Does the code do what it's supposed to do?
2. **Regression prevention** — Does a change break existing behaviour?
3. **Documentation** — How is this function/ module intended to be used?

Without tests, every change is a gamble. With tests, you can refactor with confidence.

## Writing Your First Test

```rust
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}
```

Run with `cargo test`:

```
running 1 test
test it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## The Three A's

Every test follows the same structure:

```rust
#[test]
fn add_returns_sum() {
    // Arrange — set up inputs and expected output
    let a = 2;
    let b = 3;
    let expected = 5;

    // Act — call the function being tested
    let result = add(a, b);

    // Assert — verify the result matches expectations
    assert_eq!(result, expected);
}
```

## Test Organisation

| Concern | Unit Test | Integration Test |
|---------|-----------|-----------------|
| Location | `src/` (inline) | `tests/` directory |
| Scope | Single function or module | Multiple modules or crate API |
| Speed | Very fast | Slower (compiles as separate binary) |
| What it tests | Internal correctness | Behaviour through the public API |

```bash
cargo test --lib        # unit tests only
cargo test --test '*'   # integration tests only
cargo test              # both
```

## Glossarium

| Term | Definition |
|------|------------|
| Unit Test | Tests that verify a single function or module in isolation. Fast, focused. |
| Integration Test | Tests that verify how multiple modules or crates work together. |
| Regression Test | A test that verifies a previously-fixed bug stays fixed. |
| Smoke Test | A minimal test that checks a feature is basically working. |
| Coverage | A metric measuring what percentage of code is exercised by tests. |
| `#[test]` | Rust's built-in attribute that marks a function as a test. |


## Next Steps

- [Assertions](assertions.md) — `assert_eq!`, `assert_ok!`, `assert_err!`, `assert_matches!`
- [BDD Specs](bdd-specs.md) — `describe`/`it` blocks with rvtest
