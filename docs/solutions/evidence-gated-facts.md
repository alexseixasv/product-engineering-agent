---
module: schemas
tags: [anti-hallucination, validation, evidence]
problem_type: design
---

# Facts must carry evidence

## Symptoms

LLM-assisted architecture summaries invent APIs/components that do not exist.

## Root cause

Treating model prose as authoritative findings without a machine-checkable evidence object.

## Working solution

- `Confidence::Fact` requires ≥1 `Evidence { path, start_line, end_line, excerpt }`
- Scanner attaches verbatim excerpts; validation fails closed
- Gaps/assumptions remain expressible without evidence but are labeled

## Prevention

Keep validation in `crates/schemas` independent of the LLM crate; never bypass `validate_proposal` before HITL.
