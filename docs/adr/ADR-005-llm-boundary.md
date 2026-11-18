# ADR-005: LLM boundary and schema validation

| Field | Value |
|---|---|
| Status | Accepted |
| Date | 2026-11-18 |
| Decider(s) | Solution Architect (persona); Staff Engineer (persona) |
| Area | Backend |

## Context

AI reasoning must be separated from deterministic application logic. LLM outputs must follow validated schemas. Tests need mocked responses.

## Decision

- Introduce a `LlmProvider` trait with `complete_structured(prompt, schema) -> JsonValue`.
- Implementations: `HttpOpenAiCompatibleProvider`, `MockLlmProvider`.
- Deterministic crates never call HTTP directly; only `llm` crate does.
- All structured outputs deserialize into `crates/schemas` types; invalid JSON/schema ⇒ stage failure with recovery (retry once, then `Failed` or partial with open questions — documented in agent).
- Prompts may include evidence excerpts; prompts must not be treated as source of truth over repository scans.

## Consequences

- Swap providers without rewriting pipeline.
- CI runs without API keys.
- Hallucinated fields fail closed at validation.

## Alternatives rejected

| Alternative | Why rejected |
|---|---|
| Provider SDKs embedded in agent crate | Couples reasoning boundary |
| Free-form markdown as sole artifact | Not schema-validatable |

## Files touched

| Path | Change |
|---|---|
| `docs/adr/ADR-005-llm-boundary.md` | added |
| `docs/specs/proposal.schema.json` | added |
| `crates/llm/**` | planned |

## Related

- ADRs: ADR-002, ADR-003
