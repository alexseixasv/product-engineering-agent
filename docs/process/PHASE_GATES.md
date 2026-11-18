# Phase Gates

Every phase ends with a **QA verdict** and a **human approval** before the next phase starts. Agents must not commit.

## Universal close checklist

- [ ] Phase plan (if any) updated under `docs/plans/`
- [ ] New/changed PDRs and ADRs indexed
- [ ] Every material decision has Decider(s), rationale, Files touched
- [ ] `docs/process/AI_USAGE.md` updated for AI-assisted work
- [ ] `AGENTS.md` still accurate for new paths
- [ ] QA persona recorded PASS/FAIL
- [ ] Human approved moving on (chat message)

## Per-phase gates

### Phase 0 — Documentation & agent OS

- [ ] `docs/` tree exists with adr, pdr, plans, specs, solutions, process, guides
- [ ] ADR and PDR templates present
- [ ] PDR-001 and ADR-001 accepted
- [ ] All seven project skills have valid YAML frontmatter
- [ ] Documentation language is English

### Phase 1 — Architecture freeze + contracts

- [ ] ADRs cover topology, evidence, HITL, LLM boundary, persistence, fixture
- [ ] Specs cover proposal, evidence, approval, backlog ticket
- [ ] Fact vs assumption vs open question modeled
- [ ] HITL state diagram documented

### Phase 2 — Fixture sample app

- [ ] App builds/runs (documented commands)
- [ ] Domain models + APIs documented
- [ ] Design system components identifiable by path
- [ ] Intentional gaps for analytics dashboard documented

### Phase 3 — Rust workspace + HITL core

- [ ] Workspace builds
- [ ] HITL transitions tested; no auto-approve path
- [ ] Schema types validate round-trip

### Phase 4 — Repository intelligence

- [ ] Read-only scanner against fixture
- [ ] Findings require path + excerpt evidence
- [ ] Design-system and debt heuristics covered by tests

### Phase 5 — LLM + pipeline

- [ ] Mock provider used in automated tests
- [ ] LLM outputs schema-validated
- [ ] Failed LLM calls recoverable
- [ ] Pipeline stages produce pre-approval proposal

### Phase 6 — API + review UI

- [ ] Submit / status / review / approve-reject endpoints
- [ ] UI shows evidence, requirements, backlog, approval state
- [ ] API↔UI contracts aligned

### Phase 7 — design.md + snapshot

- [ ] design.md only after approval
- [ ] Playwright snapshot reproducible

### Phase 8 — Hardening + DoD

- [ ] README DoD checklist 100% with evidence links
- [ ] `docs/solutions/` has durable learnings
- [ ] AI_USAGE complete for selection-process disclosure

## Verdict format

```markdown
## QA Verdict — Phase N
Status: PASS | FAIL
Checklist: (link or inline)
Defects: (none | list)
Reviewed by: QA persona
Date: YYYY-MM-DD
```
