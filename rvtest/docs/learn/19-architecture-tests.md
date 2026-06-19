# Chapter 19 — Architecture Tests

[← Previous](18-ci-integration.md) • [Index](00-index.md) • [Next →](20-testing-concurrent-code.md)

---

As a project grows, its architecture tends to degrade.  Modules that should
be independent develop dependencies on each other.  Layers that should be
strictly separated become entangled.  Architecture tests catch these
violations automatically by verifying the dependency graph against declared
rules.

---

## The Problem: Architecture Drift

Consider a project with three layers:

```
routes  →  services  →  repositories
```

The rule is clear: routes depend on services, services depend on repositories,
and no layer should skip.  But over time, someone imports a repository
directly from a route:

```rust
// routes/user_routes.rs
use crate::repositories::UserRepository;  // Violation!
```

This compiles and works.  The tests pass.  But the architecture is now
compromised.  Future changes to the repository layer may break the route
layer, which was supposed to be isolated.

---

## Architecture Tests with `rvtest::arch`

`rvtest::arch` lets you declare dependency rules as code:

```rust
use rvtest::arch::arch_check;

#[test]
 fn architecture_is_enforced() {
     arch_check()
         .module("routes").may_depend_on(&["services"])
         .module("services").may_depend_on(&["repositories"])
         .module("repositories").may_depend_on(&[])
         .all_modules().must_not_have_cycles()
         .all_modules().public_api_doc_required()
         .assert_all_pass();
 }
```

If any module violates a rule, the test fails:

```
Architecture violations:
  routes must not depend on repositories (allowed: services)
  cycle detected: repositories → services → repositories
```

---

## How It Works

`arch_check()` scans your `src/` directory for `.rs` files, parses `mod`
declarations and `use crate::...` statements using lightweight text analysis
(no `syn` dependency), and builds a directed graph of module dependencies.

It then checks each declared rule against the actual graph and reports
violations with the offending import path.

---

## Available Rules

| Rule | Description |
|------|-------------|
| `may_depend_on(&[...])` | Module can only depend on listed peers |
| `may_not_depend_on(&[...])` | Module cannot depend on listed peers |
| `must_not_have_cycles()` | No circular dependencies anywhere |
| `public_api_doc_required()` | All public items must have doc comments |

---

## Detecting Cycles

Circular dependencies are a common architectural problem:

```rust
// a.rs
mod b; // a depends on b

// b.rs
mod a; // b depends on a — cycle!
```

Detect with:

```rust
#[test]
 fn no_circular_dependencies() {
     arch_check()
         .all_modules().must_not_have_cycles()
         .assert_all_pass();
 }
```

---

## Requiring Documentation

Ensure all public API has doc comments:

```rust
#[test]
 fn public_api_is_documented() {
     arch_check()
         .all_modules().public_api_doc_required()
         .assert_all_pass();
 }
```

This scans for `pub fn`, `pub struct`, `pub enum`, `pub trait`, and other
public items, and verifies each has a preceding `///` or `//!` comment.

---

## Real-World Example

Here is the architecture test `rvtest` uses on itself:

```rust
#[test]
 fn rvtest_architecture() {
     arch_check()
         // core is the foundation — no internal dependencies
         .module("core").may_depend_on(&[])
         // spec depends on core and tag
         .module("spec").may_depend_on(&["core", "tag"])
         // runner depends on core, report, and spec
         .module("runner").may_depend_on(&["core", "report", "spec"])
         // coverage depends on core and coverage_raw
         .module("coverage").may_depend_on(&["core", "coverage_raw"])
         // report depends only on core
         .module("report").may_depend_on(&["core"])
         // no cycles anywhere
         .all_modules().must_not_have_cycles()
         // public API must be documented
         .all_modules().public_api_doc_required()
         .assert_all_pass();
 }
```

---

## When to Add Architecture Tests

Add architecture tests early — ideally in the first week of a project.  Once
architectural violations accumulate, fixing them is expensive.  The test
serves as a contract that future developers (including your future self) must
respect.

If you are adding architecture tests to an existing project, start with a
permissive rule and tighten it over time:

```rust
#[test]
 fn architecture_boundaries() {
     arch_check()
         // Start with just cycles — the least restrictive check
         .all_modules().must_not_have_cycles()
         // Gradually add module rules as violations are fixed
         // .module("core").may_depend_on(&[])
         // .module("services").may_depend_on(&["core"])
         .assert_all_pass();
 }
```

---

## Custom Source Directory

For non-standard crate layouts, set the source directory:

```rust
arch_check()
    .src_dir("my_crate/src")
    .module("internal").may_not_depend_on(&["external"])
    .assert_all_pass();
```

---

## Summary

- Architecture tests prevent module dependency violations
- `rvtest::arch` scans `src/` and builds a dependency graph
- Declare rules with `may_depend_on`, `may_not_depend_on`,
  `must_not_have_cycles`, `public_api_doc_required`
- Add architecture tests early in a project's lifecycle
- Start permissive and tighten rules over time
- Architecture tests are fastest — they only scan files, no runtime

In the next chapter, we explore testing concurrent code — race conditions,
async code, and ensuring determinism.

---

[← Previous](18-ci-integration.md) • [Index](00-index.md) • [Next →](20-testing-concurrent-code.md)
