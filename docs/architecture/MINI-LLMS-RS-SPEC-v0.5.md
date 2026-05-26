# Spec: mini-llms.rs — LogLine-First Local Inference Metabolism

**Status:** Architecture draft  
**Version:** v0.5  
**Project name:** `mini-llms.rs`  
**Scope:** local inference metabolism, Lab 512 adapter, LogLineCandidate output contract, database-backed candidate/evidence projection  
**Non-scope:** App Park deployment, receipt closure, Tower mutation, production authority, generic chatbot behavior  
**Parent doctrines:** LogLine Foundation canon, Minilab Constitution, Characters/Ontology Canon, App Park Primordial Packet, Constitutional Runtime primitives  
**Companion docs:** `PR-001-LAB512-LOGLINE-DB-RUNBOOK-v0.1.md`, `MINI-LLMS-RS-MATH-CONTRACT-v0.1.md`

---

## 0. Architectural decision

`mini-llms.rs` remains `mini-llms.rs`.

It is not renamed to Intelligence App.  
It is not the Constitutional Runtime.  
It is not Tower.  
It is not App Park.  
It is not a receipt engine.  
It is not an authority layer.

`mini-llms.rs` is the Rust-native local inference metabolism engine below Operators.

It produces candidate structure from local or private model calls. It records runtime observations. It validates and projects candidates. It does not execute consequence, authorize power, or close receipts.

Canonical sentence:

```txt
mini-llms.rs is the Rust local inference metabolism organ:
it contacts local/private models, extracts candidate LogLines,
records observations, projects findings and ghosts,
and leaves admission, power, execution, and closure to higher layers.
```

Short form:

```txt
mini-llms.rs proposes.
Constitutional Runtime admits.
Tower authorizes power.
LABs execute.
Supabase projects.
Receipts close.
Dan signs consequence.
```

---

## 1. Placement in the Minilab hierarchy

```txt
Dan
  source of intent, consequence signer

ChatGPT Control Plane v0
  soft cockpit, operator surface, project/jurisdiction layer

Minilab
  Dan's constitutional laboratory

Constitutional Runtime
  semantic admission, policy, capability, evidence, closure boundary

mini-llms.rs
  local cognition metabolism below Operators

LAB-512
  private/heavy inference execution place

Supabase/Postgres
  operational projection and queryable memory

Tower
  operational power, protected command windows

App Park
  managed application execution zone

Intelligence App
  future App Park member using mini-llms.rs as substrate
```

This separation prevents four collapses:

```txt
model output ≠ truth
candidate ≠ admitted act
provider success ≠ receipt
LAB identity ≠ authority
```

---

## 2. Canonical surfaces

The system has distinct surfaces. They must not be collapsed.

### 2.1 LogLineCandidate

Primary live model output contract.

A model may propose:

```txt
who
did
this
when
confirmed_by
if_ok
if_doubt
if_not
status
```

A `LogLineCandidate` is not a receipt.  
A valid `LogLineCandidate` is still only a candidate.

### 2.2 Operational Grammar

Compact command/planning/lowering surface.

Example:

```txt
lab.classify kind=receipt_review target=events:latest infer=lab512
host.inspect target=lab512 scope=health
flow.verify_report target=lab8gb infer=lab512
```

Operational Grammar is not the primary model output. It is a program/command surface that may compile toward IR and operational commands.

### 2.3 IRPrimitive / OperationalCommand

Internal Constitutional Runtime planning/lowering artifacts.

They are not human-facing truth. They are executable planning forms under admission.

### 2.4 EvidenceRecord

Observed proof material:

```txt
stdout/stderr
HTTP response
provider response
model response
parse result
schema validation result
guard result
hash
runtime status
database result
```

Evidence is input to closure. It is not closure.

### 2.5 Receipt

Scoped proof closure.

