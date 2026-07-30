# AVORYN Architecture

**Status:** Approved foundation architecture  
**Last updated:** July 30, 2026

## Architectural Style

AVORYN uses a **modular monolith** for Version 1.

This keeps deployment simple while preserving clear boundaries between features. Modules can later be extracted or replaced without redesigning the entire application.

## Primary Modules

- Shell
- Dashboard
- Projects
- Tasks
- Memory
- Knowledge Vault
- Journal
- Decisions
- Promise Ledger
- Trust
- Audit
- Settings
- Shared Core

## Layered Architecture

```text
Presentation
    ↓
Application
    ↓
Domain
    ↓
Infrastructure
```

### Presentation

React and TypeScript render pages, forms, navigation, loading states, validation feedback, and errors.

### Application

Application services coordinate use cases such as creating a project, listing tasks, importing a file, or approving a protected action.

### Domain

The domain layer contains business rules, entities, value objects, policies, and stable error definitions. It must not depend on React, Tauri, SQLite, Windows APIs, or external AI providers.

### Infrastructure

Infrastructure implements database repositories, file-system operations, logging, security adapters, and future provider integrations.

## Frontend-to-Backend Boundary

```text
React component
    ↓
Frontend service
    ↓
Typed Tauri command
    ↓
Rust validation
    ↓
Application service
    ↓
Repository or system adapter
    ↓
SQLite or operating system
```

React components must not access SQLite directly or scatter unrestricted `invoke()` calls throughout the interface.

## Data Ownership

- Rust owns persistent application data.
- SQLite is the source of truth.
- React owns temporary interface state only.
- Backend validation is authoritative even when frontend validation already ran.

## Core Entities

- Project
- Task
- Memory
- JournalEntry
- Decision
- Promise
- VaultItem
- ApprovalRequest
- AuditEvent
- Setting

## Database Principles

- UUID primary keys
- UTC timestamps
- foreign-key enforcement
- parameterized SQL
- transactions for multi-step operations
- optimistic concurrency where needed
- soft deletion for recoverable records
- migration history
- checksums for managed files

## Initial Tables

- `projects`
- `tasks`
- `memories`
- `journal_entries`
- `decisions`
- `promises`
- `vault_items`
- `approval_requests`
- `audit_events`
- `settings`

## Trust Architecture

AVORYN assigns actions a risk level:

| Level | Meaning |
|---|---|
| 0 | Read-only |
| 1 | Reversible local action |
| 2 | Sensitive local action |
| 3 | External action |
| 4 | Financial or security-critical action |

Foundation simulates Levels 3 and 4 rather than performing autonomous external actions.

Protected operations require approval records that are validated in Rust, expire appropriately, cannot be replayed, and generate audit events.

## File Architecture

The Knowledge Vault uses managed storage:

1. User selects a file through an approved scoped picker.
2. Rust validates the file reference, type, size, and path.
3. AVORYN calculates a SHA-256 checksum.
4. AVORYN copies the file into managed application storage.
5. SQLite stores metadata and relationships.
6. The action is audited.

Original user filenames are display metadata, not trusted storage paths.

## Security Principles

- local-first behavior
- single-user Foundation release
- least-privilege Tauri capabilities
- no generic shell-execution command
- no arbitrary SQL command
- no arbitrary file deletion
- restrictive content security policy
- secrets excluded from Git
- sensitive content excluded from normal logs
- stable public error codes
- explicit approval for protected actions

## AI Provider Boundary

AI support will use a provider-neutral interface. Planned implementations may include:

- OpenAI provider
- local provider
- additional remote providers
- mock provider for tests and early architecture

No domain rule should depend directly on one provider.

## Future Mobile Boundary

A future mobile companion must communicate through a versioned gateway. It will never access the desktop SQLite database directly.

## Frontend Structure

```text
src/
├── components/
├── layouts/
├── pages/
├── hooks/
├── services/
├── styles/
├── types/
├── utils/
├── App.tsx
└── main.tsx
```

As features grow, feature-first modules will be introduced so each major domain owns its pages, components, hooks, validation, and types.

## Backend Target Structure

```text
src-tauri/src/
├── application/
├── commands/
├── domain/
├── infrastructure/
├── shared/
├── lib.rs
└── main.rs
```

Tauri command handlers remain thin. SQL belongs in repository implementations, not commands or React.

## Quality Gates

Frontend:

```text
npm run typecheck
npm run lint
npm run test
npm run build
```

Rust:

```text
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo build
```

A feature is complete only when its behavior, validation, security, testing, documentation, and recovery states are addressed.

## First Persistent Vertical Slice

```text
Create Project
    ↓
Typed frontend service
    ↓
Tauri command
    ↓
Rust application service
    ↓
SQLite repository
    ↓
Audit event
    ↓
Typed result
    ↓
Rendered project
```

Required proof:

1. Create a project.
2. Close AVORYN.
3. Reopen AVORYN.
4. Confirm the project persists.
5. Update it.
6. Reopen again.
7. Confirm the update persists.
8. Verify the audit record.
