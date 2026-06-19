# Ecosystem First

Learning materials MUST cover the broader Rust ecosystem and community, not just
the rvlibs libraries.  rvlibs is a starting point, not the destination.

## Why

- Learners need to navigate the real Rust ecosystem — crates.io, docs.rs, Rust
  Book, Rust by Example, rustc, clippy, rust-analyzer.
- rvlibs crates are tools, not the curriculum.  The curriculum is Rust itself.
- Teaching only rvlibs creates vendor lock-in in the learner's mind.

## What to Include

Every document SHOULD reference at least one external resource:

| Resource | Purpose | Link Format |
|----------|---------|-------------|
| The Rust Book | Canonical reference for language concepts | `https://doc.rust-lang.org/book/...` |
| Rust by Example | Code-first learning | `https://doc.rust-lang.org/stable/rust-by-example/...` |
| docs.rs | API documentation | `https://docs.rs/{crate}/{version}/...` |
| crates.io | Package registry | `https://crates.io/crates/{crate}` |
| Rust Reference | Language specification | `https://doc.rust-lang.org/reference/...` |
| Rust Edition Guide | Edition migration | `https://doc.rust-lang.org/edition-guide/...` |
| Rustonomicon | Unsafe Rust | `https://doc.rust-lang.org/nomicon/...` |
| rustc book | Compiler flags and attributes | `https://doc.rust-lang.org/rustc/...` |
| cargo book | Package manager | `https://doc.rust-lang.org/cargo/...` |
| clippy book | Lint collection | `https://doc.rust-lang.org/clippy/...` |

## What to Avoid

| ❌ Avoid | ✅ Instead |
|----------|------------|
| Only referencing rvlibs docs | Reference Rust Book + rvlibs docs |
| "In rvlibs we do X" | "In Rust, the idiomatic way is X. rvlibs follows this pattern." |
| Ignoring community alternatives | Mention alternatives exist, explain why rvlibs chose its approach |
| Outdated ecosystem references | Check that linked versions and APIs are current |

## Rules

1. **Every document must have at least one link to an external Rust resource.**
2. **rvlibs APIs must be contextualised** within the broader Rust ecosystem.
3. **When discussing a pattern, mention how it's done in std / popular crates first, then show rvlibs.**
4. **Do not present rvlibs as "the one true way"** — acknowledge trade-offs.
5. **Link to active community resources** — avoid dead links to deprecated projects.
6. **Use `https://doc.rust-lang.org/stable/...` not `nightly`** unless the feature is nightly-only.
