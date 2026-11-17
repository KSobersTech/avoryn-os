# Changelog

All notable changes to AVORYN are documented here. Formal semantic versioning will begin with the first packaged release.

## Unreleased

### Added

- Windows GitHub Actions quality gate for the frontend build, dependency audit, Rust formatting, Clippy, and tests
- Memory boundary-validation tests

### Changed

- Updated public documentation to match the implemented Projects, Tasks, and Memory vertical slices
- Corrected application and Rust package metadata
- Updated locked frontend dependencies to remediate known security advisories

### Planned

- Memory tag management in the interface
- Memory archive and recovery behavior
- Project-aware Memory selection
- Knowledge Vault managed-file foundation
- Visual design system and accessibility review

## Foundation — Persistent Core

### Added

- Rust-managed SQLite initialization and versioned migrations
- Persistent Projects create, list, and edit workflows
- Persistent Tasks create, list, and edit workflows
- Persistent Memory create, list, and edit workflows
- Shared workspaces, priorities, statuses, and structured application errors
- Parameterized repositories and transactional audit events
- Tag and Memory-tag database schema

## Foundation — Application Shell

### Added

- Tauri 2 desktop application with React, TypeScript, Vite, and Rust
- AVORYN application shell and sidebar navigation
- Dashboard, Projects, Tasks, Memory, Journal, Knowledge Vault, and Settings pages
- Product blueprint, architecture, security policy, contribution guide, and ownership notice

### Security

- Established local-first and least-privilege principles
- Excluded secrets, personal databases, Vault content, and private logs from version control
- Documented the single-user Foundation limitations
