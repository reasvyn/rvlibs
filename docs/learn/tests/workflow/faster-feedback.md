# Faster Feedback

Speeding up the test cycle — fast mode, watch mode, daemon, and smart test selection.

## Prerequisites

- [CI Integration](ci-integration.md) — CI pipelines

## Glossarium

| Term | Definition |
|------|------------|
| `--fast` | Use fast linker (mold/lld) and disable debug info for faster compilation. |
| `--watch` | Re-run tests automatically when files change. |
| `--daemon` | Persistent compile daemon for sub-second iteration. |
| `--changed` | Auto-select tests based on `git diff` output. |
| `--retest` | Re-run only the tests that failed in the previous run. |

## Fast Compilation

```bash
# Fast linker + minimal debug info
cargo rvtest --fast

# Cranelift codegen backend (nightly only)
cargo rvtest --cranelift

# Parallel front-end threads (nightly only)
cargo rvtest --parallel-frontend 8

# Combine them
cargo rvtest --fast --cranelift --parallel-frontend 8
```

## Watch Mode

```bash
# Re-run tests on every file change
cargo rvtest --watch
```

Uses `notify` internally to watch for filesystem changes. Only tests in changed crates are re-run when possible.

## Daemon Mode

```bash
# Start the daemon
cargo rvtest --daemon
```

The daemon keeps the test binary compiled and warm in memory. Subsequent runs skip compilation entirely:

```
[daemon] binary is up-to-date (0.02s)
running 42 tests ... ok
```

## Smart Test Selection

```bash
# Run only tests affected by changed files (git-aware)
cargo rvtest --changed

# Re-run only previously failed tests
cargo rvtest --retest

# Re-run only failed tests (alias)
cargo rvtest --failed
```

## Comparing Strategies

| Strategy | Speed Gain | When to Use |
|----------|-----------|-------------|
| `--fast` | 2–5x compile | Every local run |
| `--changed` | 10–100x | After editing a few files |
| `--retest` | 100x+ | After a failing run |
| `--daemon` | 50–500x | Repeated iterations |
| `--watch` | Continuous | TDD workflow |

## Run Only What Matters

```bash
# By name
cargo rvtest -f arithmetic

# By tag
cargo rvtest --tag smoke

# Skip slow tests
cargo rvtest --skip slow --exclude-tag slow

# Stop on first failure
cargo rvtest --fail-fast
```

## Next Steps

- [CI Integration](ci-integration.md) — running tests in CI pipelines
- [Rust Modular System](../../rust/project-structure/modules-and-packages.md) — organising code with modules
