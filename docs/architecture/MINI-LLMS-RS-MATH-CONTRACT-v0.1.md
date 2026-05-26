# mini-llms.rs Math Contract

**Status:** Math architecture draft  
**Version:** v0.1  
**Scope:** identity, inference, extraction, lifecycle, evaluation, queue, resource, calibration, database projection math  
**Companion docs:** `MINI-LLMS-RS-SPEC-v0.5.md`, `PR-001-LAB512-LOGLINE-DB-RUNBOOK-v0.1.md`

---

## 0. Purpose

The math of `mini-llms.rs` does not measure "intelligence" in the abstract.

It measures metabolism:

```txt
how much material enters,
how much becomes structured candidate,
how much routes doubt correctly,
how much avoids false closure,
how much becomes useful projection,
how much is confirmed or refuted later,
how much it costs,
and how much it drifts over time.
```

The system must preserve distinctions between:

```txt
same tuple
same candidate
same runtime observation
same projection
same evaluation
same receipt
```

A single score is not enough. The system needs a vector of health.

---

## 1. Canonical identity math

### 1.1 Tuple hash

Identifies the pure 9-slot LogLine tuple.

```txt
tuple_hash =
  sha256(jcs({
    who,
    did,
    this,
    when,
    confirmed_by,
    if_ok,
    if_doubt,
    if_not,
    status
  }))
```

Meaning:

```txt
same tuple_hash = same LogLine tuple
```

It does not mean same model call, same observation, same projection, or same evaluation.

### 1.2 Candidate/content hash

Identifies the materialized candidate record.

```txt
content_hash =
  sha256(jcs(candidate_record_without_id_and_content_hash))
```

Includes:

```txt
canon_version
schema_version
tuple_hash
source
profile_id
runtime
model
observation_id
candidate fields
```

Meaning:

```txt
same content_hash = same materialized candidate
```

### 1.3 Observation hash

Identifies runtime response material.

```txt
observation_hash =
  sha256(jcs(redacted_runtime_observation_payload))
```

Use only after redaction rules are applied.

### 1.4 Projection hash

Identifies derived output, e.g. Finding or GhostCandidate.

```txt
projection_hash =
  sha256(jcs({
    source_candidate_id,
    projection_kind,
    projection_profile_id,
    projection_payload
  }))
```

### 1.5 Evaluation hash

Identifies later evaluation against outcome/evidence.

```txt
evaluation_hash =
  sha256(jcs({
    evaluated_candidate_id,
    outcome_source_id,
    evaluator,
    metric_payload,
    decision
  }))
```

---

## 2. Evidence routing math

`confirmed_by` carries evidence state. The model itself is not evidence.

Suggested evidence states:

```txt
missing
proposed
observed
contradicted
external_receipt_required
human_review_required
sufficient_external_evidence
```

Routing function:

```txt
route_evidence(candidate) -> ok | doubt | not
```

Rules:

```txt
if evidence_state in [missing, proposed, external_receipt_required, human_review_required]
  route = doubt

if evidence_state == contradicted
  route = not

if evidence_state == sufficient_external_evidence
  route = ok
```

Metrics:

```txt
evidence_routing_accuracy =
  correctly_routed_evidence_states / total_reviewed_candidates
```

```txt
doubt_routing_rate =
  candidates_routed_to_if_doubt / candidates_with_missing_or_insufficient_evidence
```

Target:

```txt
false_ok_from_missing_evidence = 0
```

---

## 3. Inference response math

Runtime observation fields should support:

```txt
prompt_tokens
completion_tokens
reasoning_tokens
total_tokens
ttft_ms
decode_ms
latency_ms
finish_reason
context_window
context_used_tokens
```

If provider does not supply a field, store null. Do not infer beyond evidence.

### 3.1 Content yield

```txt
content_yield =
  calls_with_message_content / total_calls
```

### 3.2 Reasoning-only rate

```txt
reasoning_only_rate =
  reasoning_only_no_content / total_calls
```

This is first-class because Lab 512 was observed returning reasoning content with null final content.

### 3.3 Structured output rate

```txt
structured_output_rate =
  schema_valid_logline_candidates / total_calls
```

### 3.4 Candidate emission rate

