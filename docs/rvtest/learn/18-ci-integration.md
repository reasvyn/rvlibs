# Chapter 18 — CI Integration

[← Previous](17-legacy-and-refactoring.md) • [Index](00-index.md) • [Next →](19-architecture-tests.md)

---

A test suite is only useful if it runs regularly.  Continuous Integration (CI)
automates test execution on every commit, giving you immediate feedback when
something breaks.  This chapter covers how to configure `rvtest` for various
CI environments and how to get the most out of your CI pipeline.

---

## Basic CI Setup

Every CI pipeline should run tests as a minimum:

```yaml
# .github/workflows/ci.yml
name: CI
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rust-lang/setup-rust-toolchain@v1

      - name: Run tests
        run: cargo rvtest
```

This runs your full test suite on every push and pull request.

---

## CI Profiles with `rvtest`

Use the `--profile` flag to switch between configurations:

```bash
# Development (local): pretty output, verbose
cargo rvtest --profile dev

# CI: JUnit output, fail-fast
cargo rvtest --profile ci
```

The `ci` profile sets:
- `--format junit` — machine-readable output for CI dashboards
- `--fail-fast` — stop on first failure to save CI minutes

You can also set the profile via environment variable:

```bash
export RVTEST_PROFILE=ci
cargo rvtest
```

---

## Output Formats for CI

### JUnit XML

JUnit XML is understood by most CI systems:

```bash
cargo rvtest --format junit > results.xml
```

GitHub Actions (using `dorny/test-reporter`):

```yaml
- name: Run tests
  run: cargo rvtest -F junit > results.xml

- name: Publish test results
  uses: dorny/test-reporter@v1
  if: always()
  with:
    name: Rust Tests
    path: results.xml
    reporter: java-junit
```

### GitHub Actions Annotations

```bash
cargo rvtest --format github
```

This produces `::error` annotations inline in GitHub Actions output:

```
::error file=src/calculator.rs,line=42,title=Calculator :: adds — assertion failed
```

These annotations appear directly on the pull request diff, showing exactly
which line caused the failure.

### JSON

For custom processing and AI-assisted debugging:

```bash
# Standard JSON (includes location and captured output)
cargo rvtest --format json > results.json
```

Example JSON output:

```json
{
  "success": false,
  "total": 42,
  "passed": 40,
  "failed": 2,
  "duration_secs": 3.45,
  "suites": [{
    "name": "Calculator",
    "tests": [{
      "name": "Calculator :: adds",
      "status": "passed",
      "duration_secs": 0.002
    }, {
      "name": "Calculator :: divides_by_zero",
      "status": "failed",
      "duration_secs": 0.001,
      "reason": "assertion failed: expected Err, got Ok(inf)",
      "location": {
        "file": "tests/calculator.rs",
        "line": 42
      },
      "captured_output": "stdout:\nDebug output from test..."
    }]
  }]
}
```

### Agent-Native JSON (LLM-Optimised)

For AI-assisted debugging workflows:

```bash
cargo rvtest --format agent > results_agent.json
```

The Agent format extends the standard JSON with:

| Field | Description |
|-------|-------------|
| `source_snippet` | Source code around each failure location (3 lines of context) |
| `analysis.is_failure` | Boolean flag for quick filtering |
| `analysis.is_flaky_candidate` | Reserved for future flaky analysis |
| `format` | Set to `"rvtest-agent-v1"` for version detection |

Example agent output:

```json
{
  "format": "rvtest-agent-v1",
  "success": false,
  "total": 1,
  "passed": 0,
  "failed": 1,
  "suites": [{
    "name": "Calculator",
    "tests": [{
      "name": "Calculator :: divides_by_zero",
      "status": "failed",
      "duration_secs": 0.001,
      "failure": "assertion failed: expected Err, got Ok(inf)",
      "location": { "file": "tests/calculator.rs", "line": 42 },
      "source_snippet": "   40:     let result = divide(1.0, 0.0);\n   41: \n→  42:     assert!(result.is_err());\n   43: }\n",
      "analysis": {
        "is_failure": true,
        "is_flaky_candidate": false
      }
    }]
  }]
}
```

