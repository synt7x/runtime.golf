use axum::{
    extract::{Path, State},
    response::Html,
};
use axum_extra::extract::CookieJar;
use serde_json::json;
use sqlx::SqlitePool;

use crate::tools::{holes, jwt, templates};

const TEMPLATE: &str = "../templates/hole.hbs";

pub async fn render(
    Path(id): Path<String>,
    State(pool): State<SqlitePool>,
    cookies: CookieJar,
) -> Html<String> {
    let holes = holes::get();
    if !holes.contains_key(&id) {
        return Html("<h1>Hole not found</h1>".to_string());
    }

    let session = jwt::session(&cookies, &pool).await;
    let data = match session {
        Some(user) => &json!({
            "user": {
                "id": user.id,
                "github_id": user.github_id,
                "username": user.username,
                "admin": user.admin,
            },
            "hole": holes.get(&id).unwrap(),
        }),
        None => &json!({
            "holes": holes.get(&id).unwrap(),
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
