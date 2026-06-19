# requirements-gathering

> SDLC Phase: **Planning**

Elicit, document, and validate requirements for features in the rvlibs ecosystem.

## Steps

### 1. Contextualise

- Read the relevant part of the roadmap (`docs/roadmap.md`)
- Understand which crate(s) are affected: rvlibs, rvmath, rvtest, rvtest-macros, cargo-rvtest
- Identify existing docs, issues, or discussions that provide background

### 2. Define Scope

Answer these questions before writing any requirement:

- **Who** is the user or stakeholder?
- **What** problem does this solve?
- **Why** now — is this blocking something else?
- **How** will success be measured?

### 3. Write Requirements

Use this template:

```markdown
# Feature: {short name}

## Problem
{one paragraph}

## Success Criteria
- Bullet list of verifiable outcomes
- Each criterion must be testable

## Out of Scope
- Explicitly list what is NOT included
```

### 4. Validate

- Does each success criterion have a corresponding test?
- Is the scope small enough for a single PR (< 400 lines changed)?
- Are there dependencies on other in-progress work?
