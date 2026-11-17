# AVORYN Development History

**Purpose:** Preserve a concise engineering record of how AVORYN was designed and built.  
**Last updated:** November 16, 2026

This file records meaningful milestones, decisions, verification steps, and next actions. It is not a full transcript of development conversations or screenshots.

## Pre-Code Architecture

Before implementation, the project established:

- product purpose and founding statement
- six core pillars
- four capability levels
- product constitution
- modular-monolith architecture
- React-to-Rust command boundary
- trust and approval model
- Version 1 scope
- database entity plan
- security threat model
- testing strategy
- Definition of Ready and Definition of Done
- repository and folder structure
- first-night implementation plan

## Development Environment Setup

The Windows development environment was prepared and verified with:

- Git and GitHub
- Visual Studio Code
- Node.js and npm
- Rust, Cargo, and rustup
- MSVC Rust target
- Visual Studio Build Tools
- Desktop development with C++ workload
- Windows SDK
- Tauri and Rust VS Code extensions

The repository was cloned into a local development workspace. Machine-specific paths are intentionally excluded from the public engineering record.

## Phase 1 — AVORYN Foundation

### Accomplishments

- Generated a Tauri 2 project.
- Selected React, TypeScript, Vite, and npm.
- Used the application identifier `com.ksoberstech.avoryn`.
- Installed frontend dependencies.
- Completed the first Rust/Tauri build.
- Launched the first AVORYN Windows desktop executable.
- Verified the React-to-Rust greeting command.
- Replaced the starter interface with the first custom AVORYN Foundation screen.

### Git Checkpoint

```text
2851ca8 — feat: initialize AVORYN Foundation application
```

## Phase 2 — Application Shell

### Accomplishments

- Added a persistent desktop shell structure.
- Created a dark sidebar and light main workspace.
- Added initial Dashboard, Projects, Tasks, Memory, Journal, Knowledge Vault, and Settings entries.
- Split the frontend into reusable component, layout, and page folders.
- Created `Sidebar.tsx`, `MainLayout.tsx`, and `DashboardPage.tsx`.

### Git Checkpoint

```text
127946f — feat: build AVORYN application shell
```

## Phase 3 — Functional Navigation

### Accomplishments

- Created the `PageId` TypeScript union.
- Converted sidebar entries into buttons.
- Added active-page state in React.
- Highlighted the selected navigation item.
- Confirmed all sidebar selections changed the displayed workspace without restarting AVORYN.

### Git Checkpoint

```text
fca1757 — feat: add functional sidebar navigtion
```

The commit message contains the original spelling of “navigation” and is preserved as part of repository history.

## Phase 4 — Separate Page Components

### Accomplishments

Created individual page components for:

- Dashboard
- Projects
- Tasks
- Memory
- Journal
- Knowledge Vault
- Settings

Updated `App.tsx` so navigation renders the appropriate reusable page component rather than inline page markup.

### Git Checkpoint

```text
96f9106 — refactor: separate navigation pages into components
```

## Documentation Checkpoint

Added:

- `docs/AVORYN-BLUEPRINT.md`
- `docs/ARCHITECTURE.md`
- `docs/DEVELOPMENT-HISTORY.md`

These documents preserve product intent, engineering architecture, and implementation history in the repository.

## Phase 5 — SQLite and Projects

### Accomplishments

- Added SQLx and SQLite persistence.
- Added versioned migrations and database initialization.
- Implemented Project domain models, validation, commands, and repository operations.
- Added persistent create, list, and update behavior.
- Added transactional audit events for Project writes.

## Phase 6 — Tasks

### Accomplishments

- Added the Tasks schema and expanded the audit model.
- Implemented Task domain models, commands, and repository operations.
- Added project relationships, scheduling fields, and persistent editing.

## Phase 7 — Memory

### Accomplishments

- Added Memory and tag schemas.
- Implemented typed Memory models and frontend service functions.
- Added persistent Memory creation, listing, and editing.
- Added workspace, kind, importance, source, and optional project metadata.
- Added transactional audit events for Memory writes.

## Current Application State

The current application includes:

- working Tauri desktop window
- Rust backend foundation
- React and TypeScript frontend
- reusable main layout
- working sidebar navigation
- active navigation highlighting
- separate page components
- persistent Projects, Tasks, and Memory workflows
- backend validation and structured errors
- transactional audit records
- versioned SQLite migrations
- GitHub-backed version history

Dashboard, Journal, Knowledge Vault, and Settings remain placeholder workspaces. Memory tag management, archive/recovery behavior, and project selection are the next focused milestone.

## Working Practice

Every meaningful milestone follows this cycle:

```text
Plan
→ Implement
→ Save
→ Run and verify
→ git status
→ git add .
→ git commit
→ git push
→ confirm clean working tree
```

A successful checkpoint ends with:

```text
Your branch is up to date with 'origin/main'.
nothing to commit, working tree clean
```

## Next Engineering Focus

Complete and harden the Memory vertical slice before beginning another Foundation module.
