use sqlx::{SqlitePool, query};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{db::user::User, tools::github::GithubUser};

pub async fn login(
    user: GithubUser,
    pool: &SqlitePool,
) -> Result<User, Box<dyn std::error::Error>> {
    let session = Uuid::new_v4().to_string();
    let result = query!(
        r#"
        INSERT INTO users (github_id, username, admin, session_id)
        VALUES (?1, ?2, FALSE, ?3)
        ON CONFLICT(github_id) DO UPDATE SET 
            username = excluded.username,
            session_id = excluded.session_id
        RETURNING id, github_id, username, admin, session_id
        "#,
        user.id,
        user.login,
        session
    )
    .fetch_one(pool)
    .await?;

    return Ok(User {
        id: result.id,
        github_id: result.github_id,
        username: result.username,
        admin: result.admin,
        session_id: result.session_id,
    });
}

pub async fn session(
    session_id: &str,
    pool: &SqlitePool,
) -> Result<Option<User>, Box<dyn std::error::Error>> {
    let result = query!(
        r#"
        SELECT id, github_id, username, admin, session_id
        FROM users
        WHERE session_id = ?1
        "#,
        session_id
    )
    .fetch_optional(pool)
    .await?;

    if let Some(user) = result {
        return Ok(Some(User {
            id: user.id.unwrap_or(0),
            github_id: user.github_id,
            username: user.username,
            admin: user.admin,
            session_id: user.session_id,
        }));
    }

    Ok(None)
}
