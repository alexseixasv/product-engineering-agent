# ADR-001: Compound Engineering documentation system

| Field | Value |
|---|---|
| Status | Accepted |
| Date | 2026-11-18 |
| Decider(s) | Staff Engineer (persona); Solution Architect (persona) |
| Area | Cross-cutting |

## Context

The selection exercise requires transparent engineering judgment, AI-usage disclosure, and a repository that stays AI-optimized as decisions accumulate. Without a durable documentation system, agents and humans will re-litigate decisions and lose evidence of why the system looks the way it does.

## Decision

We adopt a Compound Engineering–aligned documentation system:

- Product decisions live in `docs/pdr/` as numbered PDRs.
- Architecture decisions live in `docs/adr/` as numbered ADRs.
- Plans live in `docs/plans/`, contracts in `docs/specs/`, durable learnings in `docs/solutions/`, and process docs in `docs/process/`.
- Every accepted ADR/PDR records **Decider(s)**, rationale, and **Files touched**.
- Accepted ADRs/PDRs are immutable in intent; reverse by superseding, not rewriting history.
- Project Cursor skills under `.cursor/skills/` encode role workflows.
- `AGENTS.md` is the AI-ready entry map.
- All technical documentation is written in English.

## Consequences

- Slower phase starts (docs first), faster later phases (less re-derivation).
- Reviewers can audit AI contribution via `AI_USAGE.md`.
- Agents have explicit gates and must not commit.

## Alternatives rejected

| Alternative | Why rejected |
|---|---|
| Docs only in README | Does not scale; mixes product, architecture, and process |
| ADRs without Files touched | Breaks CE auditability for file-level change |
| Portuguese technical docs | Misaligned with challenge README language and external reviewers |

## Files touched

| Path | Change |
|---|---|
| `AGENTS.md` | added |
| `docs/adr/TEMPLATE.md` | added |
| `docs/adr/INDEX.md` | added |
| `docs/adr/ADR-001-compound-engineering-documentation-system.md` | added |
| `docs/pdr/TEMPLATE.md` | added |
| `docs/pdr/INDEX.md` | added |
| `docs/process/ROLES.md` | added |
| `docs/process/PHASE_GATES.md` | added |
| `docs/process/AI_USAGE.md` | added |
| `docs/guides/compound-engineering.md` | added |
| `.cursor/skills/**` | added |

## Related

- PDRs: PDR-001
- ADRs: —
