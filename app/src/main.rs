use std::net::SocketAddr;

mod api;
mod db;
mod pages;
mod tools;

use axum::Router;
use sqlx::sqlite::SqlitePoolOptions;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{api::docs::ApiDoc, tools::holes};

#[tokio::main]
async fn main() {
    dotenvy::from_filename("../.env").ok();
    holes::load();

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:../data/runtime_golf.db")
        .await
        .expect("Failed to connect to the database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(api::routes())
        .merge(pages::routes())
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);
    println!("Swagger UI available at http://{}/swagger-ui/", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
