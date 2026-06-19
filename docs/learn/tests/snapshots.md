# Snapshots

File-based assertions — compare output against a stored reference ("snapshot").

## Prerequisites

- [Why Test](why-test.md) — test basics


## Basic Snapshot Test

```rust
use rvtest::snapshot::assert_snapshot;

#[test]
fn render_html() {
    let output = render_page("hello", "world");
    assert_snapshot("render_html", &output);
}
```

On first run, the snapshot is created. On subsequent runs, output is compared against the stored file. If they match, the test passes. If not, it fails with a diff.

## Workflow

1. Run tests: `cargo test`
2. If output changed intentionally: `cargo rvtest --update-all`
3. Review changes: `cargo rvtest --review`
4. Commit updated snapshot files alongside code changes

## Custom Directory

```rust
use rvtest::snapshot::assert_snapshot_in;

assert_snapshot_in("snapshots/render", "html_output", &output);
```

## When to Use Snapshots

| Good For | Not Good For |
|----------|-------------|
| HTML/XML/serialized output | Highly dynamic output (timestamps, random IDs) |
| Error messages and diagnostics | Data that varies by platform |
| Generated code or configs | Very large files (use file comparison instead) |
| Regression detection | The first snapshot needs manual verification |

## Glossarium

| Term | Definition |
|------|------------|
| Snapshot | A stored file containing the expected output of a test. |
| `--update-all` | Auto-accept all snapshot changes — overwrite stored files with current output. |
| `--review` | Interactive mode to review each snapshot change one by one. |


## Next Steps

- [Coverage](coverage.md) — measuring code coverage
- [CI Integration](ci-integration.md) — running tests in CI pipelines
