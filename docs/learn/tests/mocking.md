# Mocking

Isolating tests from external dependencies — spies, stubs, and scoped function replacement.

## Prerequisites

- [Why Test](why-test.md) — test basics, `#[test]`


## Spy — Record Calls

```rust
use rvtest::mock::spy;

let s = spy(|x: i32| x * 2);
assert_eq!(s(21), 42);
s.assert_called_with(&[21]);
```

Spies are useful when you need to verify that a function was called with specific arguments.

## Stub — Fixed Return

```rust
use rvtest::mock::stub;

let s = stub(|| 42);
assert_eq!(s(), 42);
```

Stubs return the same value every time — useful for replacing external services with deterministic responses.

## `patch!` — Scoped Replacement

```rust
use rvtest::mock::patch;

fn get_user(id: u32) -> String {
    format!("user-{id}")
}

#[test]
fn test_with_patch() {
    let result = patch!(get_user, |id: u32| format!("mock-{id}"), || {
        // Inside this closure, calls to get_user go through the mock
        get_user(42)
    });
    assert_eq!(result, "mock-42");

    // Outside the closure, get_user is restored
    assert_eq!(get_user(42), "user-42");
}
```

## Integrating with BDD Specs

```rust
describe("User Service")
    .it("returns user from API", || {
        let mut api_called = false;
        let result = patch!(fetch_user, |id: u32| {
            api_called = true;
            User { id, name: "Alice".into() }
        }, || {
            get_user_profile(42)
        });
        assert!(api_called);
    })
    .run()
    .assert_all_pass();
```

## Glossarium

| Term | Definition |
|------|------------|
| Spy | Records calls made to a function for later inspection. |
| Stub | A function that returns a fixed value regardless of input. |
| `patch!` | Temporarily replaces a function with a mock in a scoped block. |
| Call Tracking | Recording arguments, call count, and return values of a spy. |


## Next Steps

- [Snapshots](snapshots.md) — file-based assertions for stable output
- [Coverage](coverage.md) — measuring how much code your tests exercise
