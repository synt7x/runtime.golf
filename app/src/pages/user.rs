use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::Html,
};
use axum_extra::extract::CookieJar;
use serde_json::json;
use sqlx::SqlitePool;

use crate::{
    RenderState,
    db::user,
    tools::{jwt, templates},
};

#[axum::debug_handler]
pub async fn render(
    Path(username): Path<String>,
    State(state): State<RenderState>,
    cookies: CookieJar,
) -> Html<String> {
    let info = user::info(&username, &state.pool).await;
    if info.is_err() || info.as_ref().unwrap().is_none() {
        return Html("<h1>User not found</h1>".to_string());
    }

    let profile = info.unwrap().unwrap();

    let session = jwt::session(&cookies, &state.pool).await;
    let data = match session {
        Some(user) => &json!({
            "user": {
                "id": user.id,
                "github_id": user.github_id,
                "username": user.username,
                "admin": user.admin,
            },
            "profile": {
                "id": profile.id,
                "github_id": profile.github_id,
                "username": profile.username,
                "admin": profile.admin,
            },
        }),
        None => &json!({
            "profile": {
                "id": profile.id,
                "github_id": profile.github_id,
                "username": profile.username,
                "admin": profile.admin,
            },
        }),
    };

    match templates::render(&state.handlebars, "user", data) {
        Ok(content) => Html(content),
        Err(e) => {
            eprint!("Error rendering template: {}", e);
            Html("<h1>Error rendering page</h1>".to_string())
        }
    }
}
