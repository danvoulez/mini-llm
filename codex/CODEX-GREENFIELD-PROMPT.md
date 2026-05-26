Build `mini-llms.rs` from zero.

Ignore any previous implementation if present. Use it only if it already matches the spec.

The controlling documents are:

```txt
docs/architecture/MINI-LLMS-RS-SPEC-v0.5.md
docs/architecture/MINI-LLMS-RS-MATH-CONTRACT-v0.1.md
docs/runbooks/PR-001-LAB512-LOGLINE-DB-RUNBOOK-v0.1.md
docs/reference/MISTRAL-RS-REFERENCE.md
docs/decisions/ADR-0001-db-store-and-logline-candidate.md
```

Implement the repo described by those documents.

Required output is a compiling Rust workspace for `mini-llms.rs`.

Do not ask for clarification. Make the best complete implementation possible. If an external runtime or database is unavailable, implement the code path and report a precise ghost.

Hard requirements:

- Keep the project name `mini-llms.rs`.
- Primary model output is `LogLineCandidate`.
- `LogLineCandidate` has version metadata and exactly the nine LogLine slots:
  - `who`
  - `did`
  - `this`
  - `when`
  - `confirmed_by`
  - `if_ok`
  - `if_doubt`
  - `if_not`
  - `status`
- A `LogLineCandidate` is not a receipt.
- Model generation is not evidence.
- Missing evidence routes through `if_doubt`.
- Do not claim truth, verification, closure, approval, safety, or production readiness from model output.
- Supabase/Postgres is the operational store/projection path.
- JSONL is fixture/export/debug only.
- Do not implement JSONL as persistence.
- Do not write `out/*.jsonl` by default.
- Do not vendor LogLine canon.
- Do not vendor `mistral.rs`.
- Treat Lab 512 / `mistral.rs` as an OpenAI-compatible remote runtime.
- Use `/v1/models` for reachability.
- Use `/v1/chat/completions` for inference.

Create this workspace:

```txt
Cargo.toml
README.md
AGENTS.md
docs/
  architecture/
  runbooks/
  decisions/
  reference/
migrations/
  0001_mini_llms_candidates.sql
crates/
  mini-llms-core/
  mini-llms-runtime/
  mini-llms-store/
  mini-llms-mistral/
  mini-llms-cli/
  mini-llms-eval/
  mini-llms-schemas/
profiles/
samples/
tests/
```

Implement at least:

1. Core types:
   - `LogLineCandidate`
   - `RuntimeObservation`
   - `ExtractionStatus`
   - `Finding`
   - `GhostCandidate`
   - profile/config types
   - validation guards

2. Hash/math basis:
   - `tuple_hash`
   - `content_hash`
   - extraction status counts
   - structured output rate basis
   - reasoning-only rate basis
   - parse/schema failure basis
   - false-closure guard count

3. Runtime layer:
   - `InferenceRequest`
   - `InferenceResponse`
   - `LocalInference` trait
   - deterministic mock runtime

4. Mistral/Lab 512 adapter:
   - feature-gated if appropriate
   - OpenAI-compatible HTTP client
   - `/v1/models` doctor check
   - `/v1/chat/completions` inference
   - JSON schema / `response_format` where supported
   - explicit provider-shape recording
   - `ReasoningOnlyNoContent` handling

5. Extraction behavior:
   - if `message.content` contains valid `LogLineCandidate`: parse, validate, guard, emit candidate
   - if `message.content` is null/empty and `reasoning_content` exists: record `ReasoningOnlyNoContent`, emit no candidate
   - if both are empty: record `EmptyCompletion`, emit no candidate
   - if provider shape is unknown: record `UnknownProviderShape`, emit no candidate
   - do not parse `reasoning_content` as final candidate by default

6. Store layer:
   - `CandidateStore` trait
   - `NullCandidateStore`
   - `PostgresCandidateStore`
   - optional explicit `JsonlExporter`, not default and not persistence

7. Database:
   - migration `migrations/0001_mini_llms_candidates.sql`
   - schema `mini_llms`
   - tables:
     - `runtime_observations`
     - `logline_candidates`
     - `findings`
     - `ghosts`
     - `profile_evaluations`
   - include `tuple_hash`, `content_hash`, `unique(content_hash)`, index on `tuple_hash`
   - app-generated UUIDs are acceptable

8. CLI:
   - `mini-llms doctor`
   - `mini-llms doctor --want-mistral`
   - `mini-llms doctor --want-db`
   - `mini-llms classify`
   - `mini-llms ghosts`
   - `mini-llms summarize`
   - `mini-llms eval`
   - `mini-llms bench`
   - `mini-llms db plan`
   - implement `db migrate` only if explicit and safe

9. Tests:
   - LogLineCandidate validation
   - forbidden closure guard
   - `unverified` does not match forbidden `verified`
   - mock runtime determinism
   - extraction status behavior
   - NullCandidateStore does not write files
   - Postgres store compiles
   - CLI smoke where feasible

Required checks:

```bash
cargo fmt --check
cargo test
cargo clippy -- -D warnings
cargo test --features mistral
cargo clippy --features mistral -- -D warnings
```

If Lab 512 is configured, run:

```bash
mini-llms doctor --want-mistral
```

If database is configured, run:

```bash
mini-llms doctor --want-db
```

Do not fake unavailable external systems.

Final response must be:

```txt
Branch:
Commit:
Files changed:
Commands run:
Receipts:
Runtime observations:
DB state:
Candidate result:
Ghosts carried:
Next PR:
```
