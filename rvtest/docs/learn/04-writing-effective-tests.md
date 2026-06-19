# Chapter 04 — Writing Effective Tests

[← Previous](03-test-organization.md) • [Index](00-index.md) • [Next →](05-assertions-and-matchers.md)

---

Not all tests are created equal.  A well-written test catches bugs, documents
behaviour, and makes refactoring safe.  A poorly-written test is fragile,
confusing, and often ignored.  This chapter teaches the patterns that produce
effective tests.

---

## The AAA Pattern

Every test should follow three phases:

```
┌─────────────────────────────────────┐
│  Arrange — set up the test data     │
│  Act     — call the code under test │
│  Assert  — verify the result        │
└─────────────────────────────────────┘
```

```rust
#[test]
fn add_two_numbers() {
    // Arrange
    let a = 2;
    let b = 3;

    // Act
    let result = add(a, b);

    // Assert
    assert_eq!(result, 5);
}
```

Separating these phases with blank lines makes the test easier to read.  When
a test fails, you can immediately see which phase went wrong.

---

## One Assertion per Behaviour

A test should verify **one behaviour**.  If a function has multiple behaviours,
write multiple tests:

```rust
// ❌ Bad — multiple concerns in one test
#[test]
fn test_all_operations() {
    assert_eq!(add(2, 3), 5);
    assert_eq!(sub(5, 3), 2);
    assert_eq!(mul(2, 3), 6);
    assert_eq!(div(6, 3), 2);
}
```

```rust
// ✅ Good — one behaviour per test
#[test]
fn add_two_positive_numbers() {
    assert_eq!(add(2, 3), 5);
}

#[test]
fn subtract_positive_numbers() {
    assert_eq!(sub(5, 3), 2);
}

#[test]
fn multiply_two_numbers() {
    assert_eq!(mul(2, 3), 6);
}

#[test]
fn divide_two_numbers() {
    assert_eq!(div(6, 3), 2);
}
```

**Why?** When a test fails, you want to know exactly which behaviour is
broken.  A single assertion per test gives you that information immediately.

---

## Naming Conventions

A test name should describe:

1. The function or scenario being tested
2. The input or condition
3. The expected outcome

```rust
// Pattern: <function>_<condition>_<expected_result>
// Pattern: <scenario>_<input>_<behaviour>
```

```rust
#[test]
fn add_positive_numbers_returns_sum() {}

#[test]
fn add_with_zero_returns_original_value() {}

#[test]
fn parse_invalid_input_returns_zero() {}

#[test]
fn authenticate_wrong_password_returns_error() {}
```

Good test names serve as documentation.  When a test fails, the name tells you
what the code should do.

---

## Testing Edge Cases

Bugs hide at boundaries.  Always test:

| Category | Example |
|----------|---------|
| **Empty input** | Empty string, empty vector, zero-length array |
| **Single element** | Vector with one item, single-character string |
| **Boundary values** | `i32::MAX`, `i32::MIN`, `0`, `-1` |
| **Invalid input** | Null-like values, malformed data |
| **Error paths** | Network timeout, file not found, permission denied |

```rust
#[test]
fn max_value_does_not_overflow() {
    // i32::MAX + 1 would overflow — make sure your code handles it
    let result = safe_add(i32::MAX, 1);
    assert_eq!(result, Err("overflow"));
}

#[test]
 fn empty_string_returns_none() {
     assert_eq!(parse(""), None);
 }
```

---

## Testing Error Messages

When testing error cases, verify not just that an error occurred, but that the
error message is correct:

```rust
#[test]
 fn invalid_email_returns_descriptive_error() {
     let result = validate_email("not-an-email");
     assert!(result.is_err());
     assert_eq!(
         result.unwrap_err().to_string(),
         "invalid email: missing @ symbol"
     );
 }
```

Error messages are part of your API.  Test them like any other output.

---

## Tests Should Be Deterministic

A test that passes sometimes and fails other times is worse than no test at
all.  Non-deterministic sources include:

```rust
// ❌ Depends on current time
#[test]
fn test_greeting() {
    let greeting = greet("Alice");
    assert_eq!(greeting, "Good morning, Alice!"); // Fails in the afternoon
}
```

```rust
// ✅ Use fixed input
#[test]
fn test_greeting_morning() {
    let greeting = greet_at_time("Alice", "08:00");
    assert_eq!(greeting, "Good morning, Alice!");
}

#[test]
fn test_greeting_evening() {
    let greeting = greet_at_time("Alice", "20:00");
    assert_eq!(greeting, "Good evening, Alice!");
}
```

Other sources of non-determinism:
- Random numbers (use a fixed seed)
- Network requests (mock the network)
- File system state (use temp directories)
- Thread scheduling (use deterministic synchronisation)

---

## Tests Should Be Isolated

Tests must not share mutable state:

```rust
// ❌ Bad — shared mutable state
static mut COUNTER: i32 = 0;

#[test]
 fn increment() {
     unsafe { COUNTER += 1; }
     assert_eq!(unsafe { COUNTER }, 1); // Only passes when run first
 }
```

```rust
// ✅ Good — local state
#[test]
 fn increment() {
     let mut counter = 0;
     counter += 1;
     assert_eq!(counter, 1);
 }

 #[test]
 fn increment_twice() {
     let mut counter = 0;
     counter += 1;
     counter += 1;
     assert_eq!(counter, 2);
 }
```

If tests share state, running them in a different order or in parallel will
produce different results.

---

## Preparing for the Next Chapter

From this point, we will start using `rvtest` for better assertions.  Add it
to your project:

```toml
[dev-dependencies]
rvtest = "0.3"
```

### Optional: Proc-Macro API (`#[describe]` / `#[it]`)

If you prefer attribute macros over builder syntax, enable the `macros`
feature:

```toml
[dev-dependencies]
rvtest = { version = "0.3", features = ["macros"] }
```

This lets you write specs with `#[describe]` and `#[it]` attributes:

```rust
use rvtest::*;

#[test]
#[describe("Calculator")]
fn calculator_tests() {
    #[it("adds two positive numbers")]
    fn adds() {
        assert_eq!(2 + 2, 4);
    }

    #[it("subtracts")]
    #[tag("arithmetic")]
    fn subtracts() {
        assert_eq!(5 - 3, 2);
    }
}
```

The macro API is purely syntactic sugar over the builder API — the same
features (hooks, tags, retries, timeouts) are available in both styles.

Then in the next chapter, we will see how `rvtest::assert_eq!` gives better
failure messages than the standard `assert_eq!`.

---

## Summary

- Use the **AAA** pattern: Arrange, Act, Assert
- Test **one behaviour per test**
- Name tests descriptively: `function_condition_expected`
- Always test **edge cases** (empty, boundary, invalid)
- Tests must be **deterministic** and **isolated**
- Test error messages, not just error presence

---

[← Previous](03-test-organization.md) • [Index](00-index.md) • [Next →](05-assertions-and-matchers.md)
