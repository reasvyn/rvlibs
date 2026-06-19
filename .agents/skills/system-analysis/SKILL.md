# system-analysis

> SDLC Phase: **Analysis**

Analyse requirements and determine technical feasibility within the rvlibs workspace.

## Steps

### 1. Dependency Impact

Read the dependency graph (`docs/dep-graph.md`). Map the change:

- Which crates need to change?
- Will this create a circular dependency? If yes, the shared contract goes in `rvlibs`.
- Will this require a new public API? Document it.

### 2. Feasibility

- Can this be implemented with safe Rust only? (project convention)
- Does it require new external dependencies? Keep them minimal.
- Check the MSRV for affected crates (see `AGENTS.md` or `docs/conventions.md`).

### 3. API Sketch

Write a rough API surface:

```rust
// Proposed public API
pub fn new_feature(input: &str) -> Result<Output, rvlibs::Error>;
```

### 4. Compatibility

- Is this a breaking change? (semver MAJOR)
- Can it be additive? (semver MINOR)
- Can it be backward-compatible? (semver PATCH)
