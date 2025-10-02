use sqlx::{SqlitePool, query};
use utoipa::ToSchema;

#[derive(ToSchema)]
pub struct User {
    pub id: i64,
    pub github_id: i64,
    pub username: String,
    pub admin: bool,
    pub session_id: String,
}

pub async fn info(
    username: &str,
    pool: &SqlitePool,
) -> Result<Option<User>, Box<dyn std::error::Error + Send + Sync>> {
    let result = query!(
        r#"
        SELECT id, github_id, username, admin, session_id
        FROM users
        WHERE username = ?1
        "#,
        username
    )
    .fetch_optional(pool)
    .await?;

    if let Some(user) = result {
        return Ok(Some(User {
            id: user.id,
            github_id: user.github_id,
            username: user.username,
            admin: user.admin,
            session_id: user.session_id,
        }));
    }

    Ok(None)
}
