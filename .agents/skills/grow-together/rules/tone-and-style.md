# Tone and Style

## Voice

- Friendly but precise — imagine explaining to a colleague who knows Rust
  basics but not this specific topic.
- Active voice ("The compiler checks lifetimes" not "Lifetimes are checked…").
- Second person ("you") — address the reader directly.
- Inclusive language — use "they" as singular. Avoid "obviously", "just",
  "simply".

## Code Examples

- Every code example must compile on stable Rust unless marked with
  `rust,ignore` or `rust,no_run`.
- Show output where it aids understanding.
- Use `// ❌ ERROR:` for deliberate compile errors.
- Use `// ✅` and `// ❌` to mark correct vs incorrect patterns.

## Structure

- One idea per paragraph.
- Readers should be able to skim the section headers and understand the flow.
- Use lists for sequential steps. Use tables for comparisons.
- ASCII diagrams are encouraged for architecture and data flow.

## Accessibility

- Every code block must have a language tag (` ```rust `).
- Links must have descriptive text: `[Rust Book: Ownership](…)` not `[here](…)`.
- Avoid colour-dependent meaning. If you use colour, describe it in text.
