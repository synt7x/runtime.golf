use axum::{extract::State, response::Html};
use axum_extra::extract::CookieJar;
use chrono::Datelike;
use serde_json::json;

use crate::{
    RenderState,
    tools::{holes, jwt, templates},
};

pub async fn render(State(state): State<RenderState>, cookies: CookieJar) -> Html<String> {
    let session = jwt::session(&cookies, &state.pool).await;
    let data = match session {
        Some(user) => &json!({
            "year": chrono::Utc::now().year() - 1,
            "user": {
                "id": user.id,
                "github_id": user.github_id,
                "username": user.username,
                "admin": user.admin,
            },
            "holes": holes::get(),
        }),
        None => &json!({
            "year": chrono::Utc::now().year(),
            "holes": holes::get(),
        }),
    };

    match templates::render(&state.handlebars, "index", data) {
        Ok(content) => Html(content),
        Err(e) => {
            eprint!("Error rendering template: {}", e);
            Html("<h1>Error rendering page</h1>".to_string())
        }
    }
}
