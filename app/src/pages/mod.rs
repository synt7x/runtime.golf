pub mod hole;
pub mod home;

use axum::{Router, routing::get};
use sqlx::SqlitePool;

pub fn routes() -> Router<SqlitePool> {
    Router::new()
        .route("/", get(home::render))
        .route("/hole/{id}", get(hole::render))
}
