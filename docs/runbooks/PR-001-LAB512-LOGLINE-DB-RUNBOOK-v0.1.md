# PR-001 — Lab 512 LogLine Candidate + DB Store Runbook

**Status:** Implementation runbook  
**Version:** v0.1  
**Companion to:** `MINI-LLMS-RS-SPEC-v0.5.md`  
**PR title:** `PR-001 — Lab 512 Mistral Runtime Adapter`  
**Branch:** `codex/pr-001-lab512-mistral-adapter`  
**Base:** PR-000 branch until PR-000 merges  
**Closure state:** Claim. This runbook defines probes and acceptance; it does not assert they have passed.

---

## 0. Purpose

Patch PR-001 so `mini-llms.rs` has the correct live contract and storage boundary.

The goal is not "make Mistral work" in the broad sense.

The goal is:

```txt
Lab 512 contact
→ provider shape observation
→ LogLineCandidate extraction when possible
→ explicit ReasoningOnlyNoContent when not possible
→ guard/schema status
→ database-oriented persistence boundary
→ no JSONL pseudo-ledger
```

---

## 1. Hard decisions

### 1.1 Name

Keep `mini-llms.rs`.

Do not rename repo/crates/binary/docs/PR to Intelligence App.

### 1.2 Output contract

Primary live model output is `LogLineCandidate`.

Not primary:

```txt
Finding JSON
GhostCandidate JSON
Operational Grammar
freeform summary
receipt
```

### 1.3 Storage

No JSONL persistence.

```txt
JSONL = fixture/export/debug only.
Supabase/Postgres = operational store/projection.
```

### 1.4 Authority

No receipt closure.  
No model truth.  
No production readiness claim.  
No App Park deployment.

---

## 2. Build gates before patch

Run from current PR-001 branch:

```bash
cargo fmt --check
cargo test
cargo clippy -- -D warnings
cargo test --features mistral
cargo clippy --features mistral -- -D warnings
```

If any fail, stop and report the pre-existing failure before changing architecture.

---

## 3. Add core data types

Add or adjust core types.

### 3.1 LogLineCandidate

```rust
pub struct LogLineCandidate {
    pub canon_version: String,
    pub schema_version: String,

    pub who: String,
    pub did: String,
    pub this: serde_json::Value,
    pub when: String,
    pub confirmed_by: serde_json::Value,
    pub if_ok: String,
    pub if_doubt: String,
    pub if_not: String,
    pub status: String,
}
```

Rules:

```txt
exactly nine LogLine slots plus version metadata
`when` is required
model generation is not evidence
missing evidence must route to if_doubt
status remains candidate/pending/doubt/error-style
```

### 3.2 RuntimeObservation

```rust
pub struct RuntimeObservation {
    pub run_id: String,
    pub profile_id: Option<String>,
    pub runtime: String,
    pub model: Option<String>,
    pub lab_id: Option<String>,

    pub provider_shape: String,
    pub content_present: bool,
    pub reasoning_content_present: bool,
    pub extraction_status: ExtractionStatus,

    pub logline_parse_status: Option<String>,
    pub guard_status: Option<String>,
    pub schema_validation_status: Option<String>,

    pub candidate_emitted: bool,
    pub derived_projection_emitted: bool,

    pub raw_response: Option<serde_json::Value>,
    pub raw_response_redacted: bool,

    pub error_kind: Option<String>,
    pub error_message: Option<String>,
}
```

### 3.3 ExtractionStatus

```rust
pub enum ExtractionStatus {
    ContentExtracted,
    ReasoningOnlyNoContent,
    EmptyCompletion,
    UnknownProviderShape,
    ParseFailed,
    SchemaInvalid,
    GuardDowngraded,
    GuardRejected,
}
```

---

## 4. Extraction behavior

Do not parse `reasoning_content` as final candidate by default.

Algorithm:

```txt
if message.content is present and non-empty:
  try parse LogLineCandidate
  validate schema
  run guard
  emit candidate only if valid and not rejected

else if message.content is null/empty and reasoning_content exists:
  extraction_status = ReasoningOnlyNoContent
  candidate_emitted = false

else if both are empty:
  extraction_status = EmptyCompletion
  candidate_emitted = false

else:
  extraction_status = UnknownProviderShape
  candidate_emitted = false
```

Record provider shape every time.

---

## 5. Guards

Forbidden model candidate claims include:

```txt
verified
closed
approved
safe
production-ready
done
works
receipt-closed
confirmed as fact
```

Default guard behavior:

```txt
downgrade or reject according to existing guard mode
record guard_status
do not erase raw observation
do not emit candidate if strict reject
```

`confirmed_by` rule:

```txt
confirmed_by must not say the model itself confirmed the act.
```

Acceptable `confirmed_by` for missing evidence:

```json
{
  "evidence_state": "missing",
  "note": "model generation is not evidence"
}
```

---

## 6. Store boundary

Add store trait.

