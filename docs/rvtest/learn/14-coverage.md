# Chapter 14 — Code Coverage

[← Previous](13-snapshot-testing.md) • [Index](00-index.md) • [Next →](15-flaky-tests.md)

---

Code coverage measures which lines, functions, and branches of your code are
executed during testing.  It tells you what you have tested and — more
importantly — what you have not.  This chapter explains how to measure
coverage with `rvtest`, interpret the results, and use them to improve your
test suite.

---

## What Coverage Measures

Coverage tools report several metrics:

| Metric | What It Measures | Interpretation |
|--------|-----------------|----------------|
| **Line coverage** | Percentage of source lines executed | Low = untested code paths |
| **Function coverage** | Percentage of functions called | Low = unused or poorly tested functions |
| **Region coverage** | Percentage of basic blocks (branches) | Low = untested if/else branches |
| **Branch coverage** | Percentage of true/false branches taken | Low = missing edge cases in conditionals |

`rvtest` reports line, function, and region coverage by default.

---

## Running Coverage with `rvtest`

`rvtest` has a self-contained coverage system that works without any external
tools:

```bash
cargo rvtest --coverage
```

Output:

```
Coverage: 48.7% lines, 56.1% functions, 48.7% regions
```

`rvtest` compiles your tests with LLVM coverage instrumentation
(`-Cinstrument-coverage`), runs the test binaries, and parses the resulting
`.profraw` files entirely in Rust — no `llvm-profdata` or `llvm-cov` needed.

---

## Coverage Output Formats

```bash
# Plain text summary (default)
cargo rvtest --coverage

# HTML report with line-level detail
cargo rvtest --coverage --coverage-format html

# LCOV for IDE integration (VS Code, IntelliJ)
cargo rvtest --coverage --coverage-format lcov

# JSON for programmatic processing
cargo rvtest --coverage --coverage-format json

# Cobertura XML (GitLab CI, Jenkins)
cargo rvtest --coverage --coverage-format cobertura

# Open HTML report in browser
cargo rvtest --coverage --coverage-open
```

The self-contained parser works automatically.  If you have `cargo-llvm-cov`
installed, `rvtest` uses it for enhanced reports instead.

---

## Setting Coverage Thresholds

Fail the build if coverage drops below a threshold:

```bash
cargo rvtest --coverage --coverage-min 80
```

This exits with a non-zero code if line coverage is below 80%.  Useful for CI
pipelines.

---

## Interpreting Coverage Results

### High Coverage Is Not Everything

```
Coverage: 95% lines, 90% functions, 92% regions
```

This looks good, but high coverage does not guarantee good tests.  You could
have 100% line coverage with tests that never assert anything useful:

```rust
fn add(a: i32, b: i32) -> i32 {
    if a > 0 && b > 0 {
        a + b  // Covered
    } else if a < 0 && b < 0 {
        a + b  // Covered
    } else {
        a + b  // Covered
    }
}

// This test covers every line but never asserts!
#[test]
 fn test_add() {
     add(1, 2);
     add(-1, -2);
     add(1, -1);
 }
```

### Low Coverage Points to Problems

```
Coverage: 30% lines
```

This clearly shows most of your code has no tests.  Start by identifying
untested functions and adding tests for them.

### Changes in Coverage Over Time

The most valuable use of coverage is **trending**.  If coverage drops from
80% to 75% between releases, new code was added without tests.  Set a CI
threshold to catch this.

---

## Improving Coverage

### Step 1: Find Untested Code

Run with HTML output and open the report:

```bash
cargo rvtest --coverage --coverage-format html --coverage-open
```

The HTML report highlights uncovered lines in red.  Focus on:

- Functions with 0% coverage (completely untested)
- Branches with partial coverage (untested if/else paths)
- Error handling code (often left uncovered)

### Step 2: Add Tests for Critical Paths

Not all code needs the same level of coverage:

```rust
// Critical: needs thorough testing
pub fn calculate_price(base: f64, tax_rate: f64, discount: f64) -> f64 {
    let after_tax = base * (1.0 + tax_rate);
    let after_discount = after_tax * (1.0 - discount);
    after_discount.max(0.0)
}

// Less critical: simple delegation
fn format_price(price: f64) -> String {
    format!("${:.2}", price)
}
```

### Step 3: Test Error Paths

Error handling is frequently left uncovered:

```rust
pub fn read_config(path: &str) -> Result<Config, String> {
    let content = std::fs::read_to_string(path).map_err(|e| format!("cannot read: {e}"))?;
    toml::from_str(&content).map_err(|e| format!("cannot parse: {e}"))
}
```

Test both the success path AND the error path:

```rust
#[test]
 fn read_config_success() {
     let tmp = tempfile::TempDir::new().unwrap();
     let path = tmp.path().join("config.toml");
     std::fs::write(&path, "key = 'value'").unwrap();
     assert!(read_config(path.to_str().unwrap()).is_ok());
 }

 #[test]
 fn read_config_file_not_found() {
     assert!(read_config("/nonexistent/file.toml").is_err());
 }

 #[test]
 fn read_config_invalid_toml() {
     let tmp = tempfile::TempDir::new().unwrap();
     let path = tmp.path().join("bad.toml");
     std::fs::write(&path, "[[[invalid").unwrap();
     assert!(read_config(path.to_str().unwrap()).is_err());
 }
```

---

## What Coverage Cannot Tell You

Coverage is a tool, not a goal.  It cannot tell you:

- Whether your assertions are correct
- Whether you tested the right edge cases
- Whether your tests are deterministic
- Whether your tests are fast

A 100% coverage test suite can still be useless if every test just calls a
function without asserting.  Coverage measures **quantity**, not **quality**.

---

## Coverage in CI

Add coverage checks to your CI pipeline:

```yaml
# .github/workflows/ci.yml
- name: Test with coverage
  run: cargo rvtest --coverage --coverage-min 80

- name: Upload coverage report
  uses: actions/upload-artifact@v4
  with:
    name: coverage-report
    path: target/coverage/
```

---

## Summary

- Code coverage shows which parts of your code are executed during tests
- `cargo rvtest --coverage` runs the self-contained pure-Rust coverage system
- Coverage thresholds in CI prevent untested code from being added
- Low coverage identifies untested functions and branches
- High coverage does not guarantee good tests — it is a quantity metric
- Focus coverage improvement on critical and error-path code
- Track coverage trends over time, not just absolute values

In the next chapter, we will look at flaky tests — how to identify them and
what to do about them.

---

[← Previous](13-snapshot-testing.md) • [Index](00-index.md) • [Next →](15-flaky-tests.md)
