# Chapter 21 — Faster Feedback Loops

[← Previous](20-testing-concurrent-code.md) • [Index](00-index.md)

---

The faster your tests run, the more often you run them.  A test suite that
takes 30 seconds is run hundreds of times a day.  A test suite that takes 10
minutes is run a few times a day, typically right before a commit.  This
chapter covers techniques for minimising the time between making a change and
knowing whether it is correct.

---

## Measuring Your Current Cycle Time

Before optimising, measure your current feedback loop:

```bash
# Time a full test run
time cargo rvtest

# Time compilation only
time cargo test --no-run

# Time test execution only (after compilation)
time cargo test --no-run && time cargo test
```

Typical breakdown:

| Phase | Time | Optimisation |
|-------|------|-------------|
| Dependency resolution | 0.5-2s | Cargo caches this |
| Compilation | 10-120s | `--fast`, incremental, Cranelift |
| Linking | 1-10s | `--fast` (mold/lld) |
| Test execution | 0.1-60s | Test selection, parallelism, `--retest` |

---

## Fast Mode

`rvtest`'s `--fast` flag combines several compile-time optimisations:

```bash
cargo rvtest --fast
```

This enables:
- Auto-detection of `mold` or `lld` linker (3-10× faster linking)
- Disabling debug info (`CARGO_PROFILE_DEV_DEBUG=0`) — 30-40% faster rebuilds

For nightly users, add Cranelift:

```bash
cargo +nightly rvtest --fast --cranelift --parallel-frontend 8
```

---

## Watch Mode

`cargo rvtest --watch` re-runs tests automatically when files change:

```bash
cargo rvtest --watch
```

Keybindings:
- `q` — quit
- `r` — force re-run
- `f` — set a new name filter

This eliminates the manual `cargo test` → wait → inspect cycle.

---

## Daemon Mode

`cargo rvtest --daemon` is a persistent compile daemon that keeps test
binaries warm:

```bash
cargo rvtest --daemon
```

Unlike watch mode (which runs `cargo test` on every change), daemon mode
builds test binaries once, then runs them directly on subsequent changes --
bypassing `cargo test`'s startup overhead.

| Aspect | `--watch` | `--daemon` |
|--------|-----------|------------|
| Build | `cargo test` | `cargo build --tests` |
| Execution | via `cargo test` | Direct binary execution |
| Startup overhead | ~37ms cargo startup | Minimal |
| Best for | Small projects | Large projects |

---

## Test Selection

### Run Only What Changed

Run tests related to specific code:

```bash
# Run tests with "auth" in the name
cargo rvtest -f auth

# Run a specific test module
cargo rvtest -f "tests::database"

# Run tests matching a tag
cargo rvtest -t database
```

### Run Only What Failed

After a failed run, re-run only the failing tests:

```bash
cargo rvtest --retest
# or
cargo rvtest --failed
```

This is the fastest way to iterate on a fix.

### Compare Against Previous Run

See what changed since the last test run:

```bash
cargo rvtest --diff
```

This shows:
- **New failures** — tests that passed before but fail now
- **Recovered tests** — tests that failed before but pass now
- **Slower tests** — tests whose duration increased significantly
- **Faster tests** — tests whose duration decreased significantly

The diff is computed against the last run's cached results.

### Git-Aware Test Selection

Automatically select tests based on which files changed:

```bash
cargo rvtest --changed
```

This runs `git diff --name-only HEAD`, extracts module names from changed
files, and passes them as a filter to `cargo test`.  Only tests related to
changed modules are executed.

### Skip Irrelevant Tests

```bash
# Skip slow tests by tag
cargo rvtest -E slow

# Skip tests by name pattern (complement to --filter)
cargo rvtest --skip slow

# Randomise test execution order (detects ordering dependencies)
cargo rvtest --shuffle
```

---

## Running Tests in Parallel

By default, `rvtest` runs tests in parallel using all available CPUs.
Control parallelism:

```bash
# Run with 4 threads
cargo rvtest --max-threads 4

# Run sequentially (useful for shared resources)
cargo rvtest --no-parallel
```

---

## Fail-Fast Mode

Stop at the first failure:

```bash
cargo rvtest --fail-fast
```

This saves time by not running the remaining tests when one already failed.
Useful during development; less useful in CI where you want a full report.

---

## Profile Presets

Switch between configurations with `--profile`:

