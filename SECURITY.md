# Security Policy

AVORYN is a local-first Windows desktop application under active development.

## Supported Versions

Security support currently applies to the latest commit on the `main` branch. Formal versioned support will begin with the first public release.

## Reporting a Vulnerability

Do not publish suspected vulnerabilities, exposed secrets, or private-data issues in a public GitHub issue.

Report security concerns privately to the repository owner with:

- a clear description of the issue
- steps to reproduce it
- the affected file, command, or feature
- the expected and observed behavior
- any relevant screenshots or logs with sensitive information removed

## Security Principles

AVORYN is designed around the following principles:

- local-first operation
- least-privilege Tauri capabilities
- backend validation for persistent and sensitive actions
- narrowly scoped React-to-Rust commands
- parameterized database queries
- versioned database migrations
- no secrets, personal databases, Vault files, or private logs committed to Git
- approval gates for destructive or external actions
- minimal logging of personal content

## Current Limitations

The Foundation release is designed as a single-user local application. It does not yet provide:

- encrypted database storage
- remote authentication
- cloud synchronization
- mobile access
- cryptographically tamper-proof audit logs

These limitations will be documented and reviewed before related features are introduced.
