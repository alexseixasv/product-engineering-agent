---
name: senior-impl-rust
description: Implement Rust workspace code aligned to ADRs and docs/specs with tests and mock-first LLM. Use when building crates, Axum API, repo intel, or agent pipeline.
---

# Senior Implementation (Rust)

## Instructions

1. Read governing ADRs and `docs/specs/` before coding.
2. Keep LLM behind a trait; default tests to mock provider.
3. Prefer small crates: `schemas`, `repo_intel`, `llm`, `agent`, `api`.
4. Never add auto-approve paths.
5. Update related ADR/PDR Files touched and AI_USAGE when materially changing design.
6. Run `cargo test` for touched crates before declaring done.
