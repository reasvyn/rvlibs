# system-analysis

> SDLC Phase: **Analysis**

Analyse requirements and determine feasibility.

## Steps

### 1. Dependency Impact

Read the dependency graph (`docs/dep-graph.md`). Map the change:

- Which crates need to change?
- Will this create a circular dependency? If yes, the shared contract goes in `rvlibs`.
- Will this require a new public API? Document it.

### 2. Feasibility

- Can this be implemented with safe Rust only?
- Does it require new external dependencies? Keep them minimal.
- Check MSRV for affected crates (see `docs/conventions.md`).

### 3. API Sketch

```rust
// Proposed public API
pub fn new_feature(input: &str) -> Result<Output, rvlibs::Error>;
```

### 4. Register Findings

- If the analysis uncovers issues, register them in `docs/known-issues.md`
- If the analysis completes without issues, update `docs/roadmap.md` with the result

### 5. Human Confirmation

After registration, **stop and wait for human confirmation** before proceeding to design or implementation.
