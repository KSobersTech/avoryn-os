# AVORYN

> A Windows-first, local-first personal intelligence platform that extends—not replaces—human judgment.

![Quality](https://github.com/KSobersTech/avoryn-os/actions/workflows/quality.yml/badge.svg)

## Engineering Snapshot

| Area | Current implementation |
|---|---|
| Desktop | Tauri 2 application with a React 19 and TypeScript interface |
| Persistence | SQLite database initialized by Rust with versioned SQLx migrations |
| Projects | Persistent create, list, and edit workflows with backend validation and audit events |
| Tasks | Persistent create, list, and edit workflows with project relationships and audit events |
| Memory | Persistent create, list, and edit workflows with workspace, kind, importance, and source metadata |
| Trust boundary | React owns temporary interface state; Rust validates and owns persistent operations |
| Quality | TypeScript strict mode, production build, Rust formatting, Clippy, tests, dependency audit, and GitHub Actions |

## Product Direction

AVORYN is designed to help its owner organize projects, preserve knowledge, track commitments, make decisions, and build a durable record of personal and professional growth. It is both a practical personal system and a flagship software-engineering portfolio project.

The project emphasizes local ownership, traceable actions, explainable recommendations, and narrowly scoped capabilities. Autonomous external actions and AI-generated answers are intentionally outside the Foundation release.

## Current Release

**Release:** Foundation

**Stage:** Active development

**Current milestone:** Complete and harden the Memory vertical slice, then extend the same architecture to remaining Foundation modules.

### Implemented

- Native Windows desktop shell and workspace navigation
- Rust-managed SQLite initialization and versioned migrations
- Persistent Projects, Tasks, and Memory vertical slices
- Typed Tauri command payloads and responses
- Backend validation and structured public errors
- Transactional audit events for create and update operations
- Loading, empty, and error states for implemented workflows
- Automated Windows quality checks

### Not yet implemented

- Memory tag management in the interface
- Knowledge Vault managed-file workflows
- Journal, Decisions, Promise Ledger, Trust, and Audit interfaces
- Database encryption and backup/restore workflows
- AI provider integration, semantic search, cloud sync, mobile access, or autonomous actions

## Architecture

AVORYN is a modular monolith with four deliberate layers:

```text
React presentation
        ↓
Typed Tauri commands
        ↓
Rust domain and application logic
        ↓
SQLx repositories and SQLite
```

Persistent data is owned by Rust. React owns temporary interface state and communicates through narrowly scoped commands. Repository operations use parameterized SQL and transactions; successful writes create audit events in the same transaction.

See [Architecture](docs/ARCHITECTURE.md) for the complete design and trust model.

## Technology Stack

- **Desktop:** Tauri 2
- **Frontend:** React 19, TypeScript, Vite
- **Backend:** Rust
- **Persistence:** SQLite and SQLx
- **Primary platform:** Windows
- **Automation:** GitHub Actions

## Repository Guide

```text
src/                         React presentation and frontend services
src-tauri/src/commands/      Typed desktop command boundary
src-tauri/src/domain/        Core entities and input models
src-tauri/src/infrastructure Database initialization and repositories
src-tauri/migrations/        Versioned SQLite schema
docs/                        Product, architecture, and delivery records
```

## Local Development

### Prerequisites

- Windows 11
- Node.js 22.12 or newer
- Rust stable, Cargo, and rustup
- Microsoft Visual Studio Build Tools with Desktop development with C++
- Windows SDK and WebView2

### Install and run

```powershell
npm ci
npm run tauri dev
```

### Required checks

```powershell
npm audit --audit-level=high
npm run build

cd src-tauri
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
```

The same checks run automatically for pull requests and updates to `main`.

## Security and Privacy

- No secrets, personal databases, Vault content, or private logs belong in Git.
- Backend validation remains authoritative even when the interface validates first.
- SQL is parameterized and migrations are versioned.
- Persistent writes use audit records and transactional boundaries.
- The Foundation release is single-user and local-first; database encryption is not yet implemented.

See [SECURITY.md](SECURITY.md) for supported versions, reporting guidance, and current limitations.

## Documentation

- [AVORYN Blueprint](docs/AVORYN-BLUEPRINT.md)
- [Architecture](docs/ARCHITECTURE.md)
- [Development History](docs/DEVELOPMENT-HISTORY.md)
- [Phase 5: SQLite Projects](docs/PHASE-5-SQLITE-PROJECTS.md)
- [Changelog](CHANGELOG.md)
- [Contribution Guide](CONTRIBUTING.md)

## Ownership and Use

Copyright © 2026 Keishmar Sobers. All rights reserved.

AVORYN is publicly visible for evaluation, demonstration, and professional review. It is not currently distributed under an open-source license. See [COPYRIGHT.md](COPYRIGHT.md) for the full ownership and usage notice.
