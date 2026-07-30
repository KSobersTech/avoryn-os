# AVORYN Blueprint

**Project:** AVORYN  
**Repository:** `KSobersTech/avoryn-os`  
**Current release:** Foundation  
**Founder and Lead Engineer:** Keishmar Sobers  
**Last updated:** July 30, 2026

## Founding Statement

> AVORYN exists to extend—not replace—human intelligence.

## Product Vision

AVORYN is a Windows-first personal intelligence platform designed to help its owner organize projects, preserve knowledge, track commitments, make decisions, and build a durable record of personal and professional growth.

AVORYN is not intended to be another chatbot, a replacement operating system, a collection of disconnected AI tools, a black box, or a privacy risk.

## Core Pillars

1. Intelligence
2. Memory
3. Time
4. Action
5. Knowledge
6. Growth

## Capability Levels

1. **Assistant** — answers questions and supports individual tasks.
2. **Operator** — coordinates workflows and structured actions.
3. **Partner** — understands context, goals, and long-running work.
4. **Legacy** — preserves projects, engineering history, education, certifications, promotions, and career milestones.

## Internal Specialist Model

AVORYN may eventually coordinate specialized internal domains:

- **Nova** — primary conversational intelligence
- **Atlas** — planning and navigation
- **Sage** — knowledge and learning
- **Sentinel** — trust, privacy, and security
- **Archivist** — memory, history, and legacy

These specialists share one governed memory system and timeline.

## AVORYN Operating Loop

1. Observe
2. Understand
3. Remember
4. Plan
5. Ask
6. Act
7. Reflect
8. Improve

## Workspaces

- Engineering
- School
- Career
- Life
- System

## Explain Mode

Important recommendations should explain:

- why the recommendation was made
- confidence level
- alternatives considered
- relevant reasoning

## Product Constitution

AVORYN is governed by these principles:

1. **Purpose** — extend human capability without replacing human judgment.
2. **Ownership** — the owner controls the system and its information.
3. **Trust** — sensitive actions require appropriate safeguards and approval.
4. **Memory** — stored information must be intentional, traceable, and manageable.
5. **Time** — AVORYN should respect deadlines, history, and long-term continuity.
6. **Transparency** — important recommendations and actions should be explainable.
7. **Growth** — the platform should improve alongside its owner.
8. **Modularity** — features should be replaceable without destabilizing the system.
9. **Privacy** — local-first behavior and least-privilege access are defaults.
10. **Legacy** — the system should preserve meaningful personal and engineering history.
11. **Engineering Integrity** — AVORYN must be designed, implemented, tested, secured, and documented according to professional engineering standards.

## Release Names

1. Foundation
2. Partnership
3. Intelligence
4. Legacy

## Version 1 Scope: Foundation

Foundation is a local-first, single-user desktop application with:

- application shell and navigation
- Projects
- Tasks
- Memory
- Knowledge Vault
- Journal
- Decisions
- Promise Ledger
- Trust and approval records
- Audit history
- Settings
- SQLite persistence

## Explicitly Outside Version 1

- voice interaction
- mobile application
- remote gateway
- cloud synchronization
- calendar integration
- GitHub integration
- semantic search
- AI-generated answers
- local large language model
- autonomous automation
- specialist council orchestration
- behavioral pattern detection

## Technology Stack

### Desktop

- Tauri 2

### Frontend

- React
- TypeScript
- Vite
- Tailwind CSS planned

### Backend

- Rust

### Persistence

- SQLite
- SQLx planned for the Rust database layer

### Source Control

- Git
- GitHub

## Ten-Year Engineering Rule

Before accepting a major design decision, ask:

> Will Future Keish appreciate this in 2036?

## Success Measures

AVORYN should be evaluated by:

- time saved
- mistakes prevented
- learning supported
- measurable progress
- reduced stress
- preserved knowledge
- trustworthy operation

## Current Status

Completed:

- Tauri, React, TypeScript, Vite, Rust, and Git environment
- first running Windows desktop build
- custom AVORYN Foundation screen
- reusable application shell
- functional sidebar navigation
- separate page components for Dashboard, Projects, Tasks, Memory, Journal, Knowledge Vault, and Settings

Next milestone:

- Phase 5: SQLite database foundation using SQLx
