---
name: ce-phase-gate
description: Close a Compound Engineering phase with docs, ADR/PDR, AI_USAGE, and QA checklist. Use when finishing a phase, requesting phase approval, or verifying phase gate readiness.
---

# CE Phase Gate

## Instructions

1. Read `docs/process/PHASE_GATES.md` universal + per-phase checklist.
2. Verify indexes in `docs/adr/INDEX.md` and `docs/pdr/INDEX.md`.
3. Verify `docs/process/AI_USAGE.md` has an entry for AI work in this phase.
4. As QA persona, emit:

```markdown
## QA Verdict — Phase N
Status: PASS | FAIL
Checklist: …
Defects: …
Reviewed by: QA persona
Date: YYYY-MM-DD
```

5. If FAIL, list blocking defects; do not recommend starting the next phase.
6. If PASS, stop and ask the human to approve the next phase. Do not commit.
