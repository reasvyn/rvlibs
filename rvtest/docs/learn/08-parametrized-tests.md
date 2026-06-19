# Chapter 08 — Parametrized Tests

[← Previous](07-testing-structs-and-enums.md) • [Index](00-index.md) • [Next →](09-setup-and-teardown.md)

---

When the same test logic applies to many different inputs, writing a separate
`#[test]` function for each one is repetitive and hard to maintain.
Parametrized tests let you write the logic once and run it against a list of
inputs.

---

## The Problem: Repetitive Tests

```rust
#[test]
 fn add_1_and_1() { assert_eq!(add(1, 1), 2); }

 #[test]
 fn add_0_and_0() { assert_eq!(add(0, 0), 0); }

 #[test]
 fn add_neg1_and_1() { assert_eq!(add(-1, 1), 0); }

 #[test]
 fn add_max_and_min() { assert_eq!(add(i32::MAX, i32::MIN), -1); }
```

This is tedious.  Adding a new test case means copying the entire function.
Deleting one means finding and removing the right function.  And the pattern
is always the same: input tuple → expected output.

---

## Manual Approach: Loop Over Test Cases

A simple for-loop can reduce the repetition:

```rust
#[test]
 fn add_many_cases() {
     let cases = vec![
         (1, 1, 2),
         (0, 0, 0),
         (-1, 1, 0),
         (i32::MAX, i32::MIN, -1),
         (100, -50, 50),
     ];
     for (a, b, expected) in cases {
         let actual = add(a, b);
         assert_eq!(actual, expected, "add({a}, {b}) should be {expected}, got {actual}");
     }
 }
```

**Problem:** When this test fails, you only see "test_add_many_cases failed".
You do not know which input caused the failure without reading the error
message.

---

## Using `rvtest::parametrize`

`rvtest::parametrize` gives each case an independent test result with its own
name:

```rust
use rvtest::param::parametrize;

#[test]
 fn add_cases() {
     for case in parametrize(
         "add",
         [(1, 1, 2), (0, 0, 0), (-1, 1, 0), (i32::MAX, i32::MIN, -1)],
         |(a, b, expected)| {
             assert_eq!(add(*a, *b), *expected);
         },
     ) {
         assert!(case.status.is_passed(), "{} failed", case.name);
     }
 }
```

Each input becomes a named test case (`add[0]`, `add[1]`, etc.).  When one
fails, the output tells you exactly which case failed:

```
test add_cases ... FAILED
  --- test case 'add[2]' failed ---
  assertion failed: add(-1, 1) should be 0, got 1
```

---

## How `parametrize` Works

`parametrize` takes three arguments:

1. **A name** — used as a prefix for each test case
2. **An array of inputs** — each element is passed to the closure
3. **A closure** — receives one input and should contain assertions

It returns an iterator of `TestCase` structs with `name` and `status` fields:

```rust
pub struct TestCase {
    pub name: String,
    pub status: TestStatus,
    // ... other fields
}
```

---

## Named Parametrization

For better names than index numbers, use `parametrize_named`:

```rust
use rvtest::param::parametrize_named;

#[test]
 fn parse_cases() {
     for case in parametrize_named(
         "parse",
         [
             ("simple_number", "42"),
             ("negative", "-5"),
             ("zero", "0"),
             ("whitespace", "  42  "),
         ],
         |input| {
             let result: i32 = input.trim().parse().unwrap();
             assert!(result > 0, "expected positive, got {result}");
         },
     ) {
         assert!(case.status.is_passed(), "{} failed: {:?}", case.name, case.status);
     }
 }
```

Now the test cases are named `parse :: simple_number`, `parse :: negative`,
etc.  Much easier to identify in test output.

---

## Testing Different Behaviours

Parametrized tests are especially useful when the same function should produce
different outcomes for different inputs:

```rust
fn classify_number(n: i32) -> &'static str {
    match n {
        0 => "zero",
        n if n > 0 => "positive",
        _ => "negative",
    }
}

#[test]
 fn classify_numbers() {
     let cases = [
         (0, "zero"),
         (1, "positive"),
         (-1, "negative"),
         (i32::MAX, "positive"),
         (i32::MIN, "negative"),
     ];
     for (input, expected) in cases {
         assert_eq!(classify_number(input), expected, "classify({input})");
     }
 }
```

When this test fails, the custom message `"classify(-1)"` immediately tells
you which input caused the problem.

---

## Combining with Property Checks

Parametrized tests can call property-based checks inside each case:

```rust
use rvtest::param::parametrize;
use rvtest::property::{check, any};

#[test]
 fn combined_test() {
     for case in parametrize(
         "validation",
         [
             ("alice", true),
             ("", false),
             ("ab", false),
             ("a", false),
             ("alice123", true),
         ],
         |(name, expected)| {
             // Test specific cases
             assert_eq!(validate_username(name), *expected);
             // Also run random property check
             check("random names", any::<String>(), |s| {
                 let result = validate_username(s);
                 if s.len() < 3 {
                     result == false
                 } else {
                     true // always passes for long enough strings
                 }
             });
         },
     ) {
         assert!(case.status.is_passed(), "{} failed", case.name);
     }
 }
```

---

## Testing Multiple Return Values

When a function returns a struct or tuple, destructure in the expectation:

```rust
fn split_name(full_name: &str) -> (&str, &str) {
    full_name.split_once(' ').unwrap_or((full_name, ""))
}

#[test]
 fn split_full_names() {
     let cases = [
         ("Alice Smith", ("Alice", "Smith")),
         ("Bob", ("Bob", "")),
         ("John Michael Doe", ("John", "Michael Doe")),
     ];
     for (input, (expected_first, expected_last)) in cases {
         let (first, last) = split_name(input);
         assert_eq!(first, expected_first, "first name for '{input}'");
         assert_eq!(last, expected_last, "last name for '{input}'");
     }
 }
```

---

## Comparing With and Without Parametrization

| Aspect | Loop + assert | `parametrize` | `parametrize_named` |
|--------|---------------|---------------|---------------------|
| Code per new case | 1 line | 1 line | 1 line |
| Failure output | Only line number | Named case | Named case |
| Runs all cases | Stops at first failure | Stops at first failure | Stops at first failure |
| Case names | Index in array | `name[index]` | `name :: label` |

---

## Summary

- `rvtest::parametrize` reduces repetitive test code
- Each input becomes an independently identifiable test case
- `parametrize_named` provides descriptive names instead of indices
- Use parametrization whenever the same logic applies to multiple inputs
- Combine parametrized tests with other patterns (property checks, assertions)
  for thorough coverage with minimal code

In the next chapter, we will look at how to manage setup and teardown logic
with lifecycle hooks.

---

[← Previous](07-testing-structs-and-enums.md) • [Index](00-index.md) • [Next →](09-setup-and-teardown.md)
