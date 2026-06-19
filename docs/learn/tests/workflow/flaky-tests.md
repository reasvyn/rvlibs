# Flaky Tests

Tests that sometimes pass and sometimes fail — detection, quarantine, and management.

## Prerequisites

- [BDD Specs](../patterns/bdd-specs.md) — `describe`/`it`, retries

## Glossarium

| Term | Definition |
|------|------------|
| Flaky Test | A test that passes and fails intermittently without code changes. |
| Quarantine | A list of known-flaky tests that are skipped by default. |
| Detection | Running the suite multiple times to find tests with inconsistent results. |
| Retry | Re-running a failing test immediately to see if it's transient. |

## Retries

For known-flaky tests, set retries:

```rust
describe("Network")
    .it("fetches data from unreliable service", || {
        // might fail transiently
    })
    .retries(3) // retry up to 3 times before marking as failed
    .run()
    .assert_all_pass();
```

## Detecting Flaky Tests

```bash
cargo rvtest --detect-flaky
```

This runs the suite multiple times (default 10) and reports pass rates:

```
Flaky test detection results:
  tests::network::fetch_data: 70% (7/10 passes) ⚠️ flaky
  tests::database::insert: 100% (10/10 passes) ✅ stable
```

Custom run count:

```bash
cargo rvtest --detect-flaky 20
```

## Quarantine

Move known-flaky tests to quarantine so they don't block CI:

```bash
# Skip quarantined tests
cargo rvtest --quarantine

# List quarantined tests
cargo rvtest --flaky-report

# Run quarantined tests anyway (to verify they're fixed)
cargo rvtest --include-flaky

# Clear quarantine list (after fixing flaky tests)
cargo rvtest --unquarantine
```

## Dealing with Flaky Tests

| Cause | Solution |
|-------|----------|
| Timing/race conditions | Add retries or fix the race condition |
| Network dependency | Mock the network call |
| Random data | Use a fixed seed (`--seed`) |
| Shared mutable state | Isolate tests with fresh state |
| External service flakiness | Use test doubles or containerized services |

## Next Steps

- [Benchmark](../advanced/benchmark.md) — performance regression testing
- [CI Integration](ci-integration.md) — running tests in CI pipelines
