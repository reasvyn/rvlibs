# requirements-gathering

> SDLC Phase: **Planning**

Elicit, document, and register requirements for features.

## Steps

### 1. Contextualise

- Read the relevant part of `docs/roadmap.md`
- Identify which crate(s) are affected
- Identify existing docs, issues, or discussions that provide background

### 2. Define Scope

- **Who** is the user or stakeholder?
- **What** problem does this solve?
- **Why** now — is this blocking something else?
- **How** will success be measured?

### 3. Register in Roadmap

Add the proposal to `docs/roadmap.md`:

```markdown
### {YYYY-MM-DD} — {Feature Name}

**Status:** proposed
**Source:** requirements-gathering

{Brief description}
```

### 4. Human Confirmation

After registration, **stop and wait for human confirmation** before proceeding.
Do not write code, create PRs, or take any follow-up action without confirmation.
