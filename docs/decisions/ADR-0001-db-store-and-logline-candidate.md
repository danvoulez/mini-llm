# ADR-0001 — LogLineCandidate and DB Store

**Status:** accepted

## Decision

`mini-llms.rs` uses `LogLineCandidate` as the primary live model output contract.

## Decision

Supabase/Postgres is the operational persistence/projection path.

## Decision

JSONL is fixture/export/debug only.

## Consequences

- No default JSONL persistence.
- Add `CandidateStore` trait.
- Add `NullCandidateStore` for explicit ephemeral mode.
- Add `PostgresCandidateStore` for database persistence.
- Add migrations for runtime observations, LogLine candidates, findings, ghosts, and profile evaluations.
- Findings and Ghosts are projections from `LogLineCandidate`.
- A `LogLineCandidate` is not a receipt.

## Non-goals

- No receipt closure.
- No model truth.
- No production readiness claim.
- No vendored LogLine canon.
- No vendored `mistral.rs`.
- No JSONL pseudo-ledger.
