use axum::{
    extract::{Path, State},
    response::Html,
};
use axum_extra::extract::CookieJar;
use serde_json::json;

use crate::{
    RenderState,
    tools::{holes, jwt, templates},
};

pub async fn render(
    Path(id): Path<String>,
    State(state): State<RenderState>,
    cookies: CookieJar,
) -> Html<String> {
    let holes = holes::get();
    if !holes.contains_key(&id) {
        return Html("<h1>Hole not found</h1>".to_string());
    }

    let session = jwt::session(&cookies, &state.pool).await;
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

    match templates::render(&state.handlebars, "hole", data) {
        Ok(content) => Html(content),
        Err(e) => {
            eprint!("Error rendering template: {}", e);
            Html("<h1>Error rendering page</h1>".to_string())
        }
    }
}
