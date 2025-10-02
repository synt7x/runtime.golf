use axum::{extract::State, Json};
use bollard::Docker;
use serde_json::json;
use std::sync::Arc;

use crate::runtime::format;

#[axum::debug_handler]
pub async fn callback(
    State(docker): State<Arc<Docker>>,
    Json(payload): Json<format::Request>
) -> Json<serde_json::Value> {
    return Json(json!({}));
}