use std::sync::Arc;

use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Json;
use axum::Router;

use crate::app::App;
use crate::network::app_error::AppError;

// --- Cluster management
/*
pub fn rest(app: &mut Server) {
    let mut cluster = app.at("/cluster");
    cluster.at("/add-learner").post(add_learner);
    cluster.at("/change-membership").post(change_membership);
    cluster.at("/init").post(init);
    cluster.at("/metrics").get(metrics);
}
*/

pub fn rest() -> Router<Arc<App>> {
    Router::new().nest("/cluster", Router::new().route("/metrics", get(metrics)))
}

/// Get the latest metrics of the cluster
async fn metrics(State(state): State<Arc<App>>) -> Result<impl IntoResponse, AppError> {
    let metrics = state.raft.metrics().borrow().clone();
    Ok(Json(metrics))
}
