# Specs

Machine-oriented contracts for the Product Engineering Agent.

| Spec | Purpose |
|---|---|
| [evidence.schema.json](evidence.schema.json) | Source evidence object |
| [approval.schema.json](approval.schema.json) | Human approval/rejection record |
| [backlog-ticket.schema.json](backlog-ticket.schema.json) | Jira-ready ticket |
| [proposal.schema.json](proposal.schema.json) | Full engineering proposal |
| [hitl-states.md](hitl-states.md) | HITL state machine |
| [pipeline.md](pipeline.md) | Stage responsibilities |

Rust types in `crates/schemas` must stay aligned with these schemas.
