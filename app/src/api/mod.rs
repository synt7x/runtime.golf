pub mod docs;
pub mod oauth;

use axum::Router;
use sqlx::SqlitePool;

pub fn routes() -> Router<SqlitePool> {
    Router::new().merge(oauth::routes())
}
