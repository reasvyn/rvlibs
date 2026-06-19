# CI Integration

Running rvtest in CI pipelines — GitHub Actions, coverage gates, and test reporting.

## Prerequisites

- [Coverage](coverage.md) — code coverage basics
- Basic CI/CD knowledge (GitHub Actions)


## Basic CI

```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v6
      - name: Install Rust
        uses: dtolnay/rust-toolchain@master
        with:
          toolchain: stable
      - run: cargo test --workspace --verbose
```

## With rvtest CLI

```yaml
- name: Run tests with rvtest
  run: cargo rvtest

- name: Coverage
  run: cargo rvtest --coverage
```

## JUnit XML Output

For CI platforms that consume JUnit XML:

```bash
cargo rvtest -F junit > results.xml
```

GitHub Actions integration:

```yaml
- name: Test with JUnit output
  run: cargo rvtest -F junit > results.xml

- name: Publish test report
  uses: dorny/test-reporter@v1
  if: always()
  with:
    name: Rust Tests
    path: results.xml
    reporter: java-junit
```

## Coverage Gates

Fail CI when coverage drops below a threshold:

```bash
cargo rvtest --coverage --coverage-min 80
```

Workspace-wide:

```bash
cargo rvtest --coverage --coverage-min 75 --workspace
```

## Workspace Testing

```bash
# Run all tests in a workspace
cargo rvtest --workspace

# With specific format and coverage gate
cargo rvtest --workspace -F junit --coverage --coverage-min 70
```

## Best Practices

- Run `cargo check` first (fast, fails on compile errors)
- Use `--fail-fast` in CI (stop on first failure, save time)
- Set coverage gates at the crate level, not workspace level
- Use JUnit XML for rich test reporting in the CI UI
- Run `cargo rvtest --coverage` on the full test suite, not just changed tests

## Glossarium

| Term | Definition |
|------|------------|
| CI | Continuous Integration — automated testing on every push/PR. |
| JUnit XML | A widely-supported XML format for test results. |
| Coverage Gate | A minimum coverage threshold — CI fails if coverage drops below it. |
| Test Reporter | A CI step that parses test results and displays them in the UI. |


## Next Steps

- [Rust Modules](../../rust/project-structure/modules-and-packages.md) — organising code with modules and workspaces
- [BDD Specs](../patterns/bdd-specs.md) — writing organised test suites
