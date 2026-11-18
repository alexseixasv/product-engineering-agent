# Pipeline stages

| Order | Stage | Deterministic inputs | LLM role |
|---|---|---|---|
| 1 | Repository Discovery | file tree, symbol/index scan | optional summarization of evidenced findings |
| 2 | Design System Inspection | UI component paths | map reuse opportunities |
| 3 | Architecture & Feasibility | findings + request | approaches/risks; mark assumptions |
| 4 | Technical Debt | debt heuristics + evidence | impact classification |
| 5 | FR & NFR | request + gaps | structured requirements |
| 6 | Backlog | requirements | epic/story/task with requirement_ids |
| 7 | HITL | proposal JSON | none |
| 8 | design.md | approved proposal | structured markdown generation |
| 9 | Snapshot | design + fixture components | none (Playwright) |

Recovery: on LLM failure, retry once; if still failing, set run `Failed` with error detail. Partial deterministic findings remain stored.
