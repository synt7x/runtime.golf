pub mod docs;
pub mod oauth;

use crate::RenderState;
use axum::Router;

pub fn routes() -> Router<RenderState> {
    Router::new().merge(oauth::routes())
}
