# Legacy and Refactoring

Adding tests to existing codebases and refactoring with confidence.

## Prerequisites

- [Writing Tests](writing-tests.md) — test structure and patterns
- [Property-Based Testing](property-based-testing.md) (recommended) — testing invariants

## Glossarium

| Term | Definition |
|------|------------|
| Legacy Code | Code without tests. Any change is risky because there's no safety net. |
| Characterisation Test | A test that captures current behaviour, regardless of correctness. |
| Golden Master | A snapshot of current output used as a reference for refactoring. |
| Refactoring | Changing the internal structure without changing external behaviour. |

## Adding Tests to Legacy Code

When adding tests to existing code, start with **characterisation tests** — tests that capture the current behaviour, not necessarily the correct behaviour:

```rust
#[test]
fn existing_parse_behaviour() {
    // This captures the CURRENT behaviour, not necessarily the correct one
    let result = legacy_parse("1 + 2 * 3");
    assert_eq!(result, 7.0); // current behaviour
}
```

Once the test passes, you know the code's current behaviour. Then you can refactor and the test tells you if the behaviour changed.

## Golden Master / Snapshot Approach

For complex output (HTML, reports, generated code), use snapshot testing as a golden master:

```rust
use rvtest::snapshot::assert_snapshot;

#[test]
fn legacy_report_output() {
    let report = generate_report();
    assert_snapshot("legacy_report", &report);
}
```

On first run, the current output is saved. After refactoring, run with `--update-all` if the output changes intentionally.

## Refactoring Workflow

1. **Cover** — Write characterisation tests around the code you want to change
2. **Refactor** — Make your changes
3. **Verify** — Run the tests to confirm behaviour is preserved
4. **Improve** — Once refactored, write proper behavioural tests

```rust
#[test]
fn refactoring_workflow() {
    describe("Legacy Module")
        .it("characterises current behaviour", || {
            let result = legacy_function(42);
            assert_eq!(result, "expected");
        })
        .it("new behaviour after refactoring", || {
            let result = refactored_function(42);
            assert_eq!(result, "expected");
        })
        .tag("refactoring")
        .run()
        .assert_all_pass();
}
```

## Tips for Legacy Code

- Start with one test per function (smoke test)
- Use snapshot tests for complex string output
- Test at the API boundary, not internals
- Add characterisation tests before touching any code
- Once refactored, replace characterisation tests with proper behavioural tests

## Next Steps

- [Architecture Tests](architecture-tests.md) — enforcing module structure
- [CI Integration](ci-integration.md) — running tests in CI
