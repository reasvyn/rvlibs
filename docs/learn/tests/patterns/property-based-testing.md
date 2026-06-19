# Property-Based Testing

Testing invariants over many randomly generated inputs — find edge cases you didn't think of.

## Prerequisites

- [Why Test](../basics/why-test.md) — test basics
- [Assertions](../basics/assertions.md) — assertion macros


## Basic Check

```rust
use rvtest::property::{check, any};

#[test]
fn addition_is_commutative() {
    check("commutativity", any::<i32>(), |a: &i32| {
        let b: i32 = 42;
        a + b == b + *a
    });
}
```

When a counterexample is found, `check` panics with the seed and the (shrunk) minimal failing input:

```
property 'commutativity' FAILED
seed: 12345678
counterexample: -42
```

## Custom Strategies

```rust
use rvtest::property::{Strategy, any, check};
use rand::RngCore;

struct EvenStrategy;

impl Strategy<i32> for EvenStrategy {
    fn generate(&self, rng: &mut dyn RngCore) -> i32 {
        rng.next_u32() as i32 & !1  // force even
    }
}

#[test]
fn even_numbers() {
    check("even numbers are divisible by 2", EvenStrategy, |n: &i32| {
        n % 2 == 0
    });
}
```

## Combining with BDD Specs

```rust
describe("Numeric")
    .it("addition is commutative", || {
        check("commutativity", any::<i32>(), |a: &i32| {
            let b: i32 = 42;
            a + b == b + *a
        });
    })
    .run()
    .assert_all_pass();
```

## When to Use Property-Based Tests

| Good For | Not Good For |
|----------|-------------|
| Mathematical invariants | Complex setup with many dependencies |
| Round-trip (serialize → deserialize) | One-off specific behaviour |
| Boundary-independent logic | Sequential/stateful tests |
| Data structure correctness | Tests that need precise input control |

## Glossarium

| Term | Definition |
|------|------------|
| Property | An invariant that should hold for all valid inputs (e.g., `a + 0 == a`). |
| Strategy | Defines how to generate random values of a given type. |
| Shrinking | The process of finding the minimal failing input from a larger one. |
| Counterexample | A specific input that causes the property to fail. |


## Next Steps

- [Mocking](mocking.md) — spies, stubs, and scoped function replacement
- [Snapshots](snapshots.md) — file-based assertions for stable output
