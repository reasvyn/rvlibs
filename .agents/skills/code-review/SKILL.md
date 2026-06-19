# code-review

> SDLC Phase: **Implementation** (Quality Gate)

Review code changes for correctness, style, and adherence to conventions.

## Checklist

### Correctness

- [ ] Does the code do what the requirement specifies?
- [ ] Are edge cases handled (empty input, error states, boundary values)?
- [ ] Are error messages descriptive and user-facing?
- [ ] Does it avoid panics for expected failure modes?

### Architecture

- [ ] Does it fit within the existing module hierarchy?
- [ ] Does it create any circular dependencies?
- [ ] Is the public API minimal and intentional?
- [ ] Are internal implementation details kept private?

### Conventions

- [ ] No vendor lock-in (`RvlibsFoo`) — disambiguate by module path
- [ ] Naming follows `docs/conventions.md`
- [ ] Doc comments on all public items
- [ ] `cargo clippy` passes with `-D warnings`
- [ ] `cargo fmt --check` passes

### Testing

- [ ] New features include tests
- [ ] rvtest changes use BDD API (dogfooding)
- [ ] Edge cases are tested
- [ ] `cargo test` passes

### Security

- [ ] No `unsafe` without justification
- [ ] No secrets/passwords in code or comments
- [ ] No hardcoded paths or environment-specific values
