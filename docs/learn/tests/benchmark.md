# Benchmark and Regression

Measuring test performance — duration tracking, baselines, and regression detection.

## Prerequisites

- [Flaky Tests](flaky-tests.md) — test reliability basics

## Glossarium

| Term | Definition |
|------|------------|
| Baseline | A saved record of test durations from a known-good run. |
| Regression | A test that has become significantly slower than its baseline. |
| `--profile-slow` | CLI flag to surface the N slowest tests. |
| Benchmark | A test that measures performance and asserts it stays within bounds. |

## Profiling Slow Tests

```bash
# Show the 10 slowest tests
cargo rvtest --profile-slow 10
```

## Baseline Comparison

Save a baseline and compare subsequent runs:

```bash
# Save current durations as baseline
cargo rvtest --save-baseline

# Compare against baseline — flag regressions
cargo rvtest --compare-baseline
```

## Benchmark Specs

rvtest supports benchmarking within `describe`/`it`:

```rust
describe("Sorting")
    .bench("quicksort", || {
        let mut v: Vec<i32> = (0..1000).collect();
        v.sort();
    })
    .bench_iterations(100) // run 100 times, report average
    .bench_threshold(2.0)  // warn if 2x slower than baseline
    .run()
    .assert_all_pass();
```

## Benchmark Regression Detection

| Metric | Description |
|--------|-------------|
| Duration | Absolute time per test in milliseconds |
| Comparison | Ratio vs baseline (e.g., 1.5x = 50% slower) |
| Threshold | Configurable limit before marking as regression |
| Distribution | Min, max, mean, median across multiple runs |

## Next Steps

- [Architecture Tests](architecture-tests.md) — enforcing module dependency rules
- [CI Integration](ci-integration.md) — running tests in CI pipelines
