# Tone and Style

Writing guidelines for learning content.

## Voice

- **Friendly but precise** — Imagine explaining to a colleague who knows Rust basics but not this specific topic.
- **Active voice** — "The compiler checks lifetimes" not "Lifetimes are checked by the compiler".
- **Second person ("you")** — Address the reader directly.
- **Inclusive language** — Use "they" as singular. Avoid "obviously", "just", "simply", "trivial".

## Code Examples

```rust
// ✅ Good: focused, compilable, annotated
fn example() {
    let x = 42;
    println!("{x}");
}
```

- Every code example must compile on stable Rust unless marked with `ignore` or `no_run`.
- Show the output of code examples where it aids understanding.
- Use `// ❌ ERROR:` for deliberate compile errors, with the error message in a comment.
- Use `// ✅` and `// ❌` to mark correct and incorrect patterns.

## Structure

- **One idea per paragraph.** If a paragraph covers two ideas, split it.
- **Headings are signposts.** Readers should be able to skim the H2s and understand the document's flow.
- **Use lists for sequential steps.** Use tables for comparisons.
- **Diagrams (ASCII) are encouraged** for architecture and data flow.

## Accessibility

- Every code block must have a language tag (` ```rust `, not just ` ``` `).
- Every image/diagram must be described in text (no actual images — use ASCII art).
- Links must have descriptive text: `[Rust Book: Ownership](...)` not `[here](...)`.
- Avoid colour-dependent meaning. If you use colour, describe it in text as well.

## What to Avoid

| ❌ Avoid | ✅ Instead |
|----------|------------|
| "It's simple" / "Obviously" | State the fact without evaluation |
| "Just use X" | "One approach is X. Consider Y when Z." |
| "This is the best way" | "This is a common pattern. Alternatives include..." |
| Cute metaphors that obscure meaning | Direct explanation with concrete examples |
| Walls of text (>10 lines without a break) | Short paragraphs, code, or lists |

## Length Guidelines

| Document type | Target length |
|---------------|---------------|
| Submodule index | 100–200 words |
| Standard lesson | 400–800 words |
| Deep-dive / advanced | 800–1500 words |
| Tutorial with multiple examples | 1000–2000 words |
