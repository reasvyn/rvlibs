# rvtest Architecture

> Internal architecture of the rvtest testing library.

---

## High-Level Overview

```
┌────────────────────────────────────────────────────────────────┐
│                        cargo rvtest (CLI)                      │
│  ┌─────────┐  ┌──────────┐  ┌───────────┐  ┌────────────────┐ │
│  │  Parser  │  │  Runner  │  │ Coverage  │  │   Reporters    │ │
│  │ (clap)   │  │ (cargo   │  │Collector  │  │ Pretty / TAP   │ │
│  │          │  │  test)   │  │ (profraw) │  │ JUnit / JSON   │ │
│  └─────────┘  └──────────┘  └───────────┘  │ / Compact      │ │
│                                             └────────────────┘ │
└────────────────────────────────────────────────────────────────┘
                           │ uses
                           ▼
┌────────────────────────────────────────────────────────────────┐
│                     rvtest (library crate)                      │
│                                                                 │
│  ┌──────────┐  ┌──────────┐  ┌───────────┐  ┌───────────────┐ │
│  │  spec    │  │  property│  │   param   │  │    tag        │ │
│  │describe/ │  │ Strategy │  │parametrize│  │ tag/name      │ │
│  │ it/run   │  │ check    │  │           │  │ filtering     │ │
│  └────┬─────┘  └────┬─────┘  └─────┬─────┘  └───────┬───────┘ │
│       │             │              │                 │         │
│       └─────────────┴──────────────┴─────────────────┘         │
│                               │ produces                       │
│                               ▼                                │
│  ┌──────────┐  ┌──────────┐  ┌───────────┐  ┌───────────────┐ │
│  │  core    │  │  runner  │  │  report   │  │   coverage    │ │
│  │TestSuite │  │TestRunner│  │TestReporter│  │ Collector +   │ │
│  │TestCase  │  │ run_tests│  │ Pretty/    │  │ RawParser     │ │
│  │TestStatus│  │ daemon   │  │ Json/...   │  │               │ │
│  └──────────┘  └──────────┘  └───────────┘  └───────────────┘ │
│  ┌──────────┐  ┌──────────┐  ┌───────────┐  ┌───────────────┐ │
│  │  arch    │  │  assert  │  │  capture  │  │  daemon       │ │
│  │ arch_check│  │ assert_eq│  │ output    │  │ CompileDaemon │ │
│  │ rules    │  │ ok/err/  │  │ capture   │  │ build + watch │ │
│  │          │  │ matches/ │  │           │  │ direct exec   │ │
│  │          │  │ delta    │  │           │  │               │ │
│  └──────────┘  └──────────┘  └───────────┘  └───────────────┘ │
│  ┌──────────┐  ┌──────────┐                                      │
│  │  mock    │  │snapshot  │                                      │
│  │ Spy/Stub │  │assert_   │                                      │
│  │ patch!   │  │snapshot  │                                      │
│  └──────────┘  └──────────┘                                      │
└────────────────────────────────────────────────────────────────┘
```

---

## Core Data Flow

```
User code                         rvtest library
──────────                        ──────────────

describe("Math")                  Spec { name, children, tests }
  .it("adds", || ...)            ──► push TestEntry { name, fn }
  .it("subs", || ...)
  .run()                          collect_tests()
      │                               │
      │                           execute_test()
      │                               │
      │                           catch_unwind(test_fn)
      │                               │
      │                           TestCase { status, duration }
      │                               │
      ▼                               ▼
  TestSuite { tests }             TestSuite
  .assert_all_pass()              panic! on any failure
```

```
CLI flow
────────

cargo rvtest
    │
    ├─►── cargo test (subprocess)
    │       │
    │       ▼
    │   parse_cargo_test_output()
    │       │
    │       ▼
    │   TestRun { suites, duration }
    │       │
    │       ▼
    │   reporter.report(&run)
    │       │
    │       ▼
    │   stdout
    │
    └─►── --coverage
            │
            ├─► cargo test --no-run (with -Cinstrument-coverage)
            │       │
            │       ▼
            │   run test binaries
            │       │
            │       ▼
            │   collect .profraw files
            │       │
            │       ▼
            │   parse_raw_profile()
            │       │
            │       ▼
            │   CoverageReport
            │
            └─► (or: cargo-llvm-cov / llvm-tools)
```