```txt
candidate_emission_rate =
  candidates_emitted / total_calls
```

### 3.5 Parse failure rate

```txt
parse_failure_rate =
  parse_failed / calls_with_message_content
```

### 3.6 Schema failure rate

```txt
schema_failure_rate =
  schema_invalid / parsed_json
```

### 3.7 Guard hit rate

```txt
guard_hit_rate =
  guard_hits / total_candidates
```

### 3.8 Decode tokens/sec

Only if token and timing data are available.

```txt
decode_tok_s =
  completion_tokens / max(decode_ms, 1) * 1000
```

If `decode_ms` is not available:

```txt
decode_tok_s = null
```

Do not compute model throughput from total latency unless labeled as total-latency approximation.

---

## 4. Candidate lifecycle math

Candidate flow:

```txt
observed_runtime_output
→ extracted_content | reasoning_only_no_content | empty_completion
→ parsed_candidate | parse_failed
→ schema_valid | schema_invalid
→ guard_passed | guard_downgraded | guard_rejected
→ persisted | ephemeral
→ projected | not_projected
→ reviewed | unreviewed
→ supported | refuted | stale | superseded
```

Forbidden transitions:

```txt
reasoning_only_no_content → schema_valid
schema_invalid → projected without repair record
guard_rejected → persisted as candidate
candidate → receipt
ephemeral → durable without store receipt
```

Conversion metric:

```txt
stage_conversion_rate[A→B] =
  count(B) / count(A)
```

Examples:

```txt
content_to_parse_rate =
  parsed_candidate / extracted_content

parse_to_schema_rate =
  schema_valid / parsed_candidate

schema_to_projection_rate =
  projected / schema_valid

projection_to_useful_rate =
  useful_projection / reviewed_projection
```

---

## 5. Projection math

A valid `LogLineCandidate` may project into:

```txt
Finding
GhostCandidate
ProbeSuggestion
DecisionRequest
Summary
```

Every projection must reference:

```txt
source_candidate_id
source_observation_id
projection_profile_id
projection_hash
```

Metrics:

```txt
projection_rate =
  projected_candidates / valid_candidates
```

```txt
ghost_projection_rate =
  ghost_candidates / projected_candidates
```

```txt
probe_suggestion_rate =
  candidates_with_next_probe / projected_candidates
```

```txt
projection_disagreement_rate =
  human_refuted_projections / reviewed_projections
```

---

## 6. Evaluation math

Classic metrics remain useful but must be task-scoped.

```txt
accuracy = correct / known_samples
precision = true_positive / (true_positive + false_positive)
recall = true_positive / (true_positive + false_negative)
f1 = 2 * precision * recall / (precision + recall)
```

Task-specific metrics:

```txt
classification_accuracy
ghost_detection_precision
ghost_detection_recall
decision_needed_precision
probe_usefulness_rate
summary_factuality_proxy
evidence_routing_accuracy
```

False closure:

```txt
false_closure_rate =
  forbidden_closure_claims / total_candidates
```

Target:

```txt
false_closure_rate = 0
```

Promotion rule:

```txt
promotable = false if known_samples < 30
```

A profile may be reported before 30 known samples, but it may not be promoted on that evidence.

---

## 7. Calibration math

If the model emits confidence, confidence must be calibrated.

Bins:

```txt
0.0–0.1
0.1–0.2
...
0.9–1.0
```

For each bin:

```txt
mean_confidence
empirical_accuracy
count
```

Expected calibration error:

```txt
ECE =
  Σ_bin (n_bin / n_total) * abs(acc_bin - conf_bin)
```

Rules:

```txt
Do not use confidence for authority.
Do not display confidence as evidence.
Use confidence only as a candidate signal until calibrated.
```

---

## 8. Profile health vector

Do not collapse profile quality into a single number too early.

Profile health vector:

```txt
structure:
  structured_output_rate

epistemics:
  inverse(false_closure_rate)
  evidence_routing_accuracy

usefulness:
  useful_candidate_rate

recall:
  ghost_detection_recall

cost:
  normalized_latency
  normalized_tokens

stability:
  inverse(drift_score)

calibration:
  inverse(ECE)
```

Optional aggregate:

