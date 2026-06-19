# rvtest Roadmap

> **Just a library — not a framework, not a product.**

Priorities are organised using the MoSCoW framework (Must have, Should have,
Could have, Won't have).  This is a living document — items move between
quadrants as the project evolves and community feedback arrives.

> **Note:** This document intentionally avoids specific release versions or
> milestone numbers.  What ships when depends on implementation readiness,
> community interest, and whether the timing is right.  Items marked Won't
> Have today may become feasible later; nothing is permanently ruled out.

---

## Current State

All features from the initial roadmap (Must Have, Should Have, Could Have)
have been implemented.  rvtest is a stable, feature-complete testing library
covering BDD specs, property testing, parametrized tests, rich assertions,
mocking, snapshots, architecture tests, code coverage, and a full-featured
CLI runner.

See the [Implementation Summary](#implementation-summary) below for the
complete list.

---

## Implementation Summary

| Feature | Description | Status |
|---|---|---|
| **BDD-style specs** | `describe`/`it`, nesting, hooks, tags, timeouts, retries | ✅ |
| **Property-based testing** | `check`, `Strategy`, `any`, vec/map/filter, shrinking | ✅ |
| **Parametrized tests** | `parametrize`, `parametrize_named` | ✅ |
| **Assertion macros** | `assert_eq!` (diff), `assert_ok!`, `assert_err!`, `assert_matches!`, `assert_delta!` | ✅ |
| **Mocking utilities** | `Spy`, `Stub`, `patch!` | ✅ |
| **Snapshot testing** | File-based assertions, `--update-all`, `--review` | ✅ |
| **Architecture tests** | `may_depend_on`, `may_not_depend_on`, `must_not_have_cycles`, `public_api_doc_required` | ✅ |
| **Output capture** | Per-test stdout/stderr capture, shown on failure | ✅ |
| **Reporting** | Pretty, TAP, JUnit XML, JSON, Compact, GitHub Actions, Agent, HTML, Nextest | ✅ |
| **Code coverage** | Self-contained profraw parser + cargo-llvm-cov + llvm-tools | ✅ |
| **CLI — fast mode** | `--fast` (mold/lld linker, debug=0) | ✅ |
| **CLI — Cranelift** | `--cranelift` (nightly codegen backend) | ✅ |
| **CLI — parallel frontend** | `--parallel-frontend N` (nightly threads) | ✅ |
| **CLI — watch mode** | `--watch` (notify-based file watching) | ✅ |
| **CLI — daemon** | `--daemon` (persistent compile daemon) | ✅ |
| **CLI — flaky detection** | `--detect-flaky` (run N times, report pass rates) | ✅ |
| **CLI — slow profiling** | `--profile-slow` (top N slowest tests) | ✅ |
| **CLI — runner** | Filter, tag, retries, timeout, parallel/sequential, fail-fast | ✅ |
| **Proc-macro API** | `#[describe]` / `#[it]` via optional `macros` feature | ✅ |
| **RunnerConfig builder** | Chainable `with_*()`, presets (`ci()`, `dev()`), `with_config_file()` | ✅ |
| **Auto-retry on failure** | Retry failed tests once without explicit `--retries` | ✅ |
| **`--retest` / `--failed`** | Re-run only previously failed tests | ✅ |
| **Workspace-aware runner** | Auto-detect and run tests across workspace members | ✅ |
| **Test listing** | `--list` to discover tests without running them | ✅ |
| **Env utilities** | `rvtest::env` module — RAII guards for env vars (`set_var`, `remove_var`, `set_vars`) | ✅ |
| **FS utilities** | `rvtest::fs` module — `temp_dir()`, `temp_dir_with_prefix()` | ✅ |
| **Test shuffling** | `--shuffle` to randomise execution order | ✅ |
| **`--skip`** | Skip tests by name or pattern | ✅ |
| **`--color`** | `always`/`auto`/`never` colour control | ✅ |
| **Enhanced JSON & JUnit** | Include duration, output, source location in all formats | ✅ |
| **Last-run cache** | Persist results, diff against previous run | ✅ |
| **Enhanced pretty reporter** | Collapsed passing groups, cleaner output | ✅ |
| **Agent-native output** | `--format agent` for LLM-optimised test results | ✅ |
| **Smart `--changed`** | Git-aware test selection based on changed files | ✅ |
| **Flaky quarantine** | `--quarantine`, `--flaky-report`, `--include-flaky`, `--unquarantine` | ✅ |
| **Benchmark regression** | `#[bench]` inside describe with duration alerts | ✅ |
| **Baseline comparison** | Compare durations against saved baseline | ✅ |
| **HTML report** | Standalone report with results + coverage | ✅ |
| **Test gap analysis** | Coverage + descriptions to find untested paths | ✅ |
| **Open-test-report** | Launch report in browser after a run | ✅ |
| **Cargo nextest integration** | Nextest-compatible output | ✅ |

---

# Next

Items being evaluated for future iterations, based on ecosystem research and
developer needs assessment.  Priorities will shift as community feedback
arrives.  Items are not committed — they are candidates.

Each item has a unique ID for tracking and discussion.

---

## 🔒 Security Improvements

| ID | # | Feature | Expected Value | Effort Estimate | Rationale |
|----|---|---------|---------------|-----------------|-----------|
| SEC-001 | 1 | **Process-per-test isolation** | High | Very High | rvtest currently uses thread-based execution. Process isolation (like nextest) prevents test A from corrupting the state of test B, and provides clean recovery from segfaults or stack overflows in native code. Requires major runner refactoring. |
| SEC-002 | 2 | **Secrets masking in test output** | High | Low | Automatically detect and redact common secret patterns (API keys, tokens, passwords, AWS keys) in captured test output before displaying or persisting. Prevents accidental secret leakage in CI logs and snapshot files. Configurable pattern list. |
| SEC-003 | 3 | **Test execution sandboxing** | Medium | High | Optional sandbox that restricts test capabilities: filesystem access (whitelist paths), network access (allow/deny rules), environment variable access. Especially valuable for CI environments and for testing untrusted code or parser fuzzers. |
| SEC-004 | 4 | **Resource limits & DoS prevention** | Medium | Medium | Per-test limits on: heap allocation, file descriptor count, thread count, execution time (already partially supported via `--timeout`). Prevents runaway tests from exhausting system resources and affecting other tests or the host. |
| SEC-005 | 5 | **Supply chain verification for test dependencies** | Medium | Medium | When running `cargo rvtest --coverage` or using snapshot files, verify integrity of generated artifacts. Checksum tracking for `.snap` files, coverage data, and cached test results to detect tampering or corruption. |
| SEC-006 | 6 | **Permission-aware test execution** | Low | High | Declarative permission model: tests declare what resources they need (network, filesystem, env vars). Runner enforces permissions at runtime. Inspired by Android's permission model and WASI's capability-based security. |

---

## ⚡ Performance Improvements

One of the most common developer complaints about Rust testing is that it
feels **heavy and slow** — slow compilation, no test result caching, no
dependency-aware test selection, and unnecessary rebuilds.  rvtest already
has several features addressing this (`--fast`, `--daemon`, `--changed`,
`--retest`, `--cranelift`, `--parallel-frontend`), but there is significant
room for improvement.

Items are ordered by impact, highest first.

| ID | # | Feature | Expected Value | Effort Estimate | Rationale |
|----|---|---------|---------------|-----------------|-----------|
| PERF-001 | 7 | **Source-level impact analysis** (`--impact`) | Highest | High | Map source files to affected tests via module/import graph. `--changed` currently does naive name matching from `git diff`. This would parse `use` statements or leverage `cargo metadata` to determine *exactly* which tests must re-run given a set of changed files. Dramatically reduces unnecessary test execution in CI and local dev. |
| PERF-005 | 8 | **Test result caching** | Very High | Medium | Hash source files + test binary → persist pass/fail status. Tests that passed and whose dependencies haven't changed are skipped on subsequent runs. `--verify-cache` for periodic re-validation. Conceptually similar to `cargo-nextest`'s test result caching but integrated directly into rvtest's runner. |
| PERF-002 | 9 | **Test binary build cache** | Very High | Medium | Content-hash test source files and dependencies. Skip `cargo build --tests` entirely when hashes match the previous build. Much faster than waiting for `cargo`'s own incremental compilation to decide nothing changed. Cache stored in `target/.rvtest-build-cache/`. |
| PERF-006 | 10 | **Parallel execution within a single binary** | High | Medium | `cargo test` runs tests sequentially within a binary. rvtest can orchestrate parallel test execution by running test binaries with `--test-threads` control or by splitting test discovery from execution. Brings nextest-style parallelism to rvtest's `--daemon` and direct-run modes. |
| PERF-003 | 11 | **Smart `--fast` defaults** | High | Low-Medium | Auto-detect hardware: RAM, CPU count, SSD vs HDD. Auto-select linker (mold > lld > default). Auto-set `CARGO_INCREMENTAL=1` for dev. Auto-configure optimal `--max-threads`. Suggest ramdisk `--target-dir` when RAM > 16 GB. Eliminates the need for users to discover and configure these flags manually. |
| PERF-008 | 12 | **Warm daemon auto-start** | Medium | Low | `--daemon` already exists but requires explicit opt-in. Background daemon that auto-starts on first `cargo rvtest` and keeps the test binary warm in memory. Subsequent runs skip compilation entirely if sources haven't changed. Integrates with shell's background job control or a local socket. |
| PERF-004 | 13 | **Parallel integration test compilation** | Medium | Medium | Integration test files in `tests/` are independent binaries. Compile them in parallel instead of sequentially. Currently `cargo test` compiles each integration test binary one at a time; rvtest can orchestrate parallel `cargo build --test <name>` invocations. |
| PERF-007 | 14 | **Pre-emptive test ordering** | Medium | Low | Run previously-failed tests first (for `--fail-fast`), slow tests last. Maximises feedback velocity — if something is going to fail, you want to know as early as possible. |
| PERF-009 | 15 | **`cargo rvtest --why-slow`** | Medium | Medium | Profiling command that breaks down total test time into: compilation time, linking time, test execution time, slowest 10 tests, and bottleneck crates. Helps users identify where their testing pipeline is spending time. |

---

## 🚀 Feature Improvements

| ID | # | Feature | Expected Value | Effort Estimate | Rationale |
|----|---|---------|---------------|-----------------|-----------|
| MOCK-001 | 16 | **Trait Mocking** (`#[automock]`) | Highest | High | Most requested testing feature in Rust. Auto-generate mock structs implementing user traits with expectation API (`.expect_foo().with(eq(x)).times(1).returning(...)`). Currently users must hand-write mock structs. |
| ASYNC-001 | 17 | **Async Test Support** | Very High | Medium | `async_it()`, `run_async()`, async lifecycle hooks (`before_all_async`, `after_each_async`). Most modern Rust projects use async runtimes (tokio). Currently `#[tokio::test]` bypasses rvtest entirely. Feature-gated behind `features = ["tokio"]`. |
| TIME-001 | 18 | **Time / Clock Mocking** | High | Medium | RAII-guarded `mock_clock()`, `freeze()`, `advance()`. Every project testing retries, timeouts, rate-limiters, or polling reimplements this manually. A `Clock` trait with test-only mock implementation. |
| MATCH-001 | 19 | **Composable Matchers** (`assert_that!`) | High | Medium-High | Expressive assertion framework: `assert_that!(value, gt(5).and(lt(10)))`, `assert_that!(result, ok(contains("expected")))`. Plus `expect_that!` (non-fatal — collects all failures, continues test) and `verify_that!` (Result-based). |
| SNAP-001 | 20 | **Inline Snapshots** | High | High | Snapshots stored as string literals in source code, not separate `.snap` files. CI-friendly — changes appear in the diff. `assert_inline_snapshot!("...")`. Requires proc-macro or source rewriting. |
| PROP-001 | 21 | **Property Testing Depth** | Highest | Very High | Close gaps vs proptest: `prop_assume!` for preconditions, `#[proptest]` macro, regex-based string generation, failure persistence to files, richer combinator library (`oneof`, `union`, recursion, tuple strategies), per-value structural shrinking. |
| FIXT-001 | 22 | **Typed Test Fixtures** (`#[fixture]`) | High | Medium-High | Strongly-typed fixture injection: `describe("x").with_fixture(db).it("test", \|conn\| { ... })`. Cleaner alternative to manual `Arc<Mutex<T>>` plumbing. Inspired by `rstest`. |
| SFILT-001 | 23 | **Snapshot Filters** | High | Medium | Mask dynamic content (timestamps, UUIDs, random values) before snapshot comparison. Enables snapshot testing of otherwise-unstable output. |
| CASE-001 | 24 | **Parametrized Test Macro** (`#[test_case]`) | Medium | Medium | Each case becomes an independent `#[test]` with its own pass/fail in `cargo test` output. `#[test_matrix]` for Cartesian product of inputs. |
| FAKE-001 | 25 | **Fake Data Generation** (`#[derive(Fake)]`) | Medium-High | Medium | Auto-generated fake data for test fixtures: names, addresses, emails, lorem, etc. Locale-aware. Reduces test data boilerplate. |
| TFILE-001 | 26 | **`NamedTempFile`** | Low | Low | Complement to `TempDir`. A file that is deleted on drop but has a known, stable path. |
| SDIFF-001 | 27 | **Side-by-Side Diff in Assertions** | Low | Low | Cosmetic improvement to `assert_eq!` failure output. |
| CONF-001 | 28 | **Config file `rvtest.toml`** | High | Medium | Persistent project-level configuration so users don't need to type flags every time. Define profiles, default formats, filter/skip patterns, coverage settings, and performance tuning options. Merged via `RunnerConfig::with_config_file()`. |
| FUZZ-001 | 29 | **Fuzzing Integration** | Medium | Very High | Coverage-guided fuzzing via `cargo-fuzz`/`libfuzzer` as a backend. Unified property-testing + fuzzing workflow. |
| MATRIX-001 | 30 | **Test Matrix / Multi-Version** | Medium | Medium | `cargo rvtest --matrix stable,nightly` as a convenience layer on top of CI matrix strategies. |
| TUI-001 | 31 | **TUI Mode** | Low | High | Interactive terminal UI for browsing test results, filtering, and re-running. High maintenance, niche use case. |
| HTTP-001 | 32 | **Network / HTTP Mocking** | Medium | Very High | Built-in HTTP mocking layer. Broad domain with existing solutions (wiremock, httpmock, mockito). Requires a clear, scoped design that adds unique value beyond existing crates. |
| PLUGIN-001 | 33 | **Custom Reporter Plugins** | Low | Very High | Stable plugin ABI or dynamic loading for third-party reporters. Not practical without ecosystem-level plugin support (WASM plugins, stable ABI). |

### Documentation

| ID | Chapter | Issue |
|----|---------|-------|
| DBENCH-001 | **16 — Benchmark** | ❌ Critical. `spec.bench()`, `spec.bench_iterations()`, and `spec.bench_threshold()` are all implemented but completely undocumented. Chapter only shows manual `Instant::now()` approach. |
| DSETUP-001 | **09 — Setup/Teardown** | ⚠️ Minor. `rvtest::env::set_vars()` (multi-variable RAII guard) not mentioned. |
| DSNAP-001 | **13 — Snapshot** | ⚠️ Minor. `assert_snapshot_in()` for custom directory not mentioned. `TempDir::leak()` not mentioned. |
| DCLI-001 | **CLI flags** | ⚠️ Minor. `--format nextest`, `--format html`, `--report-html`, `--bench`, `--save-baseline`, `--compare-baseline`, `--gap-analysis`, `--open-report` not fully documented in learn chapters. |

---

## Not Yet Prioritised

Items that have been considered but are not currently planned.  They may
become candidates if the timing is right — community demand, contributor
interest, or ecosystem shifts can move these up.

| Item | Category | Why Not Yet | What Would Change This |
|------|----------|-------------|-----------------------|
| **Custom reporter plugins** | Feature | Would require a stable plugin ABI or dynamic loading. Not practical for a dev-dependency library. | A stable plugin system emerges in the Rust ecosystem (e.g., WASM plugins, stable ABI). |
| **TUI mode** | Feature | Interactive terminal UI. High maintenance, niche use case. | A clear design and maintainer interest. Could be explored as a separate crate. |
| **Network/HTTP mocking** | Feature | Broad domain with many existing solutions (wiremock, httpmock, mockito). rvtest should focus on core testing library concerns. | A clear, scoped design that adds unique value beyond existing crates. |

---

## Non-Goals

- **Replacing Cargo's test harness entirely.**  rvtest complements
  `#[test]`, it does not replace it.  Users adopt features incrementally.
- **Runtime reflection or code generation.**  The proc-macro API uses
  standard Rust macros, not build scripts or compiler plugins.
- **Stable-only lock-in.**  rvtest targets stable Rust.  Features that
  require nightly (Cranelift, parallel-frontend) are optional and gated
  behind runtime detection.
- **Competing with dedicated tools.**  rvtest is a library, not a product.
  Where the ecosystem already has mature solutions (mockall, proptest,
  insta), rvtest may defer to them rather than duplicate effort — unless
  integration provides clear added value.
