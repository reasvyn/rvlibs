# Chapter 12 — Property-Based Testing

[← Previous](11-mocking-external-deps.md) • [Index](00-index.md) • [Next →](13-snapshot-testing.md)

---

Every test you have written so far has been **example-based**: you pick a
specific input, run the code, and assert on the output.  This works well, but
it only covers the examples you thought of.  Property-based testing flips the
model: you describe an **invariant** that must hold for all inputs, and the
computer generates thousands of random inputs to verify it.

---

## Example-Based vs Property-Based

```rust
// Example-based: tests specific inputs
#[test]
 fn reverse_of_reverse_is_original() {
     let v = vec![1, 2, 3];
     assert_eq!(v.clone().into_iter().rev().rev().collect::<Vec<_>>(), v);
 }
```

This test passes for `[1, 2, 3]`.  But does it hold for an empty vector?  A
single element?  A vector with duplicate values?  You could add more examples,
but you will never cover every possible input.

```rust
// Property-based: describe an invariant, let the computer generate inputs
use rvtest::property::{check, any};

#[test]
 fn reverse_involutive() {
     check("reverse of reverse is original", any::<Vec<i32>>(), |v: &Vec<i32>| {
         let rev: Vec<_> = v.iter().rev().copied().collect();
         let revrev: Vec<_> = rev.iter().rev().copied().collect();
         revrev == *v
     });
 }
```

This generates 100 random vectors (by default) and verifies the property on
each one.  If it finds a counter-example, it **shrinks** it to the smallest
failing input.

---

## Properties: What to Test

A property is a statement that must be true for every input.  Common property
categories:

| Category | Description | Example |
|----------|-------------|---------|
| **Invariant** | A condition that never changes | `list.len() == list.iter().rev().count()` |
| **Idempotence** | Applying an operation twice gives the same result | `sort(sort(list)) == sort(list)` |
| **Round-trip** | Serialise then deserialise gives the original | `decode(encode(value)) == value` |
| **Commutativity** | Order of operations does not matter | `add(a, b) == add(b, a)` |
| **Induction** | If it works for n, it works for n+1 | Building up test cases incrementally |
| **Oracle** | Compare against a known-good implementation | `fast_sort(list) == slow_sort(list)` |

---

## Using `rvtest::property::check`

```rust
use rvtest::property::{check, any};

#[test]
 fn addition_is_commutative() {
     check("commutativity", any::<i32>(), |a: &i32| {
         let b: i32 = 42;
         *a + b == b + *a
     });
 }
```

When a counter-example is found, `check` panics with:

```
property falsified after 100 test(s)
seed: 1234567890
counterexample: -123456
shrunk to: -1
```

The **shrunk to** value is the minimal input that still violates the property.
This is the key insight: instead of showing you a random failing input (which
might be huge and confusing), property-based testing finds the smallest input
that causes the failure.

---

## Built-in Strategies

`rvtest` provides strategies for all primitive types:

```rust
any::<i8>()    any::<i16>()    any::<i32>()    any::<i64>()
any::<u8>()    any::<u16>()    any::<u32>()    any::<u64>()
any::<usize>() any::<bool>()
```

### Vec Strategy

```rust
use rvtest::property::{vec, any};

let strategy = vec(any::<i32>(), 0, 10); // Vec of length 0 to 10
```

Useful for testing collection operations:

```rust
#[test]
 fn sort_is_stable() {
     check("sort is idempotent", vec(any::<i32>(), 0, 20), |v: &Vec<i32>| {
         let mut sorted = v.clone();
         sorted.sort();
         let mut double_sorted = sorted.clone();
         double_sorted.sort();
         sorted == double_sorted // Idempotence: sorting twice is the same
     });
 }
```

### Map Strategy

Transform a strategy's output:

```rust
use rvtest::property::{map, any};

let even_strategy = map(any::<i32>(), |x| x * 2); // Always even
```

### Filter Strategy

Only generate values that satisfy a predicate:

```rust
use rvtest::property::{filter, any};

let positive_strategy = filter(any::<i32>(), |x| *x > 0);
```

---

## Custom Strategies

