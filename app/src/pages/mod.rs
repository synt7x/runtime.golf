mod about;
pub mod hole;
pub mod home;
mod leaderboards;
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
        .route("/leaderboards", get(leaderboards::render))
        .route("/about", get(about::render))
}
