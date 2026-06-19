# Self-Contained

Every document stands alone. A reader should be able to start here without
having read the entire module.

## Convention

```
docs/learn/{module}/{submodule}/{short-description}.md
```

No numeric IDs in filenames — `ownership.md` not `01-ownership.md`.

## Index Hierarchy

Each directory level has an `index.md` that references only the next level down.
Never list sibling content at the wrong level.

```
docs/learn/index.md       → lists {module}/index.md
    {module}/index.md      → lists {submodule}/index.md
        {submodule}/index.md → lists individual documents
```

Examples:

| Level | File | Content |
|-------|------|---------|
| Master | `docs/learn/index.md` | `[Rust](rust/index.md)`, `[Tests](tests/index.md)` |
| Module | `rust/index.md` | `[Basics](basics/index.md)`, `[Collections](collections/index.md)` |
| Submodule | `rust/basics/index.md` | `[Ownership](ownership.md)`, `[Borrowing](borrowing.md)` |

Index files at the module and submodule level use a table with two columns:
`{Entity Name}` and `Description`.  Entity is `Submodule` at module level,
`Document` at submodule level.

## Document Anatomy

Every document MUST have these sections in **exactly this order**:

| # | Section | Required | Description |
|---|---------|----------|-------------|
| 1 | `# {Title}` | ✅ | H1, short and descriptive |
| 2 | Overview | ✅ | One paragraph explaining what this document covers |
| 3 | `## Prerequisites` | ✅ | Concepts the reader must know, with links to prior docs or external resources |
| 4 | Content | ✅ | H2/H3 sections with explanations, code examples, diagrams |
| 5 | `## Glossarium` | ✅ | Table of `\| Term \| Definition \|` — only terms introduced in THIS document |
| 6 | `## Next Steps` | ✅ | At least one link, preferably branching internal and external |

## Concrete Rules

### Prerequisites
- Be honest. If the reader needs generics, write "Basic generics — `T`, trait bounds."
- Link to the relevant prerequisite document when available.
- Format: bullet list.

### Glossarium
- Must be a markdown table with columns `Term` and `Definition`.
- Only terms introduced in this document. If a term was defined in a prerequisite, link there instead.
- Definition should be one sentence. Example:

  ```
  | Term | Definition |
  |------|------------|
  | Ownership | The set of rules that govern how Rust manages memory. |
  ```

### Next Steps
- Must offer at least one internal link and one external link.
- Internal: another document in `docs/learn/`.
- External: Rust Book, docs.rs, crates.io, Rust by Example, rustc book, etc.
- Format: bullet list with descriptive link text.

### Code Examples
- Must compile on stable Rust unless marked with `rust,ignore` or `rust,no_run`.
- Show output where it aids understanding.
- Use `// ❌ ERROR:` for deliberate compile errors with the error message.
- Use `// ✅` and `// ❌` to mark correct vs incorrect patterns.
