---
name: qa-contracts-ux
description: QA persona for rigid contract checks, HITL enforcement, UX/UI clarity, and DoD evidence. Use at phase close, UI review, schema validation, or Playwright snapshot verification.
---

# QA Contracts + UX/UI

## Instructions

1. Run the relevant checklist in `docs/process/PHASE_GATES.md`.
2. Verify API/UI field names match `docs/specs/`.
3. Verify approval states are visible and irreversible actions require explicit human action.
4. For UI: clear hierarchy, readable evidence, no ambiguous primary CTA.
5. For snapshots: document how to reproduce.
6. Emit PASS/FAIL verdict; FAIL blocks phase close.
