use std::collections::HashMap;

use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AccessToken {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
}

#[derive(Serialize, Deserialize)]
pub struct GithubUser {
    pub id: i64,
    pub login: String,
}

pub async fn access_token(code: String) -> Result<AccessToken, Box<dyn std::error::Error>> {
    let id = std::env::var("CLIENT_ID")?;
    let secret = std::env::var("CLIENT_SECRET")?;

    let client = Client::new();
    let url = "https://github.com/login/oauth/access_token";

    let mut params = HashMap::new();
    params.insert("client_id", id);
    params.insert("client_secret", secret);
    params.insert("code", code);

    let response = client
        .post(url)
        .header("Accept", "application/json")
        .form(&params)
        .send()
        .await?;
    let token: AccessToken = response.json().await?;

    return Ok(token);
}

pub async fn user(token: AccessToken) -> Result<GithubUser, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "https://api.github.com/user";

    let response = client
        .get(url)
        .header("Authorization", format!("Bearer {}", token.access_token))
        .header("User-Agent", "runtime-golf")
        .send()
        .await?;

    let user: GithubUser = response.json().await?;

    return Ok(user);
}
