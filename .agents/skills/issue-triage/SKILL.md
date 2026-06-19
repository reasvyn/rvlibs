# issue-triage

> SDLC Phase: **Maintenance**

Triage, diagnose, and resolve issues across the rvlibs workspace.

## Steps

### 1. Classification

Determine the issue type:

| Type | Label | Response Time |
|------|-------|---------------|
| Bug | `bug` | 48h |
| Security vulnerability | `security` | 24h (private disclosure) |
| Feature request | `enhancement` | 1 week |
| Documentation | `documentation` | 1 week |
| Question | — | 1 week |

### 2. Reproduction

For bugs, verify the reproduction:

```bash
# Check if it reproduces on main
cargo test
# Check the specific crate
cargo test -p rvmath
```

- If the issue lacks a reproduction, request one
- If it's a regression, use `git bisect` to find the introducing commit

### 3. Severity

| Severity | Definition | Action |
|----------|------------|--------|
| Critical | Data loss, security, crash | Stop all work, fix immediately |
| High | Feature broken, no workaround | Fix within 48h |
| Medium | Feature broken, workaround exists | Fix within 1 week |
| Low | Cosmetic, enhancement | Fix with next release |

### 4. Resolution

- **Bug**: Write a regression test, fix, then verify
- **Feature**: Route to `requirements-gathering` skill
- **Documentation**: Fix the relevant `.md` file
- **Duplicate**: Close with reference to original issue

### 5. Follow-Up

- Ensure the fix has a test that covers the reported scenario
- Update the issue with resolution notes
- Close only after the fix is merged and released
