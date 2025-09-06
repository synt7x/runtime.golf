use axum::{extract::State, response::Html};
use axum_extra::extract::CookieJar;
use chrono::Datelike;
use serde_json::json;
use sqlx::SqlitePool;

use crate::tools::{holes, jwt, templates};

const TEMPLATE: &str = "../templates/index.hbs";

pub async fn render(State(pool): State<SqlitePool>, cookies: CookieJar) -> Html<String> {
    let session = jwt::session(&cookies, &pool).await;
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

    match templates::render(TEMPLATE, data) {
        Ok(content) => Html(content),
        Err(e) => {
            eprint!("Error rendering template: {}", e);
            Html("<h1>Error rendering page</h1>".to_string())
        }
    }
}
