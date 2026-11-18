# ADR-003: Evidence-backed findings model

| Field | Value |
|---|---|
| Status | Accepted |
| Date | 2026-11-18 |
| Decider(s) | Solution Architect (persona); Staff Engineer (persona) |
| Area | Cross-cutting |

## Context

Technical conclusions must reference source evidence. LLM reasoning tends to invent APIs and components.

## Decision

Every repository-derived finding MUST include:

```text
Evidence {
  path: string,          // repo-relative path
  start_line: u32,       // 1-indexed inclusive
  end_line: u32,         // 1-indexed inclusive
  excerpt: string        // verbatim snippet
}
```

Claims are classified as:

- **fact** — supported by at least one Evidence
- **assumption** — not evidenced; must be flagged
- **open_question** — unresolved; blocks confident commitment language

Pipeline validation rejects `fact` claims with empty evidence.

## Consequences

- Higher precision, possibly lower recall.
- UI can deep-link to paths/lines.
- Hallucinations become detectable as missing evidence.

## Alternatives rejected

| Alternative | Why rejected |
|---|---|
| Free-text citations only | Not machine-validatable |
| Embedding similarity as proof | Not verifiable for reviewers |

## Files touched

| Path | Change |
|---|---|
| `docs/adr/ADR-003-evidence-model.md` | added |
| `docs/specs/evidence.schema.json` | added |
| `crates/schemas/**` | planned |

## Related

- PDRs: PDR-001
- ADRs: ADR-002, ADR-005