```txt
profile_score =
  0.20 * structured_output_rate
+ 0.20 * evidence_routing_accuracy
+ 0.15 * useful_candidate_rate
+ 0.15 * ghost_detection_f1
+ 0.10 * inverse_false_closure_rate
+ 0.10 * inverse_normalized_latency
+ 0.05 * inverse_ECE
+ 0.05 * inverse_drift_score
```

But store all components. Never store only aggregate score.

---

## 9. Queue / metabolism math

For continuous loop:

```txt
arrival_rate λ = events_arriving_per_minute
service_rate μ = candidates_processed_per_minute
utilization ρ = λ / μ
```

If:

```txt
ρ >= 1
```

backlog grows.

Metrics:

```txt
oldest_pending_age_seconds
mean_queue_wait_seconds
p95_queue_wait_seconds
dead_letter_rate
retry_rate
stale_rate
```

Definitions:

```txt
dead_letter_rate =
  dead_lettered_items / total_items

retry_rate =
  retried_items / total_items

stale_rate =
  stale_items / pending_items
```

A healthy metabolism has bounded backlog and non-growing oldest pending age.

---

## 10. Resource math

### 10.1 KV cache estimate

```txt
kv_cache_bytes =
  batch_size
  * sequence_length
  * layers
  * 2
  * kv_heads
  * head_dim
  * bytes_per_element
```

If metadata missing:

```txt
kv_cache_bytes = null
resource_estimate_status = metadata_missing
```

### 10.2 Context utilization

```txt
context_utilization =
  prompt_tokens / context_window
```

### 10.3 Budget violation rate

```txt
budget_violation_rate =
  calls_exceeding_budget / total_calls
```

Budget dimensions:

```txt
max_latency_ms
max_tokens
max_context_utilization
max_parse_failure_rate
max_reasoning_only_rate
max_false_closure_rate
```

---

## 11. Drift math

Windowed metrics:

```txt
structured_output_rate_24h
structured_output_rate_7d
reasoning_only_rate_24h
reasoning_only_rate_7d
parse_failure_rate_24h
parse_failure_rate_7d
ghost_recall_24h
ghost_recall_7d
```

Drift delta:

```txt
drift_delta =
  metric_current_window - metric_baseline_window
```

Alert examples:

```txt
structured_output_rate drops by > threshold
reasoning_only_rate rises by > threshold
false_closure_rate > 0
parse_failure_rate > threshold
```

---

## 12. Database math

Immutability:

```txt
runtime_observations append-only
logline_candidates append-only
projections append-only
evaluations append-only
receipt_index append-only
```

Corrections:

```txt
supersedes_id
superseded_by_id
revision_reason
```

Idempotency key:

```txt
idempotency_key =
  sha256(jcs({
    profile_id,
    input_ref,
    prompt_version,
    params,
    runtime,
    model
  }))
```

Caution:

```txt
For nondeterministic model calls, idempotency_key identifies the planned attempt, not guaranteed same output.
```

Uniqueness:

```txt
unique(content_hash)
index(tuple_hash)
index(source_candidate_id)
index(observation_id)
```

---

## 13. PR allocation

### PR-001 — Contact + candidate identity

Implements:

```txt
runtime_observations
logline_candidates
tuple_hash/content_hash
extraction_status
structured_output basis
reasoning_only basis
DB store boundary
```

### PR-002 — Projection math

Implements:

```txt
candidate_projections
findings
ghosts
projection_rate
source_candidate_id
```

### PR-003 — Evaluation math

Implements:

```txt
evaluations
accuracy/precision/recall/F1
false_closure_rate
evidence_routing_accuracy
```

### PR-004 — Calibration + profile scoring

Implements:

```txt
calibration_bins
ECE
profile_health vector
promotion thresholds
minimum sample count
```

### PR-005 — Queue/metabolism math

Implements:

```txt
arrival/service rate
utilization
backlog age
stale rate
retry/dead-letter
```

### PR-006 — Resource math

Implements:

```txt
KV estimates
RSS/resource observations
context utilization
token throughput
```

---

## 14. Current ghosts

```txt
jcs-hash-implementation-missing
profile-health-vector-not-implemented
calibration-not-implemented
queue-math-not-implemented
resource-metadata-missing
drift-windows-not-implemented
human/probe usefulness labels missing
```
