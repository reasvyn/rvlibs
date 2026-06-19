# feature-development

> SDLC Phase: **Implementation**

Implement features following rvlibs conventions and patterns.

## Steps

### 1. Read Existing Patterns

Before writing code, read:
- The relevant crate's `docs/` files (architecture, conventions, testing)
- Existing similar modules for style reference
- `AGENTS.md` for workspace-wide conventions

### 2. Implementation Checklist

- [ ] Follows the crate naming conventions (see `docs/conventions.md`)
- [ ] No vendor lock-in — no `RvlibsFoo` prefixes
- [ ] Doc comments (`///`) on all public items
- [ ] No `unsafe` unless absolutely necessary and documented
- [ ] Functions return `Result<T, String>` for fallible operations
- [ ] External dependencies kept to a minimum

### 3. Code Style

- Edition 2024, `rustfmt` default
- Single uppercase type params: `T`, `N`, `U`
- `snake_case` for functions and methods
- `PascalCase` for types and traits
- `UPPER_SNAKE_CASE` for constants

### 4. Verify

```bash
cargo check                     # compiles?
cargo clippy -- -D warnings     # clean lints?
cargo fmt --check               # formatted?
cargo test                      # tests pass?
```

### 5. Commit

- One feature per commit
- Descriptive commit message referencing the requirement
- Signed-off (`git commit -s`)
