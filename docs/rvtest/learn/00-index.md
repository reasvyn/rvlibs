# Learn Testing in Rust

> A step-by-step guide to writing effective tests in Rust, using `rvtest` as
> the testing toolkit.  No prior testing experience required — just basic Rust
> knowledge (ownership, traits, enums, pattern matching).

---

## How to Use This Guide

Each chapter builds on the previous one.  Start from chapter 1 if you are new
to testing, or jump to any chapter using the index below.

Every chapter includes:
- A clear problem statement
- Step-by-step code examples
- Why each pattern matters
- Navigation links to previous and next chapters

---

## Part 1 — Foundations of Rust Testing

| # | Chapter | What You Will Learn |
|---|---------|---------------------|
| 01 | [Why Test?](01-why-test.md) | What testing is, types of tests, the cost-benefit of testing |
| 02 | [Rust Basics for Testing](02-rust-basics-for-test.md) | `#[test]`, `assert!`, `cargo test`, test modules |
| 03 | [Test Organization](03-test-organization.md) | Unit vs integration tests, where to put test code |
| 04 | [Writing Effective Tests](04-writing-effective-tests.md) | The AAA pattern, naming conventions, structuring test code |

## Part 2 — Basic Test Patterns

| # | Chapter | What You Will Learn |
|---|---------|---------------------|
| 05 | [Assertions and Matchers](05-assertions-and-matchers.md) | `assert_eq!`, custom failure messages, `rvtest::assert_eq!` with diffs |
| 06 | [Testing Errors](06-testing-errors.md) | `#[should_panic]`, `Result<T, E>` in tests, testing error variants |
| 07 | [Testing Structs and Enums](07-testing-structs-and-enums.md) | `PartialEq`, `Debug`, custom comparison logic |
| 08 | [Parametrized Tests](08-parametrized-tests.md) | Running the same logic with multiple inputs, `rvtest::parametrize` |

## Part 3 — Beyond Basic Tests

| # | Chapter | What You Will Learn |
|---|---------|---------------------|
| 09 | [Setup and Teardown](09-setup-and-teardown.md) | Before/after hooks, temp directories, environment variables |
| 10 | [Test Doubles](10-test-doubles.md) | Stubs, spies, mocks — what each is for and when to use them |
| 11 | [Mocking External Dependencies](11-mocking-external-deps.md) | `rvtest::mock::Spy`, `Stub`, `patch!` for functions |
| 12 | [Property-Based Testing](12-property-based-testing.md) | Random inputs, shrinking, invariants with `rvtest::property` |
| 13 | [Snapshot Testing](13-snapshot-testing.md) | Golden files, review workflow, `rvtest::snapshot` |

## Part 4 — Advanced Testing Concerns

| # | Chapter | What You Will Learn |
|---|---------|---------------------|
| 14 | [Code Coverage](14-coverage.md) | Measuring coverage, interpreting results, improving coverage |
| 15 | [Flaky Tests](15-flaky-tests.md) | Identifying flaky tests, quarantining, fixing common causes |
| 16 | [Benchmark and Regression](16-benchmark-and-regression.md) | Measuring performance, detecting regressions |
| 17 | [Legacy and Refactoring](17-legacy-and-refactoring.md) | Adding tests to existing code, characterization tests |
| 18 | [CI Integration](18-ci-integration.md) | Running tests in CI, JUnit output, GitHub Actions |

## Part 5 — Real-World Workflows

| # | Chapter | What You Will Learn |
|---|---------|---------------------|
| 19 | [Architecture Tests](19-architecture-tests.md) | Enforcing module boundaries with `rvtest::arch` |
| 20 | [Testing Concurrent Code](20-testing-concurrent-code.md) | Race conditions, determinism, testing async code |
| 21 | [Faster Feedback Loops](21-faster-feedback-loops.md) | Watch mode, compile daemon, test selection, `--retest` |

---

## Prerequisites

- Rust toolchain installed (1.96+)
- Basic familiarity with Rust syntax
- A Rust project to experiment with

Some chapters use `rvtest`.  Add the library to your project:

```toml
[dev-dependencies]
rvtest = "0.3"
```

For the `cargo rvtest` CLI, install the binary separately:

```bash
cargo install cargo-rvtest
```

---

## Conventions Used

```rust
// Code blocks like this show Rust code
// Comments highlight important details
```

```bash
# Terminal commands are shown like this
cargo test
```

> Notes and tips appear in block quotes like this.

---

[Next →](01-why-test.md) — Start with Chapter 1: Why Test?
