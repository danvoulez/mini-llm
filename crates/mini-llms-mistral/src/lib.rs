use async_trait::async_trait;
use mini_llms_core::{validate_candidate, ExtractionStatus, LogLineCandidate, RuntimeObservation};
use mini_llms_runtime::{InferenceRequest, InferenceResponse, LocalInference};

pub struct MistralAdapter {
    pub base_url: String,
    pub api_key: Option<String>,
    client: reqwest::Client,
}
impl MistralAdapter {
    pub fn new(base_url: String, api_key: Option<String>) -> Self {
        Self {
            base_url,
            api_key,
            client: reqwest::Client::new(),
        }
    }
    pub async fn doctor_models(&self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.get(format!("{}/v1/models", self.base_url));
        if let Some(k) = &self.api_key {
            r = r.bearer_auth(k);
        }
        Ok(r.send().await?.json().await?)
    }
}

#[async_trait]
impl LocalInference for MistralAdapter {
    async fn infer(&self, req: &InferenceRequest) -> anyhow::Result<InferenceResponse> {
        let body = serde_json::json!({"model":req.model.clone().unwrap_or("mistral".into()),"messages":[{"role":"user","content":req.prompt}],"response_format":{"type":"json_object"}});
        let mut r = self
            .client
            .post(format!("{}/v1/chat/completions", self.base_url))
            .json(&body);
        if let Some(k) = &self.api_key {
            r = r.bearer_auth(k);
        }
        let v: serde_json::Value = r.send().await?.json().await?;
        let msg = &v["choices"][0]["message"];
        let content = msg["content"].as_str().unwrap_or("");
        let reasoning = msg["reasoning_content"].as_str().unwrap_or("");
        let (status, candidate) = if !content.trim().is_empty() {
            match serde_json::from_str::<LogLineCandidate>(content) {
                Ok(c) => {
                    validate_candidate(&c)?;
                    (ExtractionStatus::ContentExtracted, Some(c))
                }
                Err(_) => (ExtractionStatus::ParseFailed, None),
            }
        } else if !reasoning.trim().is_empty() {
            (ExtractionStatus::ReasoningOnlyNoContent, None)
        } else {
            (ExtractionStatus::EmptyCompletion, None)
        };
        Ok(InferenceResponse {
            observation: RuntimeObservation {
                run_id: "mistral-run".into(),
                profile_id: None,
                runtime: "lab512".into(),
                model: req.model.clone(),
                lab_id: Some("lab512".into()),
                provider_shape: "openai.chat.completions".into(),
                content_present: !content.is_empty(),
                reasoning_content_present: !reasoning.is_empty(),
                extraction_status: status,
                logline_parse_status: None,
                guard_status: None,
                schema_validation_status: None,
                candidate_emitted: candidate.is_some(),
                derived_projection_emitted: false,
                raw_response: Some(v),
                raw_response_redacted: false,
                error_kind: None,
                error_message: None,
            },
            candidate,
        })
    }
}
