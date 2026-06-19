# Chapter 16 — Benchmark and Regression

[← Previous](15-flaky-tests.md) • [Index](00-index.md) • [Next →](17-legacy-and-refactoring.md)

---

Performance matters.  A function that passes the correctness test but takes
100× longer than before is still a bug.  This chapter covers how to measure
test execution time and detect performance regressions using `rvtest`'s
built-in profiling tools.

---

## The Problem: Performance Regressions

Consider a function that sorts a list:

```rust
fn process_items(items: &mut [i32]) {
    items.sort();
    // More processing...
}
```

A developer changes the implementation:

```rust
fn process_items(items: &mut [i32]) {
    // New "optimised" version — but is it actually faster?
    for i in 0..items.len() {
        for j in i + 1..items.len() {
            if items[i] > items[j] {
                items.swap(i, j);
            }
        }
    }
}
```

The new version is O(n²) instead of O(n log n).  Correctness tests still
pass.  But the performance is dramatically worse.  Without measuring, you
would not notice until production.

---

## Slow Test Profiling with `rvtest`

`rvtest` can identify the slowest tests in your suite:

```bash
# Show the 5 slowest tests (default)
cargo rvtest --profile-slow

# Show the 10 slowest tests
cargo rvtest --profile-slow=10
```

Output:

```
  ⏱  Slowest tests
    1.   2.34s  Database :: insert_large_batch
    2.   1.12s  API :: full_integration_flow
    3.   0.89s  Renderer :: complex_svg_output
```

This helps you identify tests that may need optimisation or that may be
slowing down your CI pipeline.

---

## Using `rvtest`'s Profiling for Performance Regression Detection

Run with `--profile-slow` and save the output.  On the next run, compare the
durations.  A test that suddenly takes significantly longer indicates a
performance regression.

```bash
# Baseline run
cargo rvtest --profile-slow > baseline.txt

# After changes
cargo rvtest --profile-slow > current.txt
diff baseline.txt current.txt
```

---

## Writing Benchmark-Style Tests

For fine-grained performance measurement, write focused tests that measure
specific operations:

```rust
#[test]
 fn benchmark_sort_small() {
     let mut data = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
     let start = std::time::Instant::now();
     data.sort();
     let duration = start.elapsed();
     assert!(duration.as_micros() < 100, "sorting took too long: {duration:?}");
 }
```

This is a simple way to catch performance regressions without adding a
benchmarking framework.  If the sort takes longer than 100 microseconds, the
test fails.

---

## When to Extract Benchmarks

If you find yourself writing many timing assertions, consider moving them to
a dedicated benchmark suite:

```rust
// tests/benchmarks.rs or benchmarks/ directory

#[test]
 fn benchmark_serialization() {
     let data = generate_large_dataset();

     let start = std::time::Instant::now();
     let encoded = serialize(&data);
     let encode_time = start.elapsed();

     let start = std::time::Instant::now();
     let decoded: Dataset = deserialize(&encoded);
     let decode_time = start.elapsed();

     assert_eq!(data, decoded);
     assert!(encode_time.as_millis() < 500, "encode took too long");
     assert!(decode_time.as_millis() < 500, "decode took too long");
 }
```

---

## CI Performance Regression Detection

In CI, you can track test durations across commits:

```yaml
# .github/workflows/ci.yml
- name: Run tests with profiling
  run: |
    cargo rvtest --profile-slow=10 > profile-${{ github.sha }}.txt

- name: Upload profile
  uses: actions/upload-artifact@v4
  with:
    name: profile-${{ github.sha }}
    path: profile-*.txt
```

Over time, you can build a history of test durations and detect regressions
programmatically.

---

## Distinguishing Noise from Regression

Test durations vary run-to-run due to system load, CPU frequency scaling, and
other factors.  A single slow run is noise.  A consistently slower run across
multiple CI executions is a regression.

| Pattern | Likely Cause |
|---------|-------------|
| One test 2× slower once | Noise (system load) |
| One test 2× slower consistently | Regression in that test's code |
| All tests 1.5× slower | System-level issue (CI runner performance) |
| One test goes from 100ms to 10s | Algorithmic regression (O(n) → O(n²)) |

---

## Common Performance Pitfalls

### Accidental Allocation in Hot Paths

```rust
// ❌ Allocates a new String every call
fn format_name(first: &str, last: &str) -> String {
    format!("{first} {last}")
}
```

### Unnecessary Data Copies

```rust
// ❌ Clones the entire vector
fn process(data: Vec<i32>) -> Vec<i32> {
    let mut result = data.clone(); // Unnecessary
    result.sort();
    result
}
```

### Using the Wrong Data Structure

```rust
// ❌ O(n) lookup instead of O(1)
fn find_user(users: &Vec<User>, id: u32) -> Option<&User> {
    users.iter().find(|u| u.id == id)
}
```

Performance tests can catch these regressions automatically.

---

## Summary

- Performance regressions are bugs — test for them
- `cargo rvtest --profile-slow` identifies the slowest tests
- Write timing assertions for performance-critical code
- Track durations across CI runs to catch regressions
- Distinguish noise from regressions by looking for consistent patterns
- Test performance for algorithmic changes (O(n) → O(n²) is often invisible
  in correctness tests)

In the next chapter, we will look at strategies for adding tests to existing
codebases — including code that was not designed for testability.

---

[← Previous](15-flaky-tests.md) • [Index](00-index.md) • [Next →](17-legacy-and-refactoring.md)