Receipts belong to the Foundation-conformant receipt path. `mini-llms.rs` may prepare receipt candidates or evidence records, but it does not claim closure.

---

## 3. Format decision

Final decision:

```txt
Primary live model output: LogLineCandidate.
Operational Grammar: command/planning/lowering surface.
Finding/GhostCandidate: derived projections.
Receipt: external scoped closure.
```

The Lab 512 adapter should ask the model for a strict LogLine candidate, not ad-hoc `Finding` / `GhostCandidate` JSON.

A valid model output is shaped as:

```json
{
  "canon_version": "logline-canon@unknown-or-configured",
  "schema_version": "mini-llms.logline-candidate.v0",
  "who": "lab512:mistral",
  "did": "propose",
  "this": {},
  "when": "2026-05-26T00:00:00Z",
  "confirmed_by": {
    "evidence_state": "missing",
    "note": "model generation is not evidence"
  },
  "if_ok": "route only after external evidence is supplied",
  "if_doubt": "create ghost or probe request",
  "if_not": "reject or mark contradicted",
  "status": "candidate"
}
```

Forbidden candidate claims:

```txt
verified
closed
approved
safe
production-ready
done
works
receipt-closed
```

---

## 4. Storage decision

No JSONL persistence path.

```txt
JSONL = fixture / export / debug only.
Supabase/Postgres = operational store/projection.
```

Wrong:

```txt
out/logline-candidates.jsonl as default store
runtime-observations.jsonl as pseudo-ledger
files as canonical state
```

Right:

```txt
CandidateStore trait
NullCandidateStore for explicit ephemeral mode
PostgresCandidateStore for Supabase/Postgres
JsonlExporter only behind explicit debug/export flag
```

Files may exist for:

```txt
fixtures
golden samples
test inputs
manual exports
debug packets
offline reproductions
```

Files do not become the operational ledger.

---

## 5. Database projection

Initial schema should use a dedicated namespace:

```sql
create schema if not exists mini_llms;
```

Minimum tables:

```txt
mini_llms.runtime_observations
mini_llms.logline_candidates
mini_llms.findings
mini_llms.ghosts
mini_llms.profile_evaluations
```

Later tables:

```txt
mini_llms.candidate_projections
mini_llms.calibration_bins
mini_llms.queue_metrics
mini_llms.resource_observations
mini_llms.drift_windows
```

Runtime observations and candidates are append-only. Corrections are new records with `supersedes_id`, not destructive updates.

---

## 6. Core Rust contracts

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

---

## 7. Store implementations

### 7.1 NullCandidateStore

Default when no DB is configured.

Behavior:

```txt
does not write files
does not claim durability
returns StoredRef::Ephemeral
prints precise DB ghost when needed
```

### 7.2 PostgresCandidateStore

Writes to Supabase/Postgres.

Rules:

```txt
connection string never printed
secret values never logged
schema/table availability checked by doctor
migration only through explicit command
```

### 7.3 JsonlExporter

Optional debug/export utility.

Rules:

```txt
explicit flag only
not default
not called store
not called ledger
not called persistence
not used by normal runtime path
```

---

## 8. Lab 512 adapter

The adapter should:

```txt
contact Lab 512 Mistral lane
record provider shape
extract message.content only as final candidate source
record reasoning_content if present, but not parse it by default
parse LogLineCandidate if content exists
validate schema
run guard
persist observation and candidate through CandidateStore
optionally derive projections
```

If current provider response has `reasoning_content` and null `message.content`:

```txt
extraction_status = ReasoningOnlyNoContent
candidate_emitted = false
model_performance_valid = false for structured candidate production
```

This is not failure theatre. It is a real runtime observation.

---

## 9. Constitutional Runtime compatibility

Inspected Constitutional Runtime has a `ProposedLogLineAct` documented as a proposed nine-slot LogLine act, but the current shape omits `when`.

Record ghost:

```txt
constitutional-runtime-proposed-logline-act-missing-when
```

