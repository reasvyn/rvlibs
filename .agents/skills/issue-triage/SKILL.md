# issue-triage

> SDLC Phase: **Maintenance**

Triage, diagnose, and resolve issues.

## Steps

### 1. Classification

| Type | Label |
|------|-------|
| Bug | `bug` |
| Security vulnerability | `security` |
| Feature request | `enhancement` |
| Documentation | `documentation` |

### 2. Reproduction

```bash
# Does the issue reproduce on main?
cargo test --workspace
```

- If the issue lacks a reproduction, request one
- If it's a regression, use `git bisect` to find the introducing commit

### 3. Severity

| Severity | Action |
|----------|--------|
| Critical (data loss, security, crash) | Immediate fix |
| High (feature broken, no workaround) | Fast-track |
| Medium (feature broken, workaround exists) | Normal schedule |
| Low (cosmetic, enhancement) | Next release |

### 4. Resolution

- **Bug**: Write a regression test, fix, then verify
- **Feature**: Route to `requirements-gathering` skill
- **Documentation**: Fix the relevant `.md` file
