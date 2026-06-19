# release-management

> SDLC Phase: **Deployment**

Release crates to crates.io, tag versions, and manage the release pipeline.

## Steps

### 1. Version Bump

Determine the version bump based on changes:

| Change Type | Bump | Example |
|-------------|------|---------|
| Breaking API change | MAJOR | `0.1.0` → `1.0.0` |
| New feature (backward-compatible) | MINOR | `0.1.0` → `0.2.0` |
| Bug fix (backward-compatible) | PATCH | `0.1.0` → `0.1.1` |

Update the version in the crate's `Cargo.toml`. For workspace-shared metadata, update root `[workspace.package]`.

### 2. Pre-Release Checklist

- [ ] All tests pass (`cargo test --workspace`)
- [ ] No clippy warnings (`cargo clippy -- -D warnings`)
- [ ] Documentation builds (`cargo doc --no-deps`)
- [ ] Changelog updated (if applicable)
- [ ] `cargo publish --dry-run` succeeds

### 3. Publish Order

Publish in dependency order to avoid broken links on crates.io:

1. `rvlibs` (no internal deps)
2. `rvtest-macros` (depends on rvlibs)
3. `rvtest` (depends on rvlibs, optionally on rvtest-macros)
4. `rvmath` (depends on rvlibs)
5. `cargo-rvtest` (depends on rvlibs, rvtest)

### 4. Publishing

```bash
cargo publish -p rvlibs
cargo publish -p rvtest-macros
cargo publish -p rvtest
cargo publish -p rvmath
cargo publish -p cargo-rvtest
```

### 5. Post-Release

- Tag the release in git: `git tag v{version}` && `git push origin v{version}`
- Update the roadmap (`docs/roadmap.md`) if features were completed
- Announce in relevant community channels
