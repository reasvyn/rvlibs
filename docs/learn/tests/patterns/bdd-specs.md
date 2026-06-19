# BDD Specs

Organising tests with `describe`/`it` blocks — inspired by Behaviour-Driven Development.

## Prerequisites

- [Why Test](../basics/why-test.md) — `#[test]`, the Three A's


## Basic Spec

```rust
use rvtest::spec::describe;

#[test]
fn calculator_tests() {
    describe("Calculator")
        .it("adds two numbers", || {
            assert_eq!(2 + 2, 4);
        })
        .it("subtracts", || {
            assert_eq!(5 - 3, 2);
        })
        .run()
        .assert_all_pass();
}
```

## Nested Specs

```rust
#[test]
fn math_spec() {
    describe("Math")
        .describe("addition")
            .it("positive + positive", || assert_eq!(2 + 2, 4))
            .it("negative + negative", || assert_eq!(-2 + -3, -5))
        .describe("multiplication")
            .it("zero property", || assert_eq!(5 * 0, 0))
            .tag("core")
            .timeout(std::time::Duration::from_secs(1))
        .run()
        .assert_all_pass();
}
```

## Tags

Tags enable selective test execution:

```rust
describe("Database")
    .it("connects", || { ... }).tag("smoke")
    .it("queries", || { ... }).tag("smoke").tag("slow")
    .run();
```

```bash
cargo rvtest --tag smoke           # only smoke tests
cargo rvtest --tag slow            # only slow tests
cargo rvtest --exclude-tag slow    # skip slow tests
```

## Hooks

Lifecycle hooks run setup and teardown code:

```rust
describe("Database")
    .before_all(|| { /* runs once before any child test */ })
    .after_all(||  { /* runs once after all child tests */ })
    .before_each(|| { /* runs before each child test */ })
    .after_each(||  { /* runs after each child test */ })
    .it("inserts", || { /* ... */ })
    .it("queries", || { /* ... */ })
    .run()
    .assert_all_pass();
```

## Retries

For flaky tests that occasionally fail:

```rust
describe("Network")
    .it("fetches data", || {
        // might fail transiently
    })
    .retries(3)
    .run()
    .assert_all_pass();
```

## Glossarium

| Term | Definition |
|------|------------|
| `describe` | A named group of related tests. Can be nested. |
| `it` | A single test case within a `describe` block. |
| Tag | Metadata attached to a test for filtering (`--tag`, `--exclude-tag`). |
| Hook | Code that runs before/after tests (`before_all`, `after_each`, etc.). |
| Retry | Number of times to re-run a flaky test before marking it as failed. |


## Next Steps

- [Parametrized Tests](parametrized-tests.md) — running the same test with multiple inputs
- [Property-Based Testing](property-based-testing.md) — testing invariants over many random inputs
