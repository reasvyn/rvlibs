# Chapter 13 — Snapshot Testing

[← Previous](12-property-based-testing.md) • [Index](00-index.md) • [Next →](14-coverage.md)

---

Snapshot testing is a pattern where you save a test's output to a file (the
"snapshot") and compare future runs against it.  Instead of writing
assertions for every field of a complex value, you let the computer record
what the output was and alert you when it changes.

---

## The Problem: Testing Large Outputs

Consider testing a function that generates HTML, JSON, or a complex report:

```rust
fn render_user_profile(user: &User) -> String {
    format!(
        "<div class='profile'>\
           <h1>{}</h1>\
           <p>Email: {}</p>\
           <p>Member since: {}</p>\
         </div>",
        user.name, user.email, user.join_date
    )
}
```

How would you test this?  You could write an assertion for the exact HTML
string:

```rust
#[test]
 fn test_render_profile() {
     let user = User { name: "Alice".into(), email: "alice@example.com".into(), /* ... */ };
     let html = render_user_profile(&user);
     assert_eq!(html, "<div class='profile'><h1>Alice</h1><p>Email: ...</p></div>");
 }
```

This is fragile — any change to the HTML structure requires updating the
assertion in the test.  And for large outputs, the assertion string becomes
unmanageable.

---

## Introducing Snapshot Testing

With `rvtest::snapshot`, you store the expected output in a file:

```rust
use rvtest::snapshot::assert_snapshot;

#[test]
 fn test_render_profile() {
     let user = User { name: "Alice".into(), email: "alice@example.com".into(), /* ... */ };
     let html = render_user_profile(&user);
     assert_snapshot("render_user_profile_alice", &html);
 }
```

The first time you run this test, it creates a snapshot file at
`.snapshots/render_user_profile_alice.snap` with the actual output.  The test
**fails** because the snapshot is new and needs review:

```
snapshot `render_user_profile_alice` created at ".snapshots/render_user_profile_alice.snap".
Review the content and commit the snapshot file.
Use `--update-all` to auto-accept new snapshots.
```

---

## The Snapshot Workflow

### First Run: Create Snapshot

```bash
$ cargo test
  --- snapshot `render_user_profile_alice` created ---
```

Inspect the generated file:

```html
<!-- .snapshots/render_user_profile_alice.snap -->
<div class='profile'>
  <h1>Alice</h1>
  <p>Email: alice@example.com</p>
  <p>Member since: 2024-01-15</p>
</div>
```

### Accept the Snapshot

If the output looks correct, accept it with `--update-all`:

```bash
$ cargo rvtest --update-all
```

Or in review mode (accept/reject per snapshot):

```bash
$ cargo rvtest --review
  Snapshot `render_user_profile_alice` mismatch:
  Accept new snapshot? [y/N]
```

Once accepted, the snapshot file is committed to version control alongside
your code.

### Subsequent Runs: Compare

On future runs, the test compares the actual output against the saved
snapshot.  If the output changed, the test fails and shows a diff:

```
snapshot `render_user_profile_alice` mismatch!
expected (snapshot)
actual (new)
.snapshots/... | <h1>Alice</h1>
.snapshots/... | <h1>Alice Smith</h1>

Rerun with `--update-all` to accept the new snapshot.
```

---

## When to Use Snapshot Testing

| Good For | Not Good For |
|----------|-------------|
| HTML/XML/JSON output | Security-sensitive data |
| Serialised data structures | Large binary output |
| Error messages | Non-deterministic output (timestamps, random IDs) |
| Rendered UI components | Output that changes on every run |
| Code generation | Output containing absolute file paths |
| Configuration files | Output containing dates or times |

---

## Deterministic Snapshots

Snapshots must be deterministic.  An output that changes on every run
(such as a timestamp) will produce a failing snapshot every time:

```rust
// ❌ Non-deterministic — timestamp changes
fn render_with_timestamp(content: &str) -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    format!("<div>{content}</div><footer>Generated at {now}</footer>")
}
```

Fix by separating the non-deterministic part:

