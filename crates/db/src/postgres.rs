use agent::{RunRecord, RunStore, StoreError};
use async_trait::async_trait;
use schemas::{ApprovalRecord, EngineeringProposal, RunState};
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Row};
use std::path::Path;

pub async fn connect_postgres(database_url: &str) -> Result<PgPool, StoreError> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .map_err(|e| StoreError::Message(e.to_string()))
}

pub async fn migrate(pool: &PgPool, migrations_dir: impl AsRef<Path>) -> Result<(), StoreError> {
    let path = migrations_dir.as_ref().join("001_init.sql");
    let sql = std::fs::read_to_string(&path).map_err(|e| {
        StoreError::Message(format!("read migration {}: {e}", path.display()))
    })?;
    sqlx::raw_sql(&sql)
        .execute(pool)
        .await
        .map_err(|e| StoreError::Message(e.to_string()))?;
    tracing::info!("applied migrations from {}", path.display());
    Ok(())
}

#[derive(Clone)]
pub struct PostgresStore {
    pool: PgPool,
}

impl PostgresStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RunStore for PostgresStore {
    async fn upsert(&self, record: RunRecord) -> Result<(), StoreError> {
        let state = serde_json::to_value(record.state)
            .map_err(|e| StoreError::Message(e.to_string()))?;
        let state = state
            .as_str()
            .ok_or_else(|| StoreError::Message("state must serialize to string".into()))?
            .to_string();
        let proposal = record
            .proposal
            .as_ref()
            .map(serde_json::to_value)
            .transpose()
            .map_err(|e| StoreError::Message(e.to_string()))?;
        let approval = record
            .approval
            .as_ref()
            .map(serde_json::to_value)
            .transpose()
            .map_err(|e| StoreError::Message(e.to_string()))?;

        sqlx::query(
            r#"
            INSERT INTO runs (id, request, repo_path, state, proposal_json, approval_json, error, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())
            ON CONFLICT (id) DO UPDATE SET
              request = EXCLUDED.request,
              repo_path = EXCLUDED.repo_path,
              state = EXCLUDED.state,
              proposal_json = EXCLUDED.proposal_json,
              approval_json = EXCLUDED.approval_json,
              error = EXCLUDED.error,
              updated_at = NOW()
            "#,
        )
        .bind(&record.id)
        .bind(&record.request)
        .bind(&record.repo_path)
        .bind(&state)
        .bind(proposal)
        .bind(approval)
        .bind(&record.error)
        .execute(&self.pool)
        .await
        .map_err(|e| StoreError::Message(e.to_string()))?;
        Ok(())
    }

    async fn get(&self, id: &str) -> Result<Option<RunRecord>, StoreError> {
        let row = sqlx::query(
            r#"
            SELECT id, request, repo_path, state, proposal_json, approval_json, error
            FROM runs WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| StoreError::Message(e.to_string()))?;

        row.map(|r| row_to_record(r)).transpose()
    }

    async fn list(&self) -> Result<Vec<RunRecord>, StoreError> {
        let rows = sqlx::query(
            r#"
            SELECT id, request, repo_path, state, proposal_json, approval_json, error
            FROM runs
            ORDER BY updated_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| StoreError::Message(e.to_string()))?;

        rows.into_iter().map(row_to_record).collect()
    }
}

fn row_to_record(row: sqlx::postgres::PgRow) -> Result<RunRecord, StoreError> {
    let state_raw: String = row
        .try_get("state")
        .map_err(|e| StoreError::Message(e.to_string()))?;
    let state: RunState = serde_json::from_value(serde_json::Value::String(state_raw))
        .map_err(|e| StoreError::Message(e.to_string()))?;

    let proposal_json: Option<serde_json::Value> = row
        .try_get("proposal_json")
        .map_err(|e| StoreError::Message(e.to_string()))?;
    let approval_json: Option<serde_json::Value> = row
        .try_get("approval_json")
        .map_err(|e| StoreError::Message(e.to_string()))?;

    let proposal = proposal_json
        .map(serde_json::from_value::<EngineeringProposal>)
        .transpose()
        .map_err(|e| StoreError::Message(e.to_string()))?;
    let approval = approval_json
        .map(serde_json::from_value::<ApprovalRecord>)
        .transpose()
        .map_err(|e| StoreError::Message(e.to_string()))?;

    Ok(RunRecord {
        id: row
            .try_get("id")
            .map_err(|e| StoreError::Message(e.to_string()))?,
        request: row
            .try_get("request")
            .map_err(|e| StoreError::Message(e.to_string()))?,
        repo_path: row
            .try_get("repo_path")
            .map_err(|e| StoreError::Message(e.to_string()))?,
        state,
        proposal,
        approval,
        error: row
            .try_get("error")
            .map_err(|e| StoreError::Message(e.to_string()))?,
    })
}
