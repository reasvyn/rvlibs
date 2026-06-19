<div align="center">
  <h1>rvtest</h1>
  <p><strong>A Next Level Testing Library for Rust</strong></p>
  <p><em>Just a library — not a framework, not a product.</em></p>
</div>

<div align="center">

[![Crates.io][crates-badge]][crates-url]
[![Crates.io (cargo-rvtest)][cli-crates-badge]][cli-crates-url]
[![GitHub][repo-badge]][repo-url]
[![MIT License][license-badge]][license-url]
[![Rust 1.96+][rust-badge]][rust-url]

</div>

[crates-badge]: https://img.shields.io/crates/v/rvtest.svg
[crates-url]: https://crates.io/crates/rvtest
[cli-crates-badge]: https://img.shields.io/crates/v/cargo-rvtest.svg
[cli-crates-url]: https://crates.io/crates/cargo-rvtest
[repo-badge]: https://img.shields.io/badge/github-reasvyn/rvtest-8da0cb?logo=github
[repo-url]: https://github.com/reasvyn/rvtest
[license-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[license-url]: https://github.com/reasvyn/rvtest/blob/main/LICENSE
[rust-badge]: https://img.shields.io/badge/rust-1.96%2B-blue?logo=rust
[rust-url]: https://www.lan

---

`rvtest` is a Rust **library** that extends the built-in test harness with a rich suite of
features designed for real-world testing workflows — no proc-macro
magic or heavy dependencies required.

It is **not** a test framework. It is **not** a product. It does not replace
`#[test]` or `cargo test`. It is a library you pull in as a dev-dependency
and use exactly where and when you need it — nothing more.

## Features

- **BDD-style specs** – Organise tests with `describe`/`it` blocks,
  nested hierarchies, hooks (`before_all`, `after_all`, `before_each`,
  `after_each`), tags, per-suite timeouts, and retries.
- **Property-based testing** – Verify invariants over thousands of
  randomly generated inputs with automatic counterexample shrinking.
- **Parametrized tests** – Run the same test logic against multiple
  inputs without boilerplate.
- **Assertion macros** – `assert_eq!` with structural diffs,
  `assert_ok!`, `assert_err!`, `assert_matches!`, `assert_delta!`.
- **Mocking utilities** – `Spy` (call-recording), `Stub` (fixed return),
  `patch!` (scoped function replacement). No proc-macro required.
- **Rich reporting** – Pretty (colourised), TAP, JUnit XML, JSON,
  Compact, GitHub Actions annotations, and **Agent-native** (LLM-optimised)
  formats.
- **Self-contained code coverage** – Measure line, function, and
  region coverage via `cargo rvtest --coverage`. Pure-Rust `.profraw`
  parser — no `llvm-profdata` or `llvm-cov` required.
- **Snapshot testing** – File-based snapshot assertions with
  `--update-all` auto-accept and `--review` interactive mode.
- **Architecture tests** – Enforce module dependencies and layering
  rules (`may_depend_on`, `may_not_depend_on`, `must_not_have_cycles`,
  `public_api_doc_required`).
- **Output capture** – Per-test stdout/stderr capture shown only on
  failure. Controlled via `--show-output`.
- **Configurable runner** – Parallel execution, name & tag filtering,
  skip patterns, fail-fast, configurable timeouts, retries, and
  execution-order shuffling.
- **Watch & daemon modes** – `--watch` re-runs `cargo test` on file
  changes; `--daemon` keeps a persistent compile daemon for sub-second
  iteration.
- **Flaky test detection** – `--detect-flaky` runs the suite multiple
  times and reports pass-rate per test.
- **Flaky quarantine** – `--quarantine` skips previously-detected flaky
  tests; `--flaky-report` lists them; `--unquarantine` clears the list.
- **Slow test profiling** – `--profile-slow` surfaces the slowest
  tests.
- **Test diff** – `--diff` compares results against the previous run,
  highlighting new failures, recovered tests, and duration changes.
- **Git-aware selection** – `--changed` auto-filters tests based on
  `git diff` output.
- **Last-run cache** – Persists full results for `--retest`, `--failed`,
  and `--diff`.
- **Env & FS utilities** – RAII-guarded `set_var`/`remove_var`/`set_vars`
  and auto-cleaning `temp_dir()`/`temp_dir_with_prefix()`.
- **Optional proc-macro API** – `#[describe]` / `#[it]` attribute
  macros via the `macros` feature.

---

## Project Structure

`rvtest` is split into two crates:

| Crate | Description | Installation |
|---|---|---|
| **`rvtest`** | Library — BDD specs, assertions, mocks, property tests, etc. | `cargo add --dev rvtest` |
| **`cargo-rvtest`** | CLI binary — `cargo rvtest` command with reporting, coverage, watch mode | `cargo install cargo-rvtest` |

## Quick Start

### Library — write tests with `describe`/`it`

Add `rvtest` to your `Cargo.toml`:

```toml
[dev-dependencies]
rvtest = "0.3.0"
```

### BDD-style specs

```rust
use rvtest::spec::describe;

#[test]
fn calculator_tests() {
    describe("Calculator")
        .it("adds two positive numbers", || {
            assert_eq!(2 + 2, 4);
        })
        .it("subtracts", || {
            assert_eq!(5 - 3, 2);
        })
        .tag("arithmetic")
        .timeout(std::time::Duration::from_secs(2))
        .run()
        .assert_all_pass();
}
```

Nested suites, lifecycle hooks, and retries are fully supported:

```rust
#[test]
fn database_tests() {
    describe("Database")
        .before_all(|| {
            // runs once before any child test
        })
        .after_all(|| {
            // runs once after all child tests
        })
        .describe("queries")
            .it("selects user by id", || { /* ... */ })
            .it("inserts new record", || { /* ... */ })
            .tag("smoke")
        .describe("transactions")
            .it("rolls back on error", || { /* ... */ })
            .retries(2)   // flaky test — retry twice
        .run()
        .assert_all_pass();
}
```

### Property-based testing

```rust
use rvtest::property::{check, any};

#[test]
fn addition_is_commutative() {
    check("commutativity", any::<i32>(), |a: &i32| {
        let b: i32 = 42;
        a + b == b + *a
    });
}
```

When a counter-example is found, `check` panics with the seed
and the (shrunk) minimal failing input.

### Parametrized tests

```rust
use rvtest::param::parametrize;

#[test]
fn addition_cases() {
    for case in parametrize("add", [(1, 1, 2), (0, 0, 0), (-1, 1, 0)], |(a, b, exp)| {
        assert_eq!(a + b, *exp);
    }) {
        assert!(case.status.is_passed(), "{} failed", case.name);
    }
}
```

---

## CLI Usage (`cargo rvtest`)

`rvtest` ships with a `cargo` subcommand in the `cargo-rvtest` crate.
Install it:

```bash
cargo install cargo-rvtest
```

Then run your project's tests with rvtest's reporting:

```bash
# Run all tests with the pretty reporter (default)
cargo rvtest

# Run only tests matching a name filter
cargo rvtest --filter arithmetic

# Skip tests whose name contains a pattern
cargo rvtest --skip slow

# Run with verbose output (show passing tests too)
cargo rvtest -v

# Output in machine-readable formats
cargo rvtest -F json
cargo rvtest -F tap
cargo rvtest -F junit
cargo rvtest -F compact
cargo rvtest -F agent      # LLM-optimised JSON output

# Collect code coverage (pure-Rust, no external tools needed)
cargo rvtest --coverage

# Coverage with different output formats
cargo rvtest --coverage --coverage-format json --coverage-dir ./coverage

# Tag-based filtering
cargo rvtest --tag smoke
cargo rvtest --tag arithmetic --exclude-tag slow

# Fail-fast mode
cargo rvtest --fail-fast

# Randomise test execution order
cargo rvtest --shuffle

# Compare against previous run (show diff)
cargo rvtest --diff

# Auto-detect tests for changed files (git-aware)
cargo rvtest --changed

# Automatic retry of failed tests
cargo rvtest --auto-retry

# Re-run only previously failed tests
cargo rvtest --retest

# List tests without running them
cargo rvtest --list

# Detect flaky tests (default 10 runs)
cargo rvtest --detect-flaky

# Skip known-flaky tests (quarantine)
cargo rvtest --quarantine

# List quarantined tests
cargo rvtest --flaky-report

# Clear quarantine list
cargo rvtest --unquarantine

# Colour control
cargo rvtest --color always
cargo rvtest --color never

# Profile-based configuration
cargo rvtest --profile ci      # JUnit output, fail-fast
cargo rvtest --profile dev     # Pretty output, verbose

# Run across all workspace members
cargo rvtest --workspace
```

All options:

| Flag | Description |
|---|---|
| `-f, --filter` | Filter test names by substring (case-insensitive) |
| `--skip` | Skip test names containing this string (case-insensitive) |
| `-t, --tag` | Only run tests carrying all of these tags |
| `-E, --exclude-tag` | Skip tests carrying any of these tags |
| `-r, --retries` | Number of retries for flaky tests (default: 0) |
| `--auto-retry` | Automatically retry failed tests once |
| `--timeout` | Default per-test timeout in seconds |
| `--shuffle` | Randomise test execution order |
| `--seed` | Seed for randomised features (shuffle, property tests) |
| `--no-parallel` | Run tests sequentially |
| `--max-threads` | Maximum number of threads for parallel execution |
| `--fail-fast` | Stop after the first failure |
| `-F, --format` | Output format: `pretty`, `tap`, `junit`, `json`, `compact`, `github`, `agent`, `html`, `nextest` |
| `--color` | Colour output: `auto`, `always`, `never` |
| `-v, --verbose` | Show all tests (including passing ones) |
| `--show-output` | Show captured stdout/stderr |
| `--profile` | Config profile: `ci`, `dev` |
| `--fast` | Fast linker (mold/lld) + disable debug info |
| `--cranelift` | Use Cranelift codegen backend (nightly) |
| `--parallel-frontend N` | Parallel front-end threads (nightly) |
| `--workspace` | Run tests across all workspace members |
| `--watch` | Re-run tests on file changes |
| `--daemon` | Persistent compile daemon for sub-second iteration |
| `--detect-flaky[=N]` | Detect flaky tests (default 10 runs) |
| `--quarantine` | Skip previously detected flaky tests |
| `--include-flaky` | Run quarantined tests anyway |
| `--flaky-report` | Show currently quarantined tests |
| `--unquarantine` | Clear the quarantine list |
| `--profile-slow[=N]` | Show N slowest tests (default 5) |
| `--list` | List all discovered tests without running |
| `--retest` / `--failed` | Re-run only previously failed tests |
| `--diff` | Compare results against the previous run |
| `--changed` | Auto-select tests based on `git diff` |
| `--update-all` | Auto-accept all snapshot changes |
| `--review` | Interactive snapshot review mode |
| `--coverage` | Enable code coverage collection |
| `--coverage-format` | Coverage format: `summary`, `html`, `lcov`, `json`, `cobertura` |
| `--coverage-dir` | Output directory for coverage artifacts |
| `--coverage-min` | Minimum line-coverage percentage (fails if below) |
| `--coverage-open` | Open coverage report in browser |

---

## Reporting Formats

Nine output formats are supported:

| Format | Description |
|---|---|
| **Pretty** (default) | Human-readable, colourised output with ✓/✗/– badges and timing |
| **TAP** | [Test Anything Protocol](https://testanything.org/) — line-based format widely supported by CI |
| **JUnit XML** | XML format understood by Jenkins, GitLab CI, GitHub Actions |
| **JSON** | Structured JSON output for programmatic consumption |
| **Compact** | Single-line-per-test summary for quick feedback |
| **GitHub** | GitHub Actions `::error` annotations for inline PR display |
| **Agent** | LLM-optimised JSON with source snippets, failure analysis, and machine metadata |
| **HTML** | Standalone dark-themed HTML report |
| **Nextest** | Cargo-nextest compatible JSON-lines output |

---

## Code Coverage

`rvtest` includes a **self-contained coverage system** that works
without any external tools. It compiles your tests with LLVM
coverage instrumentation (`-Cinstrument-coverage`) and parses the
resulting `.profraw` files entirely in Rust — no `llvm-profdata`,
`llvm-cov`, or `cargo-llvm-cov` required.

```
$ cargo rvtest --coverage
Coverage: 48.7% lines, 56.1% functions, 48.7% regions
```

The coverage output is 100 % compatible with the format produced
by `llvm-cov report --summary-only`, so it can be used with any
tooling that understands LLVM coverage data.

If `cargo-llvm-cov` or `llvm-profdata`/`llvm-cov` are installed,
`rvtest` uses them automatically for enhanced report generation.

---

## Learn Testing in Rust

A complete step-by-step guide to testing Rust code — from absolute basics to
real-world workflows — is available in the [learn](docs/learn/00-index.md)
directory.  21 chapters covering everything from `#[test]` to architecture
enforcement and concurrent code testing.

---

## How It Works

`rvtest` is designed as a **library** that you use inside standard
`#[test]` functions. The `describe`/`it` builder constructs a test
spec, and `run()` executes it — catching panics, measuring timing,
and recording results. `assert_all_pass()` panics with a detailed
report if any test failed, which causes the `#[test]` to fail
naturally.

The `cargo rvtest` CLI (from the separate `cargo-rvtest` crate) runs
your project's tests via `cargo test`, parses the output, and re-renders
it using rvtest's reporting system. This gives you all the format
flexibility without requiring any changes to your test code.

---

## Roadmap / Future Ideas

See the full [roadmap](docs/roadmap.md) for details. Highlights:

- **Config file** — `rvtest.toml` with persistent defaults and profiles.
- **Benchmark regression** — detect duration changes over time.
- **HTML report** — standalone report with results + coverage.
- **Test gap analysis** — coverage-driven untested path detection.
- **Baseline comparison** — compare durations against saved baseline.

---

## License

`rvtest` is released under the MIT License.
