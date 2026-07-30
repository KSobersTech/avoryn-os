# Changelog

All notable changes to AVORYN will be documented in this file.

The project follows an incremental development model. Formal semantic versioning will begin with the first public release.

## Unreleased

### Planned

- SQLite persistence with SQLx
- Persistent Projects vertical slice
- Input validation and structured errors
- Audit events for project creation and updates
- AVORYN visual identity and design system
- Automated tests and GitHub Actions quality checks

## Foundation — July 2026

### Added

- Tauri 2 desktop application initialized with React, TypeScript, Vite, and Rust
- Custom AVORYN Foundation screen
- Reusable application shell and sidebar
- Functional navigation across core workspaces
- Separate page components for Dashboard, Projects, Tasks, Memory, Journal, Knowledge Vault, and Settings
- Product blueprint, architecture documentation, and development history
- Professional README with project status, stack, roadmap, and setup instructions
- Security policy and contribution guide
- Copyright and usage notice reserving AVORYN's commercial and distribution rights

### Changed

- Replaced the default Tauri starter interface with the AVORYN application identity
- Refactored navigation content from a single component into reusable pages
- Clarified that the public repository is for portfolio review and is not currently open source
- Defined the approval requirement for external contributions

### Security

- Established local-first and least-privilege security principles
- Documented current Foundation limitations
- Defined rules against committing secrets, personal databases, Vault content, and private logs
