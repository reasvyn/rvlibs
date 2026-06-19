# issue-triage

> SDLC Phase: **Maintenance**

Triage, diagnose, and register issues.

## Steps

### 1. Classification

| Type | Label |
|------|-------|
| Bug | `bug` |
| Security vulnerability | `security` |
| Feature request | `enhancement` |
| Documentation | `documentation` |

### 2. Reproduce

```bash
cargo test --workspace
```

- If the issue lacks a reproduction, request one
- If it's a regression, use `git bisect` to find the introducing commit

### 3. Severity

| Severity | Action |
|----------|--------|
| Critical (data loss, security, crash) | Fix immediately |
| High (feature broken, no workaround) | Fast-track |
| Medium (feature broken, workaround exists) | Normal schedule |
| Low (cosmetic, enhancement) | Next release |

### 4. Register in Known Issues

Register the finding in `docs/known-issues.md`:

```markdown
### {YYYY-MM-DD} — {Issue Title}

**Source:** issue-triage
**Category:** {bug | design-flaw | security | …}
**Impact:** {low | medium | high | critical}
**Status:** registered

{description and reproduction steps}
```

### 5. Human Confirmation

After registration, **stop and wait for human confirmation** before fixing.
Only **critical** issues may be fixed without confirmation.
