# Testing Policy — Dogfooding & Modularity

> **rvtest tests itself with rvtest.**  Every test MUST use the
> `describe` / `it` BDD API whenever possible.  Raw `#[test]`
> functions are reserved for bootstrapping and doc-tests only.

---

## Why Dogfooding

1. **Quality signal** — If the API is awkward to use in our own tests,
   it will be awkward for users.  Dogfooding forces us to eat our own
   dog food.

2. **Regression detection** — Breaking changes in the builder API or
   runner are caught immediately because our tests depend on the exact
   same chain: `describe().it().run().assert_all_pass()`.

3. **Living documentation** — Our test suite serves as a canonical set
   of usage examples.  New contributors can look at the tests to
   understand how features work end-to-end.

4. **Coverage of edge cases** — Retries, timeouts, hooks, nesting,
   tags, parametrized tests, property checks — all must be exercised
   in the dogfooded suite.

---

## Why Modularity

1. **Readability** — A focused test function with a clear name is easier
   to understand than a monolithic test that covers many behaviours.

2. **Debuggability** — When a modular test fails, the failure point
   maps directly to one sub-behaviour, not a dozen.

3. **Maintainability** — Adding or removing a sub-behaviour means adding
   or removing one `.it()` block, not restructuring a whole function.

---

## Rules

### Rule 1: Dogfood everything possible

All tests — including inline `#[cfg(test)]` unit tests in source
modules — MUST use `describe()` / `.it()` / `.run()` / `.assert_all_pass()`
whenever the feature being tested is part of rvtest's own public API.

Raw `#[test]` functions are allowed ONLY for:

- Bootstrapping (testing `describe`/`it` itself)
- Doc-tests (`rustdoc` examples)
- Internal helper functions that are `#[doc(hidden)]`

```rust
// ✅ CORRECT — dogfooded even for unit tests
#[cfg(test)]
mod tests {
    #[test]
    fn test_feature() {
        describe("Feature")
            .it("works correctly", || {
                rvtest::assert_eq!(computed_value(), expected);
            })
            .run()
            .assert_all_pass();
    }
}
```

```rust
// ❌ WRONG — bare #[test] for something that could use describe/it
#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {
        assert_eq!(foo(), bar);
    }
}
```

### Rule 2: Every API must be dogfooded

When adding a new public API, a corresponding dogfooded test MUST be
added in the same PR.  The test MUST exercise the API through
`describe()` / `it()` rather than calling the API directly.

### Rule 3: One behaviour per `.it()` block

Each `.it("description", || ...)` block MUST test exactly one
behaviour or scenario.  If a test function needs to verify multiple
behaviours, add multiple `.it()` blocks.

```
// ✅ CORRECT — focused blocks
describe("Calculator")
    .it("adds positive numbers", || { ... })
    .it("adds negative numbers", || { ... })
    .it("handles overflow", || { ... })
    .run()
    .assert_all_pass();

// ❌ WRONG — one block doing everything
.it("handles all cases", || {
    assert_eq!(add(1, 1), 2);
    assert_eq!(add(-1, -1), -2);
    assert_eq!(add(i32::MAX, 1), i32::MIN);  // three concerns, one block
})
```

### Rule 4: `#[should_panic]` is allowed only at the outer level

Tests that verify failure behaviour (e.g. `assert_all_pass` panics)
may use `#[should_panic]` on the outer `#[test]` function, or use
`catch_unwind` inside a dogfooded `.it()` block.

```rust
// ✅ CORRECT — catch_unwind inside describe/it
#[test]
fn rvtest_reporters() {
    describe("Reporters")
        .it("pretty reporter shows summary", || {
            let result = std::panic::catch_unwind(|| {
                describe("Failing")
                    .it("fails", || panic!("intentional"))
                    .run()
                    .assert_all_pass();
            });
            assert!(result.is_err());
        })
        .run()
        .assert_all_pass();
}
```

### Rule 5: Test function names use the `rvtest_` prefix

All test functions that exercise rvtest itself MUST be named with the
`rvtest_` prefix for easy discovery:

```
rvtest_spec
rvtest_parametrized
rvtest_property
rvtest_runner
rvtest_reporters
rvtest_architecture
rvtest_snapshot_create_and_match
rvtest_snapshot_mismatch_detected
rvtest_child_hooks
rvtest_before_each_after_each
rvtest_source_location
rvtest_assert_macros
```

### Rule 6: Keep tests independent

Each `#[test]` function is a separate `describe` block that covers one
feature area.  Tests must not share mutable state — use local
`AtomicU32`, `Arc<Mutex<>>`, or fresh `TestRun` instances inside each
`.it()` closure.

### Rule 7: Structure tests by module hierarchy

Organise `.describe()` nesting to mirror the module or feature
hierarchy.  This makes it easy to locate tests for a given component:

```rust
// ✅ CORRECT — hierarchy mirrors code structure
describe("Spec")
    .describe("execution")
        .it("passes when all tests pass", || { ... })
        .it("reports failures", || { ... })
    .describe("hooks")
        .it("runs before_all", || { ... })
        .it("runs after_all", || { ... })
    .run()
    .assert_all_pass();
```

### Rule 8: Prefer real TestRun data over empty/mocked

When testing reporters or runners, construct `TestRun` instances with
realistic data (mixed pass/fail/skip, realistic durations).  Empty
`TestRun` objects should only be used to verify edge-case formatting.

---

## Enforcement

- **CI:** `cargo test` MUST pass with zero failures and zero warnings.
- **Code review:** Every PR is checked for dogfooding compliance and
  modularity.
- **Exceptions:** Only the `rvtest-macros` proc-macro crate may have
  integration tests that import macros directly (`use rvtest_macros::*`)
  instead of going through the `rvtest` re-export.  These tests still
  use `#[describe]` / `#[it]` macros, which are the proc-macro form
  of the same API.

---

## Modularity Checklist for PRs

Before submitting, verify:

- [ ] Every public API change has a corresponding dogfooded test
- [ ] Each `.it("...", ||` block tests exactly one behaviour
- [ ] Tests use realistic data, not empty stubs (unless testing empty
      edge cases)
- [ ] Test names use the `rvtest_` prefix
- [ ] No two tests share mutable state
- [ ] CI passes with zero failures and zero warnings

---

## Current Coverage

Every public feature listed below has at least one dogfooded test:

| Feature | Test(s) |
|---|---|
| Basic specs | `rvtest_spec` |
| Nesting | `rvtest_spec` |
| Tags | `rvtest_spec`, `rvtest_parametrized`, `rvtest_property`, `rvtest_runner`, `rvtest_reporters` |
| Timeouts | `rvtest_spec` |
| Retries | `rvtest_spec` |
| Before/after hooks | `rvtest_spec` |
| Child hooks (nested before/after) | `rvtest_child_hooks` |
| before_each / after_each | `rvtest_before_each_after_each` |
| Source location tracking | `rvtest_source_location` |
| Parametrized tests | `rvtest_parametrized` |
| Property-based tests | `rvtest_property` |
| Runner config | `rvtest_runner` |
| Reporters (all 6) | `rvtest_reporters` |
| Architecture tests | `rvtest_architecture` |
| Snapshots | `rvtest_snapshot_create_and_match`, `rvtest_snapshot_mismatch_detected` |
| Assertion macros (assert_eq/ok/err/matches/delta) | `rvtest_assert_macros` |
| Proc-macros | `rvtest-macros/tests/integration.rs` (4 tests) |