For complex types, implement the `Strategy` trait:

```rust
use rvtest::property::{Strategy, check};
use rand::Rng;

#[derive(Debug)]
struct User {
    name: String,
    age: u8,
}

struct UserStrategy;

impl Strategy<User> for UserStrategy {
    fn generate(&self, rng: &mut dyn Rng) -> User {
        let names = ["Alice", "Bob", "Charlie", "Diana"];
        User {
            name: names[rng.random_range(0..names.len())].to_string(),
            age: rng.random_range(0..=120),
        }
    }

    fn shrink(&self, value: &User) -> Vec<User> {
        // Shrink toward minimum values
        let mut result = Vec::new();
        if value.age > 0 {
            result.push(User { age: value.age / 2, ..value.name.clone() });
        }
        result
    }
}

#[test]
 fn user_age_is_non_negative() {
     check("age is valid", UserStrategy, |user: &User| {
         user.age <= 150
     });
 }
```

---

## Configuring the Number of Tests

```rust
use rvtest::property::{check_with, any, PropertyConfig};

#[test]
 fn test_with_more_iterations() {
     check_with(
         "extensive check",
         any::<i32>(),
         |x: &i32| *x + 0 == *x,
         PropertyConfig {
             num_tests: 1000,  // Run 1000 iterations instead of default 100
             max_shrinks: 100, // Maximum shrink steps
             seed: Some(42),   // Deterministic seed for reproducibility
         },
     );
 }
```

---

## A Complete Example: Testing a Sort Function

```rust
fn my_sort(mut v: Vec<i32>) -> Vec<i32> {
    v.sort();
    v
}

#[test]
 fn test_sort_properties() {
     // Property 1: The output is sorted
     check("output is sorted", vec(any::<i32>(), 0, 20), |v: &Vec<i32>| {
         let sorted = my_sort(v.clone());
         sorted.windows(2).all(|w| w[0] <= w[1])
     });

     // Property 2: The output has the same length
     check("length is preserved", vec(any::<i32>(), 0, 20), |v: &Vec<i32>| {
         let sorted = my_sort(v.clone());
         sorted.len() == v.len()
     });

     // Property 3: Every element in the input appears in the output
     check("elements are preserved", vec(any::<i32>(), 0, 20), |v: &Vec<i32>| {
         let sorted = my_sort(v.clone());
         v.iter().all(|x| sorted.contains(x))
     });

     // Property 4: Sorting twice is the same as sorting once (idempotence)
     check("sort is idempotent", vec(any::<i32>(), 0, 20), |v: &Vec<i32>| {
         let once = my_sort(v.clone());
         let twice = my_sort(once.clone());
         once == twice
     });
 }
```

---

## When Property-Based Testing Shines

| Scenario | Why It Helps |
|----------|-------------|
| **Mathematical functions** | Easy to state invariants (commutativity, associativity) |
| **Parsing and formatting** | Round-trip: format(parse(s)) == s |
| **Data structures** | Invariants: size, ordering, containment |
| **Serialisation** | Round-trip: decode(encode(value)) == value |
| **Validation** | Valid inputs pass, invalid inputs fail |

## When Example-Based Tests Are Better

| Scenario | Why |
|----------|-----|
| **Specific bug regression** | You know the exact input that caused the bug |
| **Error messages** | Must match exact text |
| **Complex setup** | Property generation is too complex for the data |
| **Edge cases you know** | `MAX`, `MIN`, `0`, empty — known boundaries |

Use both.  Example-based tests for known cases, property-based tests for
everything else.

---

## Summary

- Property-based tests describe **invariants**, not specific examples
- The computer generates random inputs and verifies the invariant
- When a failure is found, it **shrinks** to the smallest failing input
- `rvtest::property` provides `check`, `any`, `vec`, `map`, `filter`
- Implement `Strategy` for custom types
- Use properties with example-based tests for thorough coverage

In the next chapter, we will explore snapshot testing — a pattern for testing
large, complex output values.

---

[← Previous](11-mocking-external-deps.md) • [Index](00-index.md) • [Next →](13-snapshot-testing.md)
