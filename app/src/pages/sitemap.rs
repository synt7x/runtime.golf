use crate::tools::holes;
use axum::response::IntoResponse;

pub async fn render() -> impl IntoResponse {
    let holes = holes::get();
    let mut urls = String::new();
    urls.push_str("<url><loc>https://runtime.golf/</loc><priority>1.0</priority></url>");

    for (slug, _) in holes.iter() {
        urls.push_str(&format!(
            "<url><loc>https://runtime.golf/holes/{}</loc><priority>0.8</priority></url>",
            slug
        ));
    }

    let xml = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?><urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">{}</urlset>",
        urls
    );

    return ([("Content-Type", "application/xml")], xml);
}
