use async_trait::async_trait;
use mini_llms_core::{
    content_hash, tuple_hash, Finding, GhostCandidate, LogLineCandidate, RuntimeObservation,
};
use sqlx::PgPool;

pub struct StoredRef {
    pub id: String,
}
#[async_trait]
pub trait CandidateStore {
    async fn append_runtime_observation(
        &self,
        obs: &RuntimeObservation,
    ) -> anyhow::Result<StoredRef>;
    async fn append_logline_candidate(
        &self,
        candidate: &LogLineCandidate,
    ) -> anyhow::Result<StoredRef>;
    async fn append_finding(&self, finding: &Finding) -> anyhow::Result<StoredRef>;
    async fn append_ghost(&self, ghost: &GhostCandidate) -> anyhow::Result<StoredRef>;
}

pub struct NullCandidateStore;
#[async_trait]
impl CandidateStore for NullCandidateStore {
    async fn append_runtime_observation(
        &self,
        _: &RuntimeObservation,
    ) -> anyhow::Result<StoredRef> {
        Ok(StoredRef { id: "null".into() })
    }
    async fn append_logline_candidate(&self, _: &LogLineCandidate) -> anyhow::Result<StoredRef> {
        Ok(StoredRef { id: "null".into() })
    }
    async fn append_finding(&self, _: &Finding) -> anyhow::Result<StoredRef> {
        Ok(StoredRef { id: "null".into() })
    }
    async fn append_ghost(&self, _: &GhostCandidate) -> anyhow::Result<StoredRef> {
        Ok(StoredRef { id: "null".into() })
    }
}

pub struct PostgresCandidateStore {
    pub pool: PgPool,
}
#[async_trait]
impl CandidateStore for PostgresCandidateStore {
    async fn append_runtime_observation(
        &self,
        obs: &RuntimeObservation,
    ) -> anyhow::Result<StoredRef> {
        let id = uuid::Uuid::new_v4();
        sqlx::query("insert into mini_llms.runtime_observations (id,run_id,runtime,model,provider_shape,extraction_status,raw_response) values ($1,$2,$3,$4,$5,$6,$7)").bind(id).bind(&obs.run_id).bind(&obs.runtime).bind(&obs.model).bind(&obs.provider_shape).bind(format!("{:?}",obs.extraction_status)).bind(&obs.raw_response).execute(&self.pool).await?;
        Ok(StoredRef { id: id.to_string() })
    }
    async fn append_logline_candidate(&self, c: &LogLineCandidate) -> anyhow::Result<StoredRef> {
        let id = uuid::Uuid::new_v4();
        let th = tuple_hash(c)?;
        let ch = content_hash(&serde_json::to_value(c)?)?;
        sqlx::query("insert into mini_llms.logline_candidates (id,tuple_hash,content_hash,canon_version,schema_version,who,did,this,\"when\",confirmed_by,if_ok,if_doubt,if_not,status) values ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14)").bind(id).bind(th).bind(ch).bind(&c.canon_version).bind(&c.schema_version).bind(&c.who).bind(&c.did).bind(&c.this).bind(&c.when).bind(&c.confirmed_by).bind(&c.if_ok).bind(&c.if_doubt).bind(&c.if_not).bind(&c.status).execute(&self.pool).await?;
        Ok(StoredRef { id: id.to_string() })
    }
    async fn append_finding(&self, _: &Finding) -> anyhow::Result<StoredRef> {
        Ok(StoredRef {
            id: uuid::Uuid::new_v4().to_string(),
        })
    }
    async fn append_ghost(&self, _: &GhostCandidate) -> anyhow::Result<StoredRef> {
        Ok(StoredRef {
            id: uuid::Uuid::new_v4().to_string(),
        })
    }
}
