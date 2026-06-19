# Writing Effective Tests

The AAA pattern, naming conventions, and structuring test code for clarity and maintainability.

## Prerequisites

- [Test Organization](test-organization.md) — unit vs integration tests

## Glossarium

| Term | Definition |
|------|------------|
| AAA | Arrange-Act-Assert — the standard pattern for structuring test code. |
| Given-When-Then | A BDD variant of AAA: Given (setup), When (action), Then (assertion). |
| FIRST | Principles: Fast, Independent, Repeatable, Self-validating, Timely. |

## The AAA Pattern

Every test follows three phases:

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

## Naming Conventions

Good test names describe **what** is being tested and **what** the expected outcome is:

```rust
// ✅ Descriptive
#[test]
fn add_returns_sum_of_two_numbers() { ... }
#[test]
fn divide_by_zero_returns_nan() { ... }
#[test]
fn empty_string_returns_none() { ... }

// ❌ Vague
#[test]
fn test1() { ... }
#[test]
fn add() { ... }
```

## Structuring Multiple Tests

Related tests should be grouped:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    mod add_tests {
        #[test]
        fn positive_numbers() { assert_eq!(add(2, 3), 5); }
        #[test]
        fn negative_numbers() { assert_eq!(add(-2, -3), -5); }
        #[test]
        fn with_zero() { assert_eq!(add(0, 5), 5); }
    }

    mod subtract_tests {
        #[test]
        fn positive_result() { assert_eq!(subtract(5, 3), 2); }
        #[test]
        fn negative_result() { assert_eq!(subtract(3, 5), -2); }
    }
}
```

## The FIRST Principles

| Principle | Meaning |
|-----------|---------|
| **F**ast | Tests should run quickly to encourage frequent use. |
| **I**ndependent | Tests should not depend on each other or shared state. |
| **R**epeatable | Tests should produce the same result every time (no random/fixture dependence). |
| **S**elf-validating | Tests should pass or fail clearly — no manual inspection. |
| **T**imely | Tests should be written alongside (or before) the code. |

## Test What, Not How

Test observable behaviour, not implementation details:

```rust
// ✅ Test the behaviour
#[test]
fn sort_returns_ordered_list() {
    let mut items = vec![3, 1, 2];
    items.sort();
    assert_eq!(items, vec![1, 2, 3]);
}

// ❌ Don't test implementation details
#[test]
fn sort_uses_quick_sort_algorithm() {
    // This is an implementation detail that might change
}
```

## Next Steps

- [Assertions](assertions.md) — assertion macros and diff output
- [BDD Specs](bdd-specs.md) — organising tests with `describe`/`it`
