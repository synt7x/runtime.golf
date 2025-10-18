use axum::{
    extract::{Path, State},
    response::Html,
};
use axum_extra::extract::CookieJar;
use serde_json::json;

use crate::{
    RenderState,
    tools::{jwt, templates},
};

pub async fn render(State(state): State<RenderState>, cookies: CookieJar) -> Html<String> {
    let session = jwt::session(&cookies, &state.pool).await;
    let data = match session {
        Some(user) => &json!({
            "user": {
                "id": user.id,
                "github_id": user.github_id,
                "username": user.username,
                "admin": user.admin,
            },
        }),
        None => &json!({}),
    };

    match templates::render(&state.handlebars, "leaderboards", data) {
        Ok(content) => Html(content),
        Err(e) => {
            eprint!("Error rendering template: {}", e);
            Html("<h1>Error rendering page</h1>".to_string())
        }
    }
}
