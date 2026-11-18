# Guide: Compound Engineering in this repo

## Capture loop

1. **Decide** — write a PDR (product) or ADR (architecture).
2. **Implement** — change only files justified by that decision.
3. **Record** — update Files touched; log AI usage.
4. **Compound** — if a non-obvious lesson appears, add `docs/solutions/`.
5. **Gate** — QA PASS, then human phase approval.

## When to write a PDR

- Scope, persona goals, success criteria, non-goals.
- Demo scenario constraints.

## When to write an ADR

- Agent topology, persistence, LLM boundary, evidence model, HITL, fixture strategy.
- Any decision with non-trivial reversal cost.

## AI-ready hygiene

- Prefer stable IDs (`ADR-00N`, `PDR-00N`, schema `$id`).
- Link decisions from plans and code comments sparingly (prefer specs).
- Keep indexes current in the same change set as the record.
