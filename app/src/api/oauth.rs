use axum::{
    Router,
    extract::{Query, State},
    http::StatusCode,
    response::Redirect,
    routing::get,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Debug, Deserialize)]
pub struct AuthCallback {
    code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubUser {
    pub id: i64,
    pub login: String,
    pub name: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenResponse {
    access_token: String,
    token_type: String,
    scope: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct User {
    pub id: i64,
    pub github_id: i64,
    pub username: String,
    pub admin: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    username: String,
    admin: bool,
    issued: usize,
}

#[utoipa::path(
    get,
    path = "/api/github",
    responses(
        (status = 302, description = "Redirect to GitHub OAuth")
    ),
    tag = "github"
)]
pub async fn github_auth() -> Result<axum::response::Redirect, StatusCode> {
    let client_id = std::env::var("CLIENT_ID").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let redirect_uri = urlencoding::encode("http://127.0.0.1:3000/api/github/callback");
    let scope = urlencoding::encode("user:email");

    let auth_url = format!(
        "https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}&scope={}",
        client_id, redirect_uri, scope
    );

    Ok(axum::response::Redirect::temporary(&auth_url))
}

#[utoipa::path(
    get,
    path = "/api/github/callback",
    params(
        ("code" = String, Query, description = "GitHub OAuth code"),
        ("state" = Option<String>, Query, description = "State parameter")
    ),
    responses(
        (status = 302, description = "Redirect to home page with auth cookie"),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "github"
)]
pub async fn github_callback(
    Query(params): Query<AuthCallback>,
    State(pool): State<SqlitePool>,
) -> Result<impl axum::response::IntoResponse, StatusCode> {
    let client_id = std::env::var("CLIENT_ID").map_err(|e| {
        println!("CLIENT_ID not found: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    let client_secret = std::env::var("CLIENT_SECRET").map_err(|e| {
        println!("CLIENT_SECRET not found: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let client = reqwest::Client::new();
    let token_url = "https://github.com/login/oauth/access_token";

    let mut token_params = HashMap::new();
    token_params.insert("client_id", client_id);
    token_params.insert("client_secret", client_secret);
    token_params.insert("code", params.code);

    let token_response = client
        .post(token_url)
        .header("Accept", "application/json")
        .form(&token_params)
        .send()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let token_data: AccessTokenResponse = token_response
        .json()
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let user_response = client
        .get("https://api.github.com/user")
        .header(
            "Authorization",
            format!("Bearer {}", token_data.access_token),
        )
        .header("User-Agent", "runtime-golf")
        .send()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let github_user: GitHubUser = user_response
        .json()
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let user = sqlx::query!(
        r#"
        INSERT INTO users (github_id, username, admin)
        VALUES (?1, ?2, FALSE)
        ON CONFLICT(github_id) DO UPDATE SET username = excluded.username
        RETURNING id, github_id, username, admin
        "#,
        github_user.id,
        github_user.login
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let now = chrono::Utc::now().timestamp() as usize;
    let claims = Claims {
        sub: user.id.to_string(),
        username: user.username,
        admin: user.admin,
        issued: now,
    };

    let jwt_secret = std::env::var("JWT_SECRET").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let cookie = Cookie::build(("auth", token))
        .http_only(true)
        .secure(false)
        .same_site(SameSite::Lax)
        .path("/")
        .build();

    Ok((
        axum_extra::extract::cookie::CookieJar::new().add(cookie),
        Redirect::to("/"),
    ))
}

pub fn routes() -> Router<SqlitePool> {
    Router::new()
        .route("/api/github", get(github_auth))
        .route("/api/github/callback", get(github_callback))
}
