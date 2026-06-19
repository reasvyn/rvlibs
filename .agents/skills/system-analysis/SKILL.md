# system-analysis

> SDLC Phase: **Analysis**

Analyse requirements and determine technical feasibility within the rvlibs workspace.

## Steps

### 1. Dependency Impact

Map the change against the crate dependency graph (`docs/dep-graph.md`):

```
rvlibs (shared contracts)
├── rvmath
├── rvtest
├── rvtest-macros
└── cargo-rvtest
```

- Which crates need to change?
- Will this create a circular dependency? If yes, the shared contract goes in `rvlibs`.
- Will this require a new public API? Document it.

### 2. Feasibility

- Can this be implemented with safe Rust only? (rvlibs convention)
- Does it require new external dependencies? Minimise them.
- Does it affect the MSRV (1.85 for rvmath, 1.96 for rvtest)?

### 3. API Sketch

Write a rough API surface in code comments:

```rust
// Proposed public API
pub fn new_feature(input: &str) -> Result<Output, rvlibs::Error>;

// Usage example
let result = new_feature("example").unwrap();
```

### 4. Compatibility

- Is this a breaking change? (semver MAJOR)
- Can it be additive? (semver MINOR)
- Can it be backward-compatible? (semver PATCH)

### 5. Output

A brief analysis document covering: crates affected, feasibility, API sketch, and compatibility assessment.
