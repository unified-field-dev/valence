# Security Policy

## Supported versions

Security fixes are accepted against the latest published `0.1.x` release line of the `uf-valence*` crates.

## Reporting a vulnerability

Please **do not** open a public GitHub issue for security-sensitive reports.

Prefer one of the following:

1. **GitHub Security Advisories** — use [Report a vulnerability](https://github.com/unified-field-dev/valence/security/advisories/new) on this repository when available.
2. Contact the maintainers privately via the repository owner listed at https://github.com/unified-field-dev/valence.

Include:

- a description of the issue and its impact
- steps to reproduce or a proof of concept when possible
- affected crate names and versions (`uf-valence`, backends, etc.)

We will acknowledge receipt as soon as practical and coordinate a fix and disclosure timeline with you.

## Scope

In scope: vulnerabilities in published `uf-valence*` crates, documentation that could cause unsafe production defaults, and CI/supply-chain issues in this repository.

Out of scope: vulnerabilities solely in third-party databases or clients (Postgres, SurrealDB, Redis, etc.) unless Valence mishandles them in a security-relevant way; demo credentials in local `infra/` compose files.