This format is designed for programmatic processing by LLMs and automated
triage tools.  Use it in CI pipelines that feed results into AI-assisted
review systems.

---

## Fast Mode for CI

CI runners are often slower than local machines.  Use `--fast` to speed up
compilation:

```yaml
- name: Run tests
  run: cargo rvtest --fast --profile ci
```

What `--fast` does:
- Disables debug info (`CARGO_PROFILE_DEV_DEBUG=0`)
- Auto-detects and uses `mold` or `lld` linker

---

## Coverage in CI

```yaml
- name: Run tests with coverage
  run: cargo rvtest --coverage --coverage-min 80

- name: Upload coverage report
  uses: actions/upload-artifact@v4
  with:
    name: coverage-report
    path: target/coverage/
```

The `--coverage-min 80` flag fails the build if line coverage drops below 80%.

---

## Caching Dependencies

Cache the `target/` directory to speed up CI:

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

## Running Tests in Parallel

GitHub Actions supports matrix builds:

```yaml
strategy:
  matrix:
    os: [ubuntu-latest, macos-latest, windows-latest]
    rust: [stable, beta]

steps:
  - uses: actions-rust-lang/setup-rust-toolchain@v1
    with:
      toolchain: ${{ matrix.rust }}

  - name: Run tests
    run: cargo rvtest
```

This runs your tests across multiple OS and Rust versions.

---

## Nightly Builds

For optional nightly features (Cranelift, parallel-frontend):

```yaml
- name: Run tests (nightly)
  if: matrix.rust == 'nightly'
  run: cargo +nightly rvtest --cranelift
```

---

## Flaky Test Detection in CI

Run flaky detection nightly (not on every push — it takes too long):

```yaml
name: Nightly Flaky Detection
on:
  schedule:
    - cron: '0 6 * * *'  # Daily at 6 AM

jobs:
  flaky:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rust-lang/setup-rust-toolchain@v1

      - name: Detect flaky tests
        run: cargo rvtest --detect-flaky
```

---

## GitLab CI

```yaml
test:
  image: rust:latest
  script:
    - cargo rvtest --format junit --coverage --coverage-min 80
  artifacts:
    reports:
      junit: target/junit.xml
      cobertura: target/coverage/cobertura.xml
```

---

## Common CI Mistakes

### Running Tests Without `--nocapture`

CI captures stdout.  If your tests print debug output, it will be hidden.
Use `--show-output` or `-v` to see it in CI logs.

### Ignoring Test Failures

```yaml
# ❌ Bad — always succeeds
- name: Run tests
  run: cargo rvtest || true
```

```yaml
# ✅ Good — fails on test failure
- name: Run tests
  run: cargo rvtest
```

### Not Caching Dependencies

Rebuilding dependencies on every CI run wastes minutes.  Always cache.

### Running Slow Tests on Every Push

Move slow tests (integration, coverage) to a separate workflow that runs
less frequently (nightly, or only on main branch).

---

## Sample Complete CI Configuration

```yaml
name: CI
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rust-lang/setup-rust-toolchain@v1

      - name: Cache
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}

      - name: Run tests
        run: cargo rvtest --fast --profile ci --coverage --coverage-min 80

      - name: Publish results
        uses: dorny/test-reporter@v1
        if: always()
        with:
          name: Rust Tests
          path: results.xml
          reporter: java-junit
```

---

## Summary

- Run `cargo rvtest` in CI as a minimum
- Use `--profile ci` for CI-optimised settings
- Output JUnit XML or GitHub annotations for dashboard integration
- Use `--fast` to speed up CI test compilation
- Run coverage with thresholds to prevent untested code
- Cache dependencies to avoid rebuilding on every run
- Move flaky detection and slow tests to nightly workflows
- Use matrix builds for cross-platform and cross-version testing

In the next chapter, we explore architecture tests — enforcing module
boundaries and dependency rules programmatically.

---

[← Previous](17-legacy-and-refactoring.md) • [Index](00-index.md) • [Next →](19-architecture-tests.md)