Rule:

```txt
Do not copy that shape into mini-llms.rs.
mini-llms.rs LogLineCandidate must include all nine slots.
```

Future work:

```txt
align LogLineCandidate with Constitutional Runtime admission candidate
open upstream issue/patch for missing when
avoid vendoring canon
avoid importing internal upstream functions as law
depend on versioned schema/crate/conformance artifacts when available
```

---

## 10. Versioned roadmap

### v0.1 — Local inference metabolism

Many small local/private model calls. Candidates, not truth.

### v0.2 — Mock skeleton

PR-000: mock runtime, guards, CLI, eval, mock bench, dev fixtures.

### v0.3 — Lab 512 contact

PR-001 initial: Mistral adapter reaches `/v1/models`; live response shape observed.

### v0.4 — LogLine-first correction

Lab 512 output contract becomes LogLineCandidate. Findings/Ghosts become projections.

### v0.5 — DB-first correction

Supabase/Postgres becomes intended operational persistence. JSONL becomes fixture/export/debug only.

### v0.6 — Runtime alignment

LogLineCandidate integrates with Constitutional Runtime admission path. Missing `when` ghost addressed.

### v0.7 — Intelligence App wrapper

mini-llms.rs becomes substrate for a future App Park Intelligence App member.

---

## 11. PR sequence

### PR-000 — Skeleton and Mock Metabolism

Historical skeleton. Keep as parked/draft base.

### PR-001 — Lab 512 Mistral Runtime Adapter

Now corrected to include:

```txt
Lab 512 contact
LogLineCandidate extraction
RuntimeObservation
CandidateStore trait
PostgresCandidateStore
NullCandidateStore
DB schema/migration
doctor --want-db
no JSONL persistence
```

### PR-002 — LogLine Candidate Projection Loop

Derive:

```txt
Finding
GhostCandidate
ProbeSuggestion
DecisionRequest
```

from stored LogLineCandidates, with source references.

### PR-003 — Evaluation and Profile Math

Add evaluations, calibration, profile metrics, promotion thresholds.

### PR-004 — Metabolism Queue

Add continuous loop, backlog/staleness metrics, retry/dead-letter semantics.

### PR-005 — Constitutional Runtime Alignment

Formal adapter from LogLineCandidate to admission path.

### PR-006 — App Park Intelligence Wrapper

Manifest/capabilities/probes for Intelligence App membership. No deploy until admitted.

---

## 12. Non-negotiable invariants

1. `mini-llms.rs` keeps its name.
2. Model output is candidate, not truth.
3. LogLineCandidate has exactly the nine LogLine slots.
4. `confirmed_by` must not pretend model generation is evidence.
5. Missing evidence routes through `if_doubt`.
6. JSONL is not persistence.
7. Supabase/Postgres is the intended operational projection/store.
8. Findings and Ghosts reference source candidate IDs.
9. No receipt closure is claimed by model or adapter.
10. No App Park deployment in PR-001.
11. No Doppler secret value reads in PR-001.
12. No Tower mutation in PR-001.
13. No public route creation.
14. No broad reliability/performance claim from one scoped probe.
15. Metrics must preserve stage distinctions.

---

## 13. Current honest state

```txt
State:
  Architecture draft v0.5.

Grounded by:
  Minilab Constitution, Ontology Canon, App Park Packet, Control Plane doctrine,
  PR-000/PR-001 reports, and inspected Constitutional Runtime shape.

Verified here:
  Conceptual architecture alignment only.

Unverified:
  Current PR-001 branch content.
  Supabase schema availability.
  Doppler config.
  Lab 512 current model options beyond user/Codex report.
  Constitutional Runtime upstream issue status.

Ghosts:
  DB schema not implemented.
  CandidateStore not implemented.
  ProposedLogLineAct missing when.
  LogLine schema package/version not available.
  Profile math not implemented.
```
