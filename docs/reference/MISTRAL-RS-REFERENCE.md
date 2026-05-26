# Mistral.rs Reference for mini-llms.rs

**Status:** reference note  
**Purpose:** guide the Lab 512 adapter without vendoring `mistral.rs`.

## Rules

- Do not vendor `mistral.rs`.
- Do not import `mistral.rs` internal functions.
- Treat Lab 512 as an OpenAI-compatible remote runtime.
- Use `/v1/models` for reachability.
- Use `/v1/chat/completions` for inference.
- Prefer structured output through JSON schema / `response_format` when supported.
- Do not assume `message.content` exists; record provider shape.
- If `reasoning_content` exists and `message.content` is null, record `ReasoningOnlyNoContent`.
- Do not parse reasoning content as final candidate by default.
- Provider response is runtime observation, not truth.
- Provider success is not receipt closure.

## Adapter expectations

The adapter should record:

```txt
provider_shape
content_present
reasoning_content_present
extraction_status
logline_parse_status
guard_status
schema_validation_status
candidate_emitted
derived_projection_emitted
```

## Structured output

When supported, request a JSON object matching `LogLineCandidate`.

The target candidate contains version metadata and exactly nine LogLine slots:

```txt
canon_version
schema_version
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

`LogLineCandidate` is not a receipt.
