use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogLineCandidate {
    pub canon_version: String,
    pub schema_version: String,
    pub who: String,
    pub did: String,
    pub this: Value,
    pub when: String,
    pub confirmed_by: Value,
    pub if_ok: String,
    pub if_doubt: String,
    pub if_not: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub raw_response: Option<Value>,
    pub raw_response_redacted: bool,
    pub error_kind: Option<String>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub kind: String,
    pub payload: Value,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GhostCandidate {
    pub reason: String,
    pub payload: Value,
}

pub fn tuple_hash(c: &LogLineCandidate) -> anyhow::Result<String> {
    let data = serde_json::to_vec(
        &serde_json::json!({"who":c.who,"did":c.did,"this":c.this,"when":c.when,"confirmed_by":c.confirmed_by,"if_ok":c.if_ok,"if_doubt":c.if_doubt,"if_not":c.if_not,"status":c.status}),
    )?;
    Ok(format!("{:x}", Sha256::digest(data)))
}
pub fn content_hash(v: &Value) -> anyhow::Result<String> {
    let data = serde_json::to_vec(v)?;
    Ok(format!("{:x}", Sha256::digest(data)))
}

pub fn forbidden_claim(s: &str) -> bool {
    [
        "verified",
        "closed",
        "approved",
        "safe",
        "production-ready",
        "done",
        "works",
        "receipt-closed",
    ]
    .iter()
    .any(|w| {
        s.to_lowercase()
            .split(|c: char| !c.is_alphanumeric() && c != '-')
            .any(|tok| tok == *w)
    })
}

pub fn validate_candidate(c: &LogLineCandidate) -> anyhow::Result<()> {
    if c.when.trim().is_empty() {
        anyhow::bail!("when required")
    }
    if forbidden_claim(&c.status) {
        anyhow::bail!("forbidden closure claim")
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unverified_not_verified() {
        assert!(!forbidden_claim("unverified"));
        assert!(forbidden_claim("verified"));
    }
}
#[cfg(test)]
mod more_tests {
    use super::*;
    #[test]
    fn validation_and_hash() {
        let c = LogLineCandidate {
            canon_version: "c".into(),
            schema_version: "s".into(),
            who: "w".into(),
            did: "d".into(),
            this: serde_json::json!({}),
            when: "2026".into(),
            confirmed_by: serde_json::json!({"evidence_state":"missing"}),
            if_ok: "ok".into(),
            if_doubt: "doubt".into(),
            if_not: "not".into(),
            status: "candidate".into(),
        };
        validate_candidate(&c).unwrap();
        assert!(!tuple_hash(&c).unwrap().is_empty());
    }
}
