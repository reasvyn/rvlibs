# Content Structure

Every document in `docs/learn/` MUST follow this exact anatomy.

## Convention

```
docs/learn/{module}/{submodule}/{short-description}.md
```

- `{module}` — major topic, e.g. `rust`, `math`, `tests`, `tooling`, `async`, `design`
- `{submodule}` — logical grouping, e.g. `basics`, `collections`, `ecosystem`, `patterns`, `workflow`
- `{short-description}` — kebab-case, no IDs: `ownership.md`, `why-test.md`

Example:
```
docs/learn/rust/basics/ownership.md
docs/learn/tests/patterns/bdd-specs.md
docs/learn/tooling/ecosystem/cargo-publish.md
```

## Document Anatomy

Every document MUST have these sections in order:

```
# Title (H1 — short, descriptive)

One-paragraph overview of what this document covers.

## Prerequisites

- Links or concepts the reader should know before starting
- One bullet per prerequisite, linked to the relevant document if available

## ... main content ...

(H2 and H3 sections as needed — code examples, explanations, diagrams)

## Glossarium

| Term | Definition |
|------|------------|
| Term | One-sentence definition |

Terms introduced in this document. Not a general Rust glossary — only terms
that are essential for understanding this specific document.

## Next Steps

- [Link 1](...) — what to read next
- [Link 2](...) — another direction
- External links encouraged (Rust Book, Rust by Example, docs.rs, crates.io)

Must include at least one link (internal or external).
```

## Rules

1. **No IDs in filenames** — `ownership.md` not `01-ownership.md`.
2. **Every document is self-contained** — a reader should be able to start here without reading the entire module.
3. **Prerequisites must be honest** — if the reader needs to know generics, say so.
4. **Glossarium entries must be scoped** — only terms introduced in THIS document. If a term was defined in a prerequisite document, link to that document instead.
5. **Next Steps must offer choices** — at least one internal path and one external path. External links to Rust Book, docs.rs, crates.io, Rust by Example are preferred.
6. **Code examples must compile or be clearly marked** — use `rust,ignore` or `rust,no_run` when they can't compile standalone.
