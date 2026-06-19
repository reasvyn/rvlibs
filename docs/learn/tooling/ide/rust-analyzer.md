# rust-analyzer

The Rust Language Server — IDE features, diagnostics, code actions, and configuration.

## Prerequisites

- Basic Rust project setup

## What rust-analyzer Provides

| Feature | Description |
|---------|-------------|
| Code completion | Type-aware completions, including method suggestions |
| Go to definition | Jump to the definition of any function, type, or trait |
| Find all references | Find all usages of an item across the project |
| Inline type hints | Show inferred types of variables and closure parameters |
| Diagnostics | Inline compiler errors and warnings as you type |
| Code actions | Auto-fix suggestions, import management, refactoring |
| Hover info | Documentation preview on hover |
| Semantic syntax highlighting | Richer highlighting based on semantic meaning |

## Common Code Actions

- **Auto-import** — Add `use` statements automatically when referring to types
- **Extract function** — Extract selected code into a new function
- **Extract variable** — Extract an expression into a named variable
- **Generate enum match arms** — Generate match arms covering all variants
- **Add missing impl members** — Generate trait method stubs
- **Fix diagnostics** — Apply compiler-suggested fixes with one click

## Configuration

```jsonc
// .vscode/settings.json
{
    "rust-analyzer.cargo.features": ["macros"],
    "rust-analyzer.check.command": "clippy",
    "rust-analyzer.inlayHints.typeHints.enable": true,
    "rust-analyzer.runnables.command": "test"
}
```

## Glossarium

| Term | Definition |
|------|------------|
| LSP | Language Server Protocol — a standard protocol for IDE features. |
| HIR | High-level Intermediate Representation — the AST after name resolution. |
| Inlay Hint | Annotations in the editor showing inferred types (e.g., variable types). |

## Next Steps

- [rustfmt and clippy](rustfmt-clippy.md) — formatting and linting in the IDE
- [rust-analyzer Book](https://rust-analyzer.github.io/book/)
