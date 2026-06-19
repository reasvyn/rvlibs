# Tests

Testing in Rust with rvtest. From basic assertions and test organisation through property-based testing, mocking, snapshots, coverage, and CI integration.

## Topics

| Document | What You Will Learn |
|----------|---------------------|
| [Why Test](why-test.md) | What testing is, types of tests, cost-benefit, `#[test]` basics |
| [Test Organization](test-organization.md) | Unit vs integration tests, where to put test code |
| [Writing Effective Tests](writing-tests.md) | AAA pattern, naming conventions, FIRST principles |
| [Assertions](assertions.md) | `assert_eq!`, `assert_ok!`, `assert_err!`, `assert_matches!`, `assert_delta!` |
| [Testing Errors](testing-errors.md) | `#[should_panic]`, `Result<T, E>` in tests, `catch_unwind` |
| [Structs and Enums](structs-and-enums.md) | `PartialEq`, `Debug`, custom comparison, enum variants |
| [BDD Specs](bdd-specs.md) | `describe`/`it` blocks, nesting, tags, timeouts, retries |
| [Hooks and Setup](hooks-and-setup.md) | `before_all`, `after_each`, RAII guards |
| [Parametrized Tests](parametrized-tests.md) | `parametrize`, `parametrize_named`, data-driven testing |
| [Property-Based Testing](property-based-testing.md) | `check`, `Strategy`, `any`, shrinking, custom strategies |
| [Mocking](mocking.md) | `Spy`, `Stub`, `patch!`, call tracking, scoped replacement |
| [Snapshots](snapshots.md) | File-based assertions, `--update-all`, `--review` |
| [Flaky Tests](flaky-tests.md) | Detection, retries, quarantine, pass-rate analysis |
| [Benchmark](benchmark.md) | Performance regression, baselines, slow test profiling |
| [Architecture Tests](architecture-tests.md) | Module dependency rules, cycle detection, doc enforcement |
| [Legacy and Refactoring](legacy-and-refactoring.md) | Characterisation tests, golden master, refactoring workflow |
| [Testing Concurrent Code](concurrent-code.md) | Thread safety, shared state, channels, race conditions |
| [Code Coverage](coverage.md) | Self-contained profraw parser, HTML reports, CI integration |
| [Faster Feedback](faster-feedback.md) | Fast mode, watch, daemon, `--changed`, `--retest` |
| [CI Integration](ci-integration.md) | GitHub Actions, JUnit XML, coverage gates, workspace testing |
