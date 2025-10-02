use axum::{extract::State, Json};
use bollard::Docker;
use serde_json::json;
use std::sync::Arc;

pub async fn callback(
    State(docker): State<Arc<Docker>>
) -> Json<serde_json::Value> {
    let version = docker.version().await;

    return match version {
        Ok(info) => Json(json!({
            "success": true, "docker": info.version.unwrap_or("unknown".to_owned())
        })),
        Err(err) => Json(json!({
            "success": false, "message": err.to_string()
        }))
    }
}