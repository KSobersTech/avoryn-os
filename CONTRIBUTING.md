# Contributing to AVORYN

AVORYN is currently led and maintained by Keishmar Sobers. Contributions should preserve the project's architecture, security model, and professional engineering standards.

## Development Workflow

1. Pull the latest `main` branch.
2. Create a focused feature branch.
3. Make one coherent change at a time.
4. Run the required checks.
5. Commit with a clear conventional-style message.
6. Push the branch and open a pull request when the workflow is enabled.

Example branch names:

- `feature/sqlite-foundation`
- `feature/projects-create-flow`
- `fix/project-validation`
- `docs/update-architecture`

## Commit Style

Use clear, focused commit messages such as:

- `feat: add persistent project creation`
- `fix: prevent blank project names`
- `refactor: separate navigation pages into components`
- `docs: update database architecture`
- `test: add project repository integration tests`

## Architecture Rules

- React components must not access SQLite directly.
- Frontend Tauri calls belong in service modules rather than scattered throughout components.
- Tauri command handlers should remain thin.
- Business rules belong in the Rust domain or application layers.
- SQL belongs in repository or infrastructure code.
- Persistent data is owned by the Rust backend.
- Protected actions must follow AVORYN's trust and approval model.

## Quality Checks

### Frontend

```powershell
npm run build
```

Additional type-checking, linting, and frontend tests will be added as the project matures.

### Rust

```powershell
cd src-tauri
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
```

## Security and Privacy

Never commit:

- API keys or access tokens
- passwords or certificates
- real `.env` files
- personal SQLite databases
- Knowledge Vault content
- private logs
- confidential client information

## Documentation

Update documentation when a change affects:

- architecture
- public behavior
- setup instructions
- security assumptions
- database schema
- known limitations
- roadmap status

## Definition of Done

A feature is complete only when:

- acceptance criteria pass
- relevant tests pass
- builds and static checks pass
- error and empty states are handled
- security implications are reviewed
- documentation is accurate
- temporary debugging code is removed
- the Git history clearly explains the change
