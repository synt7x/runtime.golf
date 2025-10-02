use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::db::{auth, user::User};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub id: i64,
    pub username: String,
    pub admin: bool,
    pub github_id: i64,
    pub session_id: String,
    pub issued: usize,
    pub exp: usize,
}

pub fn create(claims: &Claims) -> Result<String, Box<dyn std::error::Error>> {
    let code: String =
        std::env::var("JWT_SECRET").expect("JWT_SECRET must be set in environment variables");

    let token = encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(code.as_ref()),
    )?;

    return Ok(token);
}

pub fn cookie(token: String) -> Cookie<'static> {
    return Cookie::build(("auth", token))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .secure(false)
        .build();
}

pub fn get(cookies: &CookieJar) -> Option<Claims> {
    if let Some(cookie) = cookies.get("auth") {
        let token = cookie.value();
        let code: String =
            std::env::var("JWT_SECRET").expect("JWT_SECRET must be set in environment variables");
        let key = DecodingKey::from_secret(code.as_ref());

        if let Ok(data) = decode::<Claims>(token, &key, &Validation::default()) {
            let claims = data.claims;
            return Some(claims);
        }
    }

    return None;
}

pub async fn session(cookies: &CookieJar, pool: &SqlitePool) -> Option<User> {
    if let Some(claims) = get(cookies) {
        if let Ok(Some(user)) = auth::session(&claims.session_id, &pool).await {
            return Some(User {
                id: user.id,
                github_id: user.github_id,
                username: user.username,
                admin: user.admin,
                session_id: user.session_id,
            });
        }
    }

    return None;
}
