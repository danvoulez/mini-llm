# mini-llms.rs Codex Greenfield Package

This package is meant to be copied into the root of a new or disposable `mini-llms.rs` repo before running Codex Online.

## Files

```txt
docs/architecture/MINI-LLMS-RS-SPEC-v0.5.md
docs/architecture/MINI-LLMS-RS-MATH-CONTRACT-v0.1.md
docs/runbooks/PR-001-LAB512-LOGLINE-DB-RUNBOOK-v0.1.md
docs/reference/MISTRAL-RS-REFERENCE.md
docs/decisions/ADR-0001-db-store-and-logline-candidate.md
codex/CODEX-GREENFIELD-PROMPT.md
```

## Use

1. Copy the `docs/` directory into the repo root.
2. Copy or open `codex/CODEX-GREENFIELD-PROMPT.md`.
3. Paste that prompt into Codex Online.
4. Attach the `mistral.rs` archive only as reference if desired.
5. Do not vendor `mistral.rs`.

## Intent

Build `mini-llms.rs` from zero as a Rust local inference metabolism workspace.

## Missing source docs during package build

None
