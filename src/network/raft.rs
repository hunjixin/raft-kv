use std::sync::Arc;

use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::Json;
use axum::Router;
use openraft::raft::AppendEntriesRequest;
use openraft::raft::InstallSnapshotRequest;
use openraft::raft::VoteRequest;

use crate::app::App;
use crate::network::app_error::AppError;
use crate::TypeConfig;

// --- Raft communication

pub fn rest() -> Router<Arc<App>> {
    Router::new().nest(
        "/raft",
        Router::new()
            .route("/raft-vote", post(vote))
            .route("/raft-append", post(append))
            .route("/raft-snapshot", post(snapshot)),
    )
}

pub async fn vote(
    State(state): State<Arc<App>>,
    Json(req): Json<VoteRequest<TypeConfig>>,
) -> Result<impl IntoResponse, AppError> {
    let res = state.raft.vote(req).await;
    Ok(Json(res))
}

pub async fn append(
    State(state): State<Arc<App>>,
    Json(req): Json<AppendEntriesRequest<TypeConfig>>,
) -> Result<impl IntoResponse, AppError> {
    let res = state.raft.append_entries(req).await;
    Ok(Json(res))
}

pub async fn snapshot(
    State(state): State<Arc<App>>,
    Json(req): Json<InstallSnapshotRequest<TypeConfig>>,
) -> Result<impl IntoResponse, AppError> {
    let res = state.raft.install_snapshot(req).await;
    Ok(Json(res))
}
