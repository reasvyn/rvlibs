# grow-together

> Step-by-step learning content development for the Rust ecosystem.

Grow-together is a framework for creating comprehensive, ecosystem-first learning
materials in `docs/learn/`.  Every piece of content follows a strict step-by-step
methodology, covers the broader Rust ecosystem (not just rvlibs), and adheres to
a consistent document structure.

## Convention

```
docs/learn/{module}/{submodule}/{short-description}.md
```

Each module is a major topic (rust, math, tests, tooling, async, design).
Each submodule is a logical grouping within that topic.
Each document is a focused, self-contained lesson.

## Rules

Rules are loaded from the `rules/` subdirectory.  Load them with the skill tool
when starting a new content-development task.

| Rule | Purpose |
|------|---------|
| [content-structure](rules/content-structure.md) | Document anatomy — Prerequisites, Glossarium, Next Steps |
| [ecosystem-first](rules/ecosystem-first.md) | Cover the full Rust ecosystem, not just rvlibs |
| [step-by-step](rules/step-by-step.md) | The development workflow: research → outline → draft → review → iterate |
| [scope-modules](rules/scope-modules.md) | Which modules and submodules exist and how to add new ones |
| [tone-and-style](rules/tone-and-style.md) | Writing style, inclusive language, code examples |

## Usage

1. Load this skill: `skill grow-together`
2. Load the relevant rules for your task
3. Follow the step-by-step workflow in `rules/step-by-step.md`
4. Ensure every document satisfies `rules/content-structure.md`
5. Ensure every document satisfies `rules/ecosystem-first.md`
