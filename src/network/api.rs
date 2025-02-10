use core::convert::Into;
use std::sync::Arc;

use anyhow::anyhow;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::routing::post;
use axum::Json;
use axum::Router;

use crate::app::App;
use crate::network::app_error::AppError;
use crate::Request;

pub fn rest() -> Router<Arc<App>> {
    Router::new().nest(
        "/api",
        Router::new()
            .route("/write", post(write))
            .route("/read", get(read))
            .route("/linearizable_read", get(linearizable_read)),
    )
}

async fn write(State(state): State<Arc<App>>, Json(input): Json<Request>) -> Result<impl IntoResponse, AppError> {
    let res = state.raft.client_write(input).await?;
    Ok(Json(res))
}

async fn read(State(state): State<Arc<App>>, Json(key): Json<String>) -> Result<impl IntoResponse, AppError> {
    let kvs = state.key_values.read().await;
    match kvs.get(&key) {
        Some(v) => Ok(v.clone()),
        None => Err(anyhow!("not found").into()),
    }
}

async fn linearizable_read(
    State(state): State<Arc<App>>,
    Json(key): Json<String>,
) -> Result<impl IntoResponse, AppError> {
    let _ = state.raft.ensure_linearizable().await?;
    let kvs = state.key_values.read().await;
    match kvs.get(&key) {
        Some(v) => Ok(v.clone()),
        None => Err(anyhow!("not found").into()),
    }
}
