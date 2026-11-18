# Product Engineering Agent

### Rust-Powered AI Engineering Challenge

An AI-powered engineering assistant that transforms product feature requests into technically validated, engineering-ready specifications by analyzing an existing codebase.

**Tech Focus:** Rust · AI Agents · Repository Intelligence · Software Architecture · Human-in-the-Loop · React

---

## The Problem

In real-world software teams, Product Managers frequently request new features without having complete visibility into the existing codebase.

A seemingly simple feature can introduce architectural constraints, technical debt, security risks, missing dependencies, and unexpected implementation complexity.

Engineering teams must investigate these concerns before committing to delivery.

**What if an AI agent could perform the initial technical discovery and prepare an evidence-based engineering proposal before development begins?**

## The Challenge

Build a Rust-powered AI agent capable of transforming a natural-language product request into a structured engineering proposal.

The agent must inspect an existing application, understand its architectural conventions, identify reusable components, evaluate feasibility, and generate actionable engineering artifacts.

The agent must not invent technical capabilities or automatically approve its own recommendations.

## Example Scenario

A Product Manager submits:

> "We need an analytics dashboard where managers can monitor team productivity, overdue tasks, and project performance. It should support filters, charts, and CSV export. Can we deliver this next sprint?"

The agent should investigate the existing codebase and answer:

1. What functionality already exists?
2. Which frontend components can be reused?
3. Which APIs and database models support the request?
4. What is missing?
5. What technical debt could affect delivery?
6. What architectural changes are required?
7. What functional and non-functional requirements should be defined?
8. How should the feature be broken down into Jira tickets?
9. What could the proposed frontend look like?

## Expected Workflow

```text
Product Manager Request
         |
         v
Repository Discovery
         |
         v
Design System Inspection
         |
         v
Architecture & Feasibility Analysis
         |
         v
Technical Debt Assessment
         |
         v
Functional & Non-Functional Requirements
         |
         v
Jira-Ready Backlog Generation
         |
         v
Human-in-the-Loop Approval
         |
         v
Design Specification (design.md)
         |
         v
Frontend Prototype & Visual Snapshot
```

## Core Requirements

### 1. Repository Intelligence

Inspect an existing application to identify relevant components, APIs, domain models, database structures, and architectural patterns.

All repository-based findings must include verifiable evidence.

### 2. Architecture Reasoning

Evaluate feature feasibility, architectural constraints, dependencies, risks, and potential implementation approaches.

Distinguish confirmed facts from assumptions and unresolved questions.

### 3. Technical Debt Detection

Identify relevant technical debt, classify its impact, and recommend remediation when necessary.

### 4. Requirements Engineering

Generate structured functional and non-functional requirements with measurable acceptance criteria.

### 5. Jira-Ready Backlog

Transform approved requirements into epics, user stories, technical tasks, dependencies, and definitions of done.

### 6. Human-in-the-Loop

Require explicit human approval before finalizing implementation proposals or performing external actions.

### 7. Design System Awareness

Inspect the application's existing Design System and prioritize reusable components.

### 8. Visual Feature Proposal

Generate a `design.md` specification and a reproducible frontend snapshot for product review.

## Technical Constraints

- The backend must be implemented in **Rust**.
- AI reasoning must be separated from deterministic application logic.
- LLM outputs must follow validated schemas.
- Repository analysis must be safe and read-only by default.
- Technical conclusions must reference source evidence.
- The system must support mocked LLM responses for automated testing.
- Human approval must be recorded and enforced.
- The application must expose a usable interface for reviewing generated proposals.

## Suggested Technologies

| Area | Technology |
|---|---|
| Backend | Rust, Axum, Tokio |
| AI Integration | HTTP-based LLM provider |
| Code Analysis | tree-sitter |
| Serialization | Serde |
| Database | PostgreSQL, SQLx |
| Frontend | React, TypeScript |
| Design System | Tailwind CSS, shadcn/ui |
| Validation | JSON Schema |
| UI Snapshots | Playwright |
| Observability | tracing, OpenTelemetry |

Technology choices beyond Rust are recommendations, not mandatory requirements.

## Engineering Questions

Before implementation, consider:

- Should this be a single agent with tools or a multi-agent system?
- How can repository context be collected efficiently?
- When is AST-based analysis preferable to text search?
- How can hallucinated architectural findings be detected?
- How should agent state and approval transitions be modeled?
- How can the system recover from failed LLM calls?
- How can generated Jira tickets remain traceable to requirements?
- How can visual proposals be generated reproducibly?
- How should analysis accuracy be evaluated?

## Definition of Done

- [ ] A Product Manager can submit a feature request.
- [ ] The agent inspects a real sample codebase.
- [ ] Relevant architecture and components are identified.
- [ ] Technical findings include source evidence.
- [ ] Feasibility and technical debt reports are generated.
- [ ] Functional and non-functional requirements are produced.
- [ ] Jira-ready tickets are generated.
- [ ] Human approval is enforced.
- [ ] A `design.md` specification is produced.
- [ ] A frontend snapshot can be reviewed.
- [ ] Core workflows are covered by automated tests.

## Engineering Philosophy

**AI should accelerate engineering discovery, not replace engineering judgment.**

The system should understand before recommending, validate before concluding, and request approval before acting.

---

**Status:** Engineering Challenge — Initial Specification

This repository will evolve incrementally through architecture decisions, implementation milestones, automated tests, and documented tradeoffs.