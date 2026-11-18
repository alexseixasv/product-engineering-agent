---
name: staff-engineer-review
description: Staff Engineer review for CE hygiene, schema gates, evidence requirements, and anti-hallucination. Use before accepting ADRs/specs or merging phase work conceptually.
---

# Staff Engineer Review

## Instructions

1. Verify every architectural claim cites an ADR or spec.
2. Reject findings/contracts that allow conclusions without evidence paths.
3. Ensure LLM outputs must pass schema validation; deterministic logic must not be buried in prompts.
4. Ensure HITL cannot be bypassed for design.md / external actions.
5. Check Files touched sections match actual edits.
6. Output findings as Critical / Suggestion / Nice-to-have.
