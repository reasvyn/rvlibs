# rustfmt and clippy

Code formatting and linting — keeping your codebase consistent and idiomatic.

## Prerequisites

- [rust-analyzer](rust-analyzer.md) — IDE integration

## rustfmt

```bash
# Format the current project
cargo fmt

# Check formatting without modifying
cargo fmt --check

# Format a specific file
rustfmt src/main.rs

# Custom config
```

```toml
# rustfmt.toml
max_width = 100
tab_spaces = 4
newline_style = "Unix"
```

## Clippy in CI

```yaml
# .github/workflows/ci.yml
- name: Lint
  run: cargo clippy --all-targets --all-features -- -D warnings

- name: Format
  run: cargo fmt --check
```

## Suppressing Clippy

Sometimes you need to suppress a clippy lint:

```rust
#[allow(clippy::needless_range_loop)]
fn sum_squares(n: usize) -> usize {
    let mut sum = 0;
    for i in 0..n {
        sum += i * i;
    }
    sum
}
```

## Glossarium

| Term | Definition |
|------|------------|
| rustfmt | Rust's official code formatter. Ensures consistent style across projects. |
| Clippy | A collection of lints that catch common mistakes and enforce best practices. |
| `-- -D warnings` | Passes `-D warnings` to rustc, treating all warnings as errors. |

## Next Steps

- [Lints and Diagnostics](../compiler/lints.md) — compiler warnings and clippy in depth
- [rustfmt Book](https://doc.rust-lang.org/rustfmt/)
