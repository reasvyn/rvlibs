# quality-assurance

> SDLC Phase: **Testing**

Ensure feature quality through automated testing, manual verification, and coverage gates.

## Steps

### 1. Test Placement

| Test Type | Location | When |
|-----------|----------|------|
| Unit test | `src/` inline `#[cfg(test)]` | Internal helper functions |
| Integration test | `tests/` at crate root | Public API behaviour |
| Doc test | `///` doc comments | Usage examples |
| Dogfooded spec | `tests/` using describe/it | rvtest features |

### 2. Test Requirements

- Every public function must have at least one test
- Edge cases: empty, error, boundary, overflow, domain
- Floating-point: use `assert!((a - b).abs() < EPSILON)` not `assert_eq!`
- rvtest: use `rvtest_` prefix for test function names

### 3. Coverage Gate

```bash
# Run with coverage
cargo rvtest --coverage

# Minimum threshold (CI)
cargo rvtest --coverage --coverage-min 75
```

### 4. Pre-Merge Verification

```bash
cargo check
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check
cargo test --workspace
```
