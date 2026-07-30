# AVORYN

AVORYN is a Windows-first personal intelligence platform built to help its owner organize projects, preserve knowledge, track commitments, and create a durable record of personal and professional growth.

> AVORYN exists to extend—not replace—human intelligence.

## Project Status

**Current release:** Foundation  
**Development stage:** Active  
**Current milestone:** SQLite persistence and the first complete Projects vertical slice

AVORYN is under active development. The current application includes a Tauri desktop shell, React and TypeScript frontend, Rust backend, functional sidebar navigation, and separate page components for Dashboard, Projects, Tasks, Memory, Journal, Knowledge Vault, and Settings.

## Why This Project Exists

AVORYN is being developed as both a practical personal system and a flagship software-engineering portfolio project. The project is designed to demonstrate professional desktop application development, architecture, persistence, security, testing, documentation, and long-term product thinking.

## Technology Stack

- **Desktop framework:** Tauri 2
- **Frontend:** React, TypeScript, Vite
- **Backend:** Rust
- **Database:** SQLite with SQLx *(Phase 5)*
- **Version control:** Git and GitHub
- **Primary platform:** Windows

## Current Capabilities

- Native Windows desktop application
- React-to-Rust communication through Tauri
- Modular application shell
- Functional sidebar navigation
- Separate page components for core workspaces
- Documented architecture, product vision, and development history

## Foundation Roadmap

- [x] Initialize Tauri, React, TypeScript, and Rust application
- [x] Build the AVORYN application shell
- [x] Add functional sidebar navigation
- [x] Separate navigation pages into reusable components
- [x] Document the product vision and architecture
- [ ] Configure SQLite persistence with SQLx
- [ ] Implement persistent Projects management
- [ ] Add validation, error handling, and audit events
- [ ] Establish the AVORYN visual identity and design system
- [ ] Add automated tests and GitHub Actions quality checks

## Planned Core Modules

- Dashboard
- Projects
- Tasks
- Memory
- Knowledge Vault
- Journal
- Decisions
- Promise Ledger
- Trust and Approvals
- Audit
- Settings

## Architecture

AVORYN uses a modular-monolith architecture with clear separation between:

1. **Presentation** — React and TypeScript interface
2. **Application** — use cases and orchestration
3. **Domain** — business rules and core entities
4. **Infrastructure** — SQLite, file system, logging, and operating-system integrations

Persistent data is owned by the Rust backend. React owns temporary interface state and communicates with Rust through narrowly scoped Tauri commands.

## Documentation

- [AVORYN Blueprint](docs/AVORYN-BLUEPRINT.md)
- [Architecture](docs/ARCHITECTURE.md)
- [Development History](docs/DEVELOPMENT-HISTORY.md)

## Local Development

### Prerequisites

- Windows 11
- Node.js and npm
- Rust, Cargo, and rustup
- Microsoft Visual Studio Build Tools with Desktop development with C++
- Windows SDK

### Run the application

```powershell
npm install
npm run tauri dev
```

### Frontend build

```powershell
npm run build
```

### Rust checks

```powershell
cd src-tauri
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
```

## Engineering Standards

AVORYN is developed with the following expectations:

- TypeScript strictness and typed frontend/backend contracts
- Safe, idiomatic Rust
- Parameterized SQL and versioned migrations
- Least-privilege desktop permissions
- Backend validation of all persistent operations
- Meaningful Git commits and documented architectural decisions
- Unit, integration, and end-to-end testing as features mature
- No secrets, personal databases, or private Vault content committed to Git

## Author

**Keishmar Sobers**  
Founder and Lead Engineer  
GitHub: `KSobersTech`

## License

A license will be selected before the first public release. Until then, all rights are reserved.
