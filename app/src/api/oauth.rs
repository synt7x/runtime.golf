use axum::{
    Router,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::get,
};
use axum_extra::extract::CookieJar;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::{
    db,
    tools::{
        github,
        jwt::{self, Claims},
    },
};

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

#[utoipa::path(
    get,
    path = "/api/github",
    responses(
        (status = 302, description = "Redirect to GitHub OAuth")
    ),
    tag = "github"
)]
pub async fn github(
    State(pool): State<SqlitePool>,
    cookies: CookieJar,
) -> Result<impl IntoResponse, StatusCode> {
    let client_id = std::env::var("CLIENT_ID").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let redirect_uri = urlencoding::encode("http://127.0.0.1/api/github/callback");
    let scope = urlencoding::encode("read:user");

    if jwt::session(&cookies, &pool).await.is_some() {
        return Ok((cookies, Redirect::to("/")));
    }

    let auth_url = format!(
        "https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}&scope={}",
        client_id, redirect_uri, scope
    );

    Ok((cookies, Redirect::temporary(&auth_url)))
}

#[utoipa::path(
    get,
    path = "/api/github/callback",
    params(
        ("code" = String, Query, description = "GitHub OAuth code"),
    ),
    responses(
        (status = 302, description = "Authentication success"),
        (status = 500, description = "Authentication failure")
    ),
    tag = "github"
)]
pub async fn callback(
    Query(params): Query<AuthCallback>,
    State(pool): State<SqlitePool>,
    cookies: CookieJar,
) -> Result<impl IntoResponse, StatusCode> {
    println!("Received GitHub OAuth code: {}", params.code);
    if jwt::session(&cookies, &pool).await.is_some() {
        return Ok((cookies, Redirect::to("/")));
    }

    let token = github::access_token(params.code)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let github_user = github::user(token)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = db::auth::login(github_user, &pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let claims = Claims {
        id: user.id,
        github_id: user.github_id,
        username: user.username,
        admin: user.admin,
        session_id: user.session_id,
        issued: Utc::now().timestamp() as usize,
        exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
    };

    let token = jwt::create(&claims).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let cookie = jwt::cookie(token);

    Ok((cookies.add(cookie), Redirect::to("/")))
}

pub fn routes() -> Router<SqlitePool> {
    Router::new()
        .route("/api/github", get(github))
        .route("/api/github/callback", get(callback))
}