```rust
fn render_with_timestamp(content: &str, now: u64) -> String {
    format!("<div>{content}</div><footer>Generated at {now}</footer>")
}

#[test]
 fn test_render_content() {
     let output = render_with_timestamp("Hello", 1000);
     assert_snapshot("render_content", &output);
 }
```

---

## Snapshot Naming

Choose descriptive, unique names for your snapshots:

```rust
assert_snapshot("render_profile_admin", &admin_html);
assert_snapshot("render_profile_guest", &guest_html);
assert_snapshot("render_error_404", &error_html);
assert_snapshot("render_error_500", &error_html);
```

Names are sanitised for the filesystem — special characters are replaced with
underscores.

---

## Custom Snapshot Directory

By default, snapshots are stored in `.snapshots/`.  Override with
`set_snapshot_dir`:

```rust
use rvtest::snapshot::set_snapshot_dir;

// In a test helper or before tests
set_snapshot_dir("tests/snapshots");
```

---

## Snapshot Testing with `describe`

Combine snapshots with BDD specs for organised test suites:

```rust
use rvtest::spec::describe;
use rvtest::snapshot::assert_snapshot;

#[test]
 fn snapshot_spec() {
     describe("HTML rendering")
         .describe("user profile")
             .it("renders admin profile", || {
                 let user = User { name: "Admin".into(), role: Role::Admin };
                 assert_snapshot("admin_profile", &render_profile(&user));
             })
             .it("renders guest profile", || {
                 let user = User { name: "Guest".into(), role: Role::Guest };
                 assert_snapshot("guest_profile", &render_profile(&user));
             })
         .describe("error pages")
             .it("renders 404 page", || {
                 assert_snapshot("error_404", &render_error(404));
             })
         .run()
         .assert_all_pass();
 }
```

---

## Snapshot File Management

The `.snapshots/` directory should be committed to version control.  Review
snapshot changes during code review just like source code changes.

```bash
# Structure
.snapshots/
├── admin_profile.snap
├── guest_profile.snap
├── error_404.snap
└── render_user_profile_alice.snap
```

---

## A Practical Example: Testing JSON Serialisation

```rust
use serde::Serialize;
use rvtest::snapshot::assert_snapshot;

#[derive(Serialize)]
struct Order {
    id: u32,
    customer: String,
    items: Vec<String>,
    total: f64,
}

fn serialize_order(order: &Order) -> String {
    serde_json::to_string_pretty(order).unwrap()
}

#[test]
 fn test_order_json() {
     let order = Order {
         id: 1,
         customer: "Alice".into(),
         items: vec!["Widget".into(), "Gadget".into()],
         total: 49.99,
     };
     assert_snapshot("order_json", &serialize_order(&order));
 }
```

The snapshot file will contain:

```json
{
  "id": 1,
  "customer": "Alice",
  "items": [
    "Widget",
    "Gadget"
  ],
  "total": 49.99
}
```

When the order structure changes (adding a field, changing JSON format), the
snapshot test will fail, prompting you to review and accept the new output.

---

## Common Pitfalls

### Snapshots That Are Too Large

Large snapshot files (thousands of lines) are hard to review.  If a snapshot
is too large, consider testing individual sections separately.

### Snapshots That Are Too Small

Tiny snapshots (a single number or boolean) are better served by regular
assertions.

### Forgetting to Update Snapshots After Intentional Changes

After a deliberate change to the output, run `--update-all` or `--review` to
accept the new snapshots.  Failing to do so will break CI.

### Ignoring Snapshot Changes in Code Review

A snapshot change should be reviewed as carefully as a source code change.
Ask: "Does the new output make sense?"

---

## Summary

- Snapshot testing records output to a file and compares future runs against it
- First run creates the snapshot (and fails) — review and accept
- Subsequent runs compare against the saved snapshot
- Use `--update-all` to mass-accept, `--review` for interactive acceptance
- Snapshots must be deterministic — strip timestamps, random IDs, etc.
- Store snapshot files in version control
- Combine with `describe` blocks for organised snapshot suites

In the next chapter, we move to Part 4 and explore code coverage — measuring
which parts of your code are exercised by tests.

---

[← Previous](12-property-based-testing.md) • [Index](00-index.md) • [Next →](14-coverage.md)
