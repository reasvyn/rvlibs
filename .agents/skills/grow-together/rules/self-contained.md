# Self-Contained

Every document stands alone. A reader should be able to start here without
having read the entire module.

## Convention

```
docs/learn/{module}/{submodule}/{short-description}.md
```

No numeric IDs in filenames — `ownership.md` not `01-ownership.md`.

## Anatomy

Every document MUST have these sections in order:

```
# Title

Overview paragraph.

## Prerequisites

Concepts the reader should know before starting, with links.

## ... content ...

(H2 and H3 sections, code examples, explanations)

## Glossarium

Terms introduced in THIS document, one definition per row.

## Next Steps

At least one link to continue learning (internal or external).
```

## Guidelines

- **Prerequisites must be honest** — if the reader needs to know generics, say so.
- **Glossarium entries must be scoped** — only terms introduced in this document.
  If a term was defined in a prerequisite document, link there instead.
- **Next Steps must branch** — offer multiple directions, at least one internal
  and one external (Rust Book, docs.rs, crates.io, Rust by Example).
- **Code examples must compile or be clearly marked** — use `rust,ignore` or
  `rust,no_run` when they can't compile standalone.
