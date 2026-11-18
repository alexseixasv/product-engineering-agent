use agent::{AgentPipeline, InMemoryStore, RunStore};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use db::{connect_postgres, migrate, PostgresStore};
use llm::MockLlmProvider;
use schemas::ApprovalDecision;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing_subscriber::EnvFilter;

#[derive(Clone)]
struct AppState {
    pipeline: Arc<AgentPipeline>,
    default_repo: PathBuf,
    db_enabled: bool,
}

#[derive(Debug, Deserialize)]
struct CreateRunRequest {
    request: String,
    #[serde(default)]
    repo_path: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DecisionRequest {
    decision: ApprovalDecision,
    actor: String,
    #[serde(default)]
    comment: Option<String>,
}

#[derive(Debug, Serialize)]
struct ErrorBody {
    error: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();

    let default_repo = std::env::var("PEA_REPO_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("fixtures/sample-app"));
    let artifacts = std::env::var("PEA_ARTIFACTS_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("artifacts"));
    let migrations_dir = std::env::var("PEA_MIGRATIONS_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("migrations"));

    let llm: Arc<dyn llm::LlmProvider> = Arc::new(MockLlmProvider::new(vec![]));

    let (store, db_enabled): (Arc<dyn RunStore>, bool) =
        if let Ok(database_url) = std::env::var("DATABASE_URL") {
            let pool = connect_postgres(&database_url).await?;
            migrate(&pool, &migrations_dir).await?;
            tracing::info!("using Postgres run store");
            (Arc::new(PostgresStore::new(pool)), true)
        } else {
            tracing::warn!("DATABASE_URL not set; using in-memory run store");
            (Arc::new(InMemoryStore::new()), false)
        };

    let pipeline = Arc::new(AgentPipeline::with_store(llm, artifacts, store));

    let state = AppState {
        pipeline,
        default_repo,
        db_enabled,
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/runs", post(create_run).get(list_runs))
        .route("/api/runs/{id}", get(get_run))
        .route("/api/runs/{id}/decision", post(decide))
        .route("/api/runs/{id}/finalize", post(finalize))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = std::env::var("PEA_BIND").unwrap_or_else(|_| "127.0.0.1:8080".into());
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("listening on {addr}");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health(State(state): State<AppState>) -> impl IntoResponse {
    Json(serde_json::json!({
        "ok": true,
        "db": if state.db_enabled { "postgres" } else { "memory" }
    }))
}

async fn create_run(
    State(state): State<AppState>,
    Json(body): Json<CreateRunRequest>,
) -> impl IntoResponse {
    let repo = body
        .repo_path
        .map(PathBuf::from)
        .unwrap_or_else(|| state.default_repo.clone());
    match state.pipeline.start_run(body.request, repo).await {
        Ok(run) => (
            StatusCode::CREATED,
            Json(serde_json::to_value(run_json(&run)).unwrap()),
        )
            .into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(ErrorBody {
                error: e.to_string(),
            }),
        )
            .into_response(),
    }
}

async fn list_runs(State(state): State<AppState>) -> impl IntoResponse {
    match state.pipeline.store.list().await {
        Ok(runs) => Json(
            runs.into_iter()
                .map(|r| run_json(&r))
                .collect::<Vec<_>>(),
        )
        .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorBody {
                error: e.to_string(),
            }),
        )
            .into_response(),
    }
}

async fn get_run(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    match state.pipeline.store.get(&id).await {
        Ok(Some(run)) => Json(run_json(&run)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ErrorBody {
                error: format!("run {id} not found"),
            }),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorBody {
                error: e.to_string(),
            }),
        )
            .into_response(),
    }
}

async fn decide(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(body): Json<DecisionRequest>,
) -> impl IntoResponse {
    match state
        .pipeline
        .decide(&id, body.decision, &body.actor, body.comment)
        .await
    {
        Ok(run) => Json(run_json(&run)).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(ErrorBody {
                error: e.to_string(),
            }),
        )
            .into_response(),
    }
}

async fn finalize(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    match state.pipeline.generate_design_and_snapshot(&id).await {
        Ok(run) => Json(run_json(&run)).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(ErrorBody {
                error: e.to_string(),
            }),
        )
            .into_response(),
    }
}

fn run_json(run: &agent::RunRecord) -> serde_json::Value {
    serde_json::json!({
        "id": run.id,
        "request": run.request,
        "repo_path": run.repo_path,
        "state": run.state,
        "proposal": run.proposal,
        "approval": run.approval,
        "error": run.error,
    })
}
