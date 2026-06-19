# architecture-design

> SDLC Phase: **Design**

Design the architecture and module structure for new features.

## Steps

### 1. Module Placement

Determine where the new code lives:

- **New crate** — Only if it has zero circular deps risk and provides a distinct capability (`crates/`).
- **New submodule** — Within an existing crate, following `src/{module}/{submodule}.rs`.
- **Existing module** — If it extends an existing concern (e.g., adding a new shape to `geometry`).

### 2. Design Principles

Apply rvlibs design principles (see `docs/philosophy.md`):

- **Modular** — Single responsibility per module. Can it be used independently?
- **Atomic** — Are the building blocks small and composable?
- **Composite** — Do the pieces compose naturally?

### 3. Type Design

```rust
// Example: type-first design
pub struct NewType {
    // fields with clear semantics
}
```

- Use newtypes for type safety over raw primitives
- Implement standard traits: `Debug`, `Clone`, `PartialEq` where applicable
- Prefer `pub` fields over getters/setters for simple data types

### 4. API Surface

- Public items must have doc comments (`///`)
- Functions return `Result<T, String>` for fallible operations (or `rvlibs::Result<T>`)
- Panics are reserved for programming errors only

### 5. Output

A design brief with: module placement rationale, type/struct definitions, trait implementations, and code examples.
