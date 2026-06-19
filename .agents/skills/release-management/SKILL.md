# release-management

> SDLC Phase: **Deployment**

Release crates to crates.io, tag versions, and manage the release pipeline.

## Steps

### 1. Version Bump

| Change Type | Bump |
|-------------|------|
| Breaking API change | MAJOR |
| New feature (backward-compatible) | MINOR |
| Bug fix (backward-compatible) | PATCH |

### 2. Pre-Release Checklist

- [ ] All tests pass (`cargo test --workspace`)
- [ ] No clippy warnings (`cargo clippy -- -D warnings`)
- [ ] Documentation builds (`cargo doc --no-deps`)
- [ ] Changelog updated (if applicable)
- [ ] `cargo publish --dry-run` succeeds

### 3. Publish Order

Derive the order from the dependency graph (`docs/dep-graph.md`).
Crates with no internal dependencies publish first; leaf crates last.

```bash
# For each crate in dependency order:
cargo publish -p {crate-name}
```

### 4. Post-Release

- Tag the release in git: `git tag v{version}` && `git push origin v{version}`
- Announce release in relevant community channels
