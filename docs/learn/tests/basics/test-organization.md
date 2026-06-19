# Test Organization

How to structure tests — unit vs integration, where to put what, and naming conventions.

## Prerequisites

- [Why Test](why-test.md) — purpose of testing, `#[test]`
- Basic Rust module system

## Glossarium

| Term | Definition |
|------|------------|
| Unit Test | Tests a single function or module in isolation. Lives in `src/` or inline. |
| Integration Test | Tests multiple modules or the public API. Lives in `tests/` directory. |
| Doc Test | Code examples in doc comments (`///`) that are run as tests. |
| Inline Test | Tests written inside `#[cfg(test)] mod tests { ... }` blocks within source files. |

Unit tests live next to the code they test:

```rust
// src/math.rs
pub fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        assert_eq!(add(2, 2), 4);
    }
}
```

Integration tests live in `tests/` and can only use the public API:

```rust
// tests/math_integration.rs
use rvmath::algebra;

#[test]
fn simplify_expression() {
    let result = algebra::simplify("2*x + 3*x").unwrap();
    assert_eq!(result, "5*x");
}
```

| Concern | Unit Test | Integration Test |
|---------|-----------|-----------------|
| Location | `src/` (inline, `#[cfg(test)]`) | `tests/` directory |
| Access | Private and public items | Public API only |
| Compilation | With the library code | Separate binary |
| Speed | Fast | Slower |
| Purpose | Internal correctness | Behaviour through public API |

```bash
cargo test --lib           # unit tests only
cargo test --test '*'      # integration tests only
cargo test                 # both
```

## Next Steps

- [Writing Tests](writing-tests.md) — AAA pattern, naming, structuring test code
- [BDD Specs](../patterns/bdd-specs.md) — organising tests with `describe`/`it`
