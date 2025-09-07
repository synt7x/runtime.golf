pub mod hole;
pub mod home;
pub mod user;

use crate::RenderState;
use axum::{Router, routing::get};

pub fn routes() -> Router<RenderState> {
    Router::new()
        .route("/", get(home::render))
        .route("/hole/{id}", get(hole::render))
        .route("/user/{username}", get(user::render))
}