```bash
# Development: pretty output, verbose
cargo rvtest --profile dev

# CI: JUnit output, fail-fast
cargo rvtest --profile ci
```

Or via environment variable:

```bash
export RVTEST_PROFILE=dev
```

## Colour Control

Explicitly control coloured output:

```bash
cargo rvtest --color always    # Force colours (e.g., for CI output capture)
cargo rvtest --color never     # Plain text (e.g., for piping to a file)
cargo rvtest --color auto      # Let the terminal decide (default)
```

`rvtest` also respects the `CARGO_TERM_COLOR` environment variable.

---

## Continuous Testing Workflow

Here is an efficient workflow for rapid iteration:

```bash
# Terminal 1: Run tests in daemon mode
cargo rvtest --daemon --profile dev

# Terminal 2: Edit code
# Every save triggers a rebuild and re-run in Terminal 1
```

Or with watch mode and filter:

```bash
# Run only the tests you are working on
cargo rvtest --watch -f auth_service
```

---

## Profiling Slow Tests

Identify your slowest tests to know what to optimise:

```bash
# Show the 5 slowest tests (default)
cargo rvtest --profile-slow

# Show the 10 slowest tests
cargo rvtest --profile-slow=10
```

Output:

```
  ⏱ Slowest tests
    1.   2.34s  database > complex_query
    2.   1.89s  api > full_integration
    3.   0.95s  file_system > large_file_io
```

Use this to find tests that should be optimised, moved to `#[ignore]`,
or separated from the fast feedback loop.

---

## Listing Tests Without Running

Discover available tests before deciding what to run:

```bash
# List all tests
cargo rvtest --list

# List tests matching a filter
cargo rvtest --list -f database
```

Example output:

```
  database :: test_connection
  database :: test_query
  database :: test_migration
  api :: test_login
  api :: test_signup

  5 test(s) listed.
```

This is useful for scripting, CI configuration, or simply understanding
your test inventory.

---

## Reproducible Randomness

Tests that use randomness (property tests, shuffled execution) can be
made deterministic with a fixed seed:

```bash
# Run with a specific seed
cargo rvtest --seed 42

# When a property test fails, rvtest prints the seed
# Re-run with that seed to reproduce the exact failure
cargo rvtest --seed 12345678
```

The `--seed` flag affects:
- Property-based test input generation
- `--shuffle` execution order

This makes randomised features reproducible for debugging.

---

## Workspace-Aware Testing

In a Cargo workspace, tests are usually run per-crate.  Run all workspace
members at once:

```bash
cargo rvtest --workspace
```

This is equivalent to `cargo test --workspace` but with rvtest's formatting
and reporting.  Results from all crates are aggregated into a single report.

---

## Caching Strategies

Rust's incremental compilation already caches unchanged code.  But test
binaries are rebuilt even when only the test code changes.  `--daemon` mode
mitigates this by caching compiled test binaries.

For persistent caching across CI runs:

```yaml
- name: Cache dependencies
  uses: actions/cache@v4
  with:
    path: |
      ~/.cargo/registry
      ~/.cargo/git
      target
    key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
```

---

## When Optimisation Is Not Worth It

Not all tests need to be fast.  Some tests are inherently slow:

- **Integration tests** that start a database
- **End-to-end tests** that deploy to a staging environment
- **Property-based tests** with many iterations

Separate these from your rapid feedback loop:

```rust
#[test]
#[ignore] // Run separately
 fn slow_integration_test() {
     // ...
 }
```

Run ignored tests only before pushing:

```bash
cargo test -- --ignored
```

---

## The Ideal Feedback Loop

For most development work, aim for:

| Phase | Target Time |
|-------|-------------|
| Code change → compile | < 5 seconds |
| Compile → test result | < 2 seconds |
| Total cycle | < 10 seconds |

With `--daemon`, `--retest`, and incremental compilation, this is achievable
for all but the largest projects.

---

## Summary

- Measure before optimising — know where time is spent
- Use `--fast` for faster compilation (linker + debug info)
- Use `--watch` or `--daemon` for automatic re-runs
- Use `-f filter`, `--retest`, `-E exclude` to run only relevant tests
- Use `--fail-fast` to stop at the first failure
- Use `--profile` to switch between development and CI configurations
- Separate slow tests from the rapid feedback loop
- Aim for a < 10 second cycle for day-to-day development

---

[← Previous](20-testing-concurrent-code.md) • [Index](00-index.md)
