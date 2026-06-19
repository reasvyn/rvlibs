# Tests

Testing in Rust with rvtest. From basic assertions to property-based testing, mocking, snapshots, and CI integration.

## Topics

| Document | What You Will Learn |
|----------|---------------------|
| [Why Test](why-test.md) | What testing is, types of tests, cost-benefit, `#[test]` basics |
| [Assertions](assertions.md) | `assert_eq!`, `assert_ok!`, `assert_err!`, `assert_matches!`, `assert_delta!` |
| [BDD Specs](bdd-specs.md) | `describe`/`it` blocks, nesting, tags, timeouts, retries |
| [Parametrized Tests](parametrized-tests.md) | `parametrize`, `parametrize_named`, data-driven testing |
| [Property-Based Testing](property-based-testing.md) | `check`, `Strategy`, `any`, shrinking, custom strategies |
| [Mocking](mocking.md) | `Spy`, `Stub`, `patch!`, call tracking, scoped replacement |
| [Snapshots](snapshots.md) | File-based assertions, `--update-all`, `--review` |
| [Code Coverage](coverage.md) | Self-contained profraw parser, HTML reports, CI integration |
| [CI Integration](ci-integration.md) | GitHub Actions, JUnit XML, coverage gates, workspace testing |
