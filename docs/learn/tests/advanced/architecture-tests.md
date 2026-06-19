# Architecture Tests

Enforcing module dependency rules — ensuring your codebase stays well-structured over time.

## Prerequisites

- [Test Organization](../basics/test-organization.md) — test structure basics

## Glossarium

| Term | Definition |
|------|------------|
| `may_depend_on` | Asserts a module may only depend on listed modules. |
| `may_not_depend_on` | Asserts a module must not depend on listed modules. |
| `must_not_have_cycles` | Asserts there are no circular dependencies between modules. |
| `public_api_doc_required` | Asserts all public items have documentation comments. |

## Dependency Rules

```rust
use rvtest::arch::{may_depend_on, may_not_depend_on};

#[test]
fn module_dependencies() {
    may_depend_on("algebra", &["num", "expr"]);
    may_not_depend_on("algebra", &["utils", "geometry"]);
}
```

## Cycle Detection

```rust
use rvtest::arch::must_not_have_cycles;

#[test]
fn no_circular_deps() {
    must_not_have_cycles();
}
```

This parses `use` statements in your crate and verifies there are no circular module dependencies.

## Documentation Enforcement

```rust
use rvtest::arch::public_api_doc_required;

#[test]
fn all_public_items_documented() {
    public_api_doc_required();
}
```

Run as part of CI to ensure every public function, struct, and trait has a doc comment.

## Combining Rules

```rust
use rvtest::spec::describe;

#[test]
fn architecture() {
    describe("Module Architecture")
        .it("prevents illegal dependencies", || {
            may_not_depend_on("algebra", &["utils", "geometry"]);
        })
        .it("allows legal dependencies", || {
            may_depend_on("algebra", &["num", "expr"]);
        })
        .it("no circular dependencies", || {
            must_not_have_cycles();
        })
        .it("all public items documented", || {
            public_api_doc_required();
        })
        .tag("arch")
        .run()
        .assert_all_pass();
}
```

## Next Steps

- [Testing Concurrent Code](concurrent-code.md) — testing multithreaded and async code
- [Faster Feedback](../workflow/faster-feedback.md) — speeding up the test cycle