```rust
#[async_trait::async_trait]
pub trait CandidateStore {
    async fn append_runtime_observation(
        &self,
        obs: &RuntimeObservation,
    ) -> anyhow::Result<StoredRef>;

    async fn append_logline_candidate(
        &self,
        candidate: &LogLineCandidate,
    ) -> anyhow::Result<StoredRef>;

    async fn append_finding(
        &self,
        finding: &Finding,
    ) -> anyhow::Result<StoredRef>;

    async fn append_ghost(
        &self,
        ghost: &GhostCandidate,
    ) -> anyhow::Result<StoredRef>;
}
```

### 6.1 StoredRef

Suggested:

```rust
pub enum StoredRef {
    Ephemeral { kind: String, id: String, reason: String },
    Database { table: String, id: String },
    Exported { path: String, id: String },
}
```

`Exported` is for explicit export/debug only.

### 6.2 NullCandidateStore

Default if DB is absent.

```txt
no filesystem writes
no durability claim
returns StoredRef::Ephemeral
```

### 6.3 PostgresCandidateStore

Enabled only when explicitly configured.

Config key by name:

```txt
MINI_LLMS_DATABASE_URL
```

Never print value.

### 6.4 JsonlExporter

Not a store.

Only explicit flag:

```bash
--export-jsonl out/debug/...
```

Do not default to it.

---

## 7. Database migration

Add migration file, for example:

```txt
migrations/0001_mini_llms_candidates.sql
```

Use app-generated UUIDs if extension rights are uncertain. If using `gen_random_uuid()`, guard it explicitly.

Recommended minimal SQL:

```sql
create schema if not exists mini_llms;

create table if not exists mini_llms.runtime_observations (
  id uuid primary key,
  created_at timestamptz not null default now(),

  run_id text not null,
  profile_id text,
  runtime text not null,
  model text,
  lab_id text,

  provider_shape text not null,
  content_present boolean not null default false,
  reasoning_content_present boolean not null default false,
  extraction_status text not null,

  logline_parse_status text,
  guard_status text,
  schema_validation_status text,

  candidate_emitted boolean not null default false,
  derived_projection_emitted boolean not null default false,

  raw_response jsonb,
  raw_response_redacted boolean not null default true,

  error_kind text,
  error_message text
);

create table if not exists mini_llms.logline_candidates (
  id uuid primary key,
  created_at timestamptz not null default now(),

  observation_id uuid references mini_llms.runtime_observations(id) on delete set null,

  canon_version text not null,
  schema_version text not null,

  who text not null,
  did text not null,
  this jsonb not null,
  "when" text not null,
  confirmed_by jsonb not null,
  if_ok text not null,
  if_doubt text not null,
  if_not text not null,
  status text not null,

  evidence_state text not null default 'candidate',
  guard_status text,

  tuple_hash text not null,
  content_hash text not null,

  source text not null,
  profile_id text,
  runtime text,
  model text,

  constraint logline_candidates_content_hash_unique unique (content_hash)
);

create index if not exists logline_candidates_tuple_hash_idx
  on mini_llms.logline_candidates(tuple_hash);

create table if not exists mini_llms.findings (
  id uuid primary key,
  created_at timestamptz not null default now(),

  source_candidate_id uuid references mini_llms.logline_candidates(id) on delete set null,

  kind text not null,
  topic text,
  subject text,
  summary text not null,
  confidence double precision,
  next_probe text,
  status text not null default 'candidate'
);

create table if not exists mini_llms.ghosts (
  id uuid primary key,
  created_at timestamptz not null default now(),

  source_candidate_id uuid references mini_llms.logline_candidates(id) on delete set null,
  source_finding_id uuid references mini_llms.findings(id) on delete set null,

  what text not null,
  why_it_matters text,
  cannot_conclude text,
  smallest_next_probe text,
  status text not null default 'open'
);

create table if not exists mini_llms.profile_evaluations (
  id uuid primary key,
  created_at timestamptz not null default now(),

  profile_id text not null,
  runtime text not null,
  model text,

  sample_count integer not null,
  known_samples integer not null,

  accuracy double precision,
  precision_score double precision,
  recall_score double precision,
  f1 double precision,

  structured_output_rate double precision,
  reasoning_only_rate double precision,
  parse_failure_rate double precision,
  schema_failure_rate double precision,
  false_closure_rate double precision,
  candidate_emission_rate double precision,

  latency_p50_ms double precision,
  latency_p95_ms double precision,
  tokens_per_second_mean double precision,

  profile_score double precision,
  promotable boolean not null default false,

  notes text
);
```

---

## 8. Doctor commands

Extend doctor.

### 8.1 `doctor --want-mistral`

Existing behavior should remain.

Must report:

```txt
Lab 512 config present by name
/v1/models reachable
selected model visible or model ghost named
no secret values printed
```

### 8.2 `doctor --want-db`

New behavior.

Checks:

```txt
MINI_LLMS_DATABASE_URL env var present by name
connection succeeds
schema mini_llms exists or migration needed
required tables exist or migration needed
no secret value printed
```

