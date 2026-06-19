# grow-together

> Step-by-step learning content development for the Rust ecosystem.

Grow-together is a framework for creating ecosystem-first learning materials in `docs/learn/`.

## Convention

```
docs/learn/{module}/{submodule}/{short-description}.md
```

- `{module}` — major topic (rust, math, tests, tooling, async, design, …)
- `{submodule}` — logical grouping within that topic (basics, patterns, ecosystem, …)
- `{short-description}` — kebab-case, no numeric IDs

## Principles

1. **Ecosystem-first** — teach Rust, not rvlibs. rvlibs is a vehicle, not the destination.
2. **Self-contained** — each document stands alone with clear prerequisites and next steps.
3. **Step-by-step** — one concept at a time, scaffold from known to unknown.
4. **Timeless** — avoid enumerating state, versions, or plans that will go stale.
5. **Consistent anatomy** — Prerequisites → content → Glossarium → Next Steps.

## Rules

See the `rules/` directory:

| Rule | Principle |
|------|-----------|
| [ecosystem-first](rules/ecosystem-first.md) | Teach Rust, not rvlibs |
| [self-contained](rules/self-contained.md) | Document anatomy convention |
| [step-by-step](rules/step-by-step.md) | Development workflow |
| [tone-and-style](rules/tone-and-style.md) | Voice and accessibility |
