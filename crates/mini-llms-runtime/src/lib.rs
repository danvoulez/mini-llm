use async_trait::async_trait;
use mini_llms_core::{validate_candidate, ExtractionStatus, LogLineCandidate, RuntimeObservation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceRequest {
    pub prompt: String,
    pub model: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceResponse {
    pub observation: RuntimeObservation,
    pub candidate: Option<LogLineCandidate>,
}

#[async_trait]
pub trait LocalInference {
    async fn infer(&self, req: &InferenceRequest) -> anyhow::Result<InferenceResponse>;
}

pub struct DeterministicMockRuntime;
#[async_trait]
impl LocalInference for DeterministicMockRuntime {
    async fn infer(&self, req: &InferenceRequest) -> anyhow::Result<InferenceResponse> {
        let c = LogLineCandidate {
            canon_version: "logline-canon@unknown".into(),
            schema_version: "mini-llms.logline-candidate.v0".into(),
            who: "lab512:mistral".into(),
            did: "propose".into(),
            this: serde_json::json!({"prompt":req.prompt}),
            when: "2026-05-26T00:00:00Z".into(),
            confirmed_by: serde_json::json!({"evidence_state":"missing","note":"model generation is not evidence"}),
            if_ok: "route after evidence".into(),
            if_doubt: "create ghost".into(),
            if_not: "reject".into(),
            status: "candidate".into(),
        };
        validate_candidate(&c)?;
        Ok(InferenceResponse {
            observation: RuntimeObservation {
                run_id: "mock-run".into(),
                profile_id: None,
                runtime: "mock".into(),
                model: req.model.clone(),
                lab_id: None,
                provider_shape: "mock.content".into(),
                content_present: true,
                reasoning_content_present: false,
                extraction_status: ExtractionStatus::ContentExtracted,
                logline_parse_status: Some("ok".into()),
                guard_status: Some("ok".into()),
                schema_validation_status: Some("ok".into()),
                candidate_emitted: true,
                derived_projection_emitted: false,
                raw_response: None,
                raw_response_redacted: false,
                error_kind: None,
                error_message: None,
            },
            candidate: Some(c),
        })
    }
}