If DB absent:

```txt
status = ghost
ghost = db-not-configured
```

### 8.3 Migration command

If included:

```bash
mini-llms db plan
mini-llms db migrate
```

Rules:

```txt
db plan is non-mutating
db migrate is explicit mutation
no implicit migration inside normal inference command
```

If migration command is too much for PR-001, include SQL and make `doctor --want-db` report missing tables.

---

## 9. Lab 512 probe matrix

Run scoped probes.

### Probe A: current shape

Ask for LogLineCandidate with current request settings.

Expected:

```txt
either valid content candidate
or ReasoningOnlyNoContent
```

### Probe B: strict JSON-only

Prompt:

```txt
Return only a JSON object with exactly these keys:
canon_version, schema_version, who, did, this, when, confirmed_by, if_ok, if_doubt, if_not, status.
Do not include markdown.
Do not include explanation.
```

Expected:

```txt
schema-valid candidate or precise extraction failure
```

### Probe C: deterministic params

Use lowest-temperature deterministic settings supported by Lab 512.

Expected:

```txt
same extraction accounting
```

### Probe D: final-content option

Only if Lab 512 exposes a known supported option to disable reasoning or force final content.

### Probe E: alternate model

Only if `/v1/models` exposes a suitable non-reasoning/chat-compatible model.

---

## 10. Metrics required in PR-001

Do not implement full profile scoring yet.

But record enough for later math:

```txt
content_present
reasoning_content_present
extraction_status
candidate_emitted
schema_validation_status
guard_status
latency_ms if available
prompt_tokens if available
completion_tokens if available
total_tokens if available
```

Derived basis metrics:

```txt
structured_output_rate
reasoning_only_rate
parse_failure_rate
schema_failure_rate
candidate_emission_rate
```

These can be reported in command summary, not necessarily stored in `profile_evaluations` yet.

---

## 11. Hashing

Implement or stub clearly:

```txt
tuple_hash = sha256(jcs(9 LogLine slots))
content_hash = sha256(jcs(candidate record without id/content_hash))
```

Do not hash non-canonical raw JSON order.

If JCS implementation is not available in PR-001:

```txt
record hash_status = not_implemented
do not populate fake hash
open ghost: jcs-hash-implementation-missing
```

Preferred: implement JCS or use a known crate if already acceptable.

---

## 12. Documentation patches

Update docs to say:

```txt
mini-llms.rs uses LogLineCandidate as the primary live model output contract.
Operational Grammar remains the compact command/program surface.
Supabase/Postgres is the intended operational store/projection.
JSONL is fixture/export/debug only.
A LogLineCandidate is not a receipt.
```

Record compatibility ghost:

```txt
Constitutional Runtime ProposedLogLineAct currently omits when despite being documented as nine-slot.
```

---

## 13. Acceptance criteria

Final required receipts:

```bash
cargo fmt --check
cargo test
cargo clippy -- -D warnings
cargo test --features mistral
cargo clippy --features mistral -- -D warnings
```

Runtime probes:

```txt
doctor --want-mistral reaches Lab 512
doctor --want-db connects or reports precise DB ghost
live Lab 512 probe emits valid LogLineCandidate or records ReasoningOnlyNoContent
```

Storage:

```txt
if DB configured:
  runtime_observation persisted
  candidate persisted if emitted

if DB absent:
  StoredRef::Ephemeral
  no JSONL default persistence
```

Security:

```txt
no secret values printed
raw response redacted or scoped
no forbidden closure labels in emitted candidates
```

Claims forbidden in PR body:

```txt
model truth
receipt closure
production readiness
general Mistral reliability
App Park admission
```

---

## 14. Definition of done

PR-001 is done when:

- [ ] all fmt/test/clippy gates pass
- [ ] Lab 512 doctor reaches `/v1/models`
- [ ] DB doctor succeeds or reports precise ghost
- [ ] LogLineCandidate schema exists and requires `when`
- [ ] ReasoningOnlyNoContent is explicitly represented
- [ ] CandidateStore trait exists
- [ ] NullCandidateStore does not write files
- [ ] PostgresCandidateStore exists or DB ghost is explicitly carried
- [ ] SQL migration exists if DB store is implemented
- [ ] no default JSONL persistence remains
- [ ] docs state JSONL is fixture/export/debug only
- [ ] PR body says what is and is not proven

---

## 15. Open ghosts

Carry these if not closed:

```txt
db-not-configured
postgres-store-unimplemented
jcs-hash-implementation-missing
lab512-reasoning-only-no-content
constitutional-runtime-proposed-logline-act-missing-when
logline-versioned-schema-package-missing
profile-scoring-not-implemented
app-park-membership-not-started
```

---

## 16. Report format back to Dan

Return only:

```txt
Branch:
Commit:
PR URL:
Commands run:
Receipts:
Runtime observations:
DB state:
Candidate result:
Ghosts carried:
Files changed:
Next PR:
```
