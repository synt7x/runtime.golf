pub mod hole;
pub mod home;
pub mod sitemap;
pub mod user;

use crate::RenderState;
use axum::{Router, routing::get};

pub fn routes() -> Router<RenderState> {
    Router::new()
        .route("/", get(home::render))
        .route("/sitemap.xml", get(sitemap::render))
        .route("/holes/{id}", get(hole::render))
        .route("/users/{username}", get(user::render))
}