---

## Key Types

### `TestRun` (aggregate root)

```rust
pub struct TestRun {
    pub suites: Vec<TestSuite>,
    pub start_time: SystemTime,
    pub end_time: SystemTime,
    pub duration: Duration,
}
```

### `TestSuite` (one describe block or binary output)

```rust
pub struct TestSuite {
    pub name: String,
    pub description: Option<String>,
    pub tests: Vec<TestCase>,
    pub duration: Duration,
    pub kind: TestKind,          // Unit | Integration | Doc
    pub source_path: String,     // e.g. "src/lib.rs"
}
```

### `TestCase` (single test execution result)

```rust
pub struct TestCase {
    pub name: String,
    pub suite: Option<String>,
    pub tags: Vec<String>,
    pub status: TestStatus,
    pub duration: Duration,
    pub assertions: u64,
    pub location: Option<SourceLocation>,
    pub parameters: Vec<(String, String)>,
    pub captured_output: Option<String>,
}
```

### `TestStatus` (outcome of one test)

```rust
pub enum TestStatus {
    Passed,
    Failed { reason: String, location: Option<SourceLocation> },
    Skipped { reason: Option<String> },
    TimedOut { duration: Duration, location: Option<SourceLocation> },
}
```

---

## Module Dependencies

```
lib.rs
  ├── core.rs           (no deps)
  ├── tag.rs            (depends on: core)
  ├── spec.rs           (depends on: core, tag)
  ├── property.rs       (depends on: rand)
  ├── param.rs          (depends on: core)
  ├── assert.rs         (depends on: core, similar)
  ├── mock.rs           (depends on: core)
  ├── arch.rs           (no deps)
  ├── snapshot.rs       (no deps)
  ├── capture.rs        (depends on: libc)
  ├── report.rs         (depends on: core)
  ├── runner.rs         (depends on: core, report, spec)
  ├── daemon.rs         (depends on: core, report, runner, notify, serde)
  ├── coverage.rs       (depends on: core, coverage_raw)
  ├── coverage_raw.rs   (depends on: serde, core)
  └── main.rs           (depends on: core, coverage, report, runner, daemon, clap)
```

No circular dependencies. Each module depends only on `core` and
possibly sibling modules.

---

## CLI Architecture

The `cargo-rvtest` binary (`src/main.rs`) is the entry point:

1. Parse args with `clap`
2. Mode dispatch:
   - `--coverage` / `--coverage-open` → `CoverageCollector`
   - `--watch` → `watch_loop()` (cargo test on file changes)
   - `--daemon` → `CompileDaemon::run()` (build once, direct binary exec)
   - `--detect-flaky N` → run suite N times, report pass rates
   - Default → `run_cargo_test()` → parse → render
3. Fast mode flags (`--fast`, `--cranelift`, `--parallel-frontend`)
   modify the build command via `RUSTFLAGS` and env vars
4. Output rendered via selected `TestReporter` (Pretty, TAP, JUnit,
   JSON, Compact, GitHub)
5. Exit 0 on success, 1 on failure

The spinner is an optional cosmetic thread that runs only when
stdout is a terminal.

---

## Coverage Architecture

```
CoverageCollector::collect()
    │
    ├── has_cargo_llvm_cov()?        ──►  run_via_cargo_llvm_cov()
    ├── has_llvm_tools()?            ──►  run_via_llvm_tools()
    ├── self_contained_profraw()?    ──►  run_via_raw_parser()
    └── (fallback, Linux only)       ──►  run_via_sampler(ptrace+addr2line)
```

The self-contained parser (`coverage_raw.rs`) implements the raw
profile format directly:

```
RawProfile {
    Magic:     0xff6c70726f667281
    Version:   10
    Header:    16 × u64 = 128 bytes
    BinaryIds: variable
    Data:      [ProfileData × NumData] (64 bytes each)
    Counters:  [u64 × NumCounters]
    Names:     [u8 × NamesSize]
}
```

LLVM 22+ raw profile format.  Coverage is computed as the ratio
of non-zero counters to total counters per function and overall.
