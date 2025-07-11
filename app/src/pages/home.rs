use axum::response::Html;
use chrono::Datelike;
use serde_json::json;

use crate::tools::templates;

const TEMPLATE: &str = "../templates/index.hbs";

pub async fn render() -> Html<String> {
    let rendered = templates::render(
        TEMPLATE,
        &json!({
            "year": chrono::Utc::now().year(),
        }),
    );

    match rendered {
        Ok(content) => Html(content),
        Err(e) => {
            eprint!("Error rendering template: {}", e);
            Html("<h1>Error rendering page</h1>".to_string())
        }
    }
}
