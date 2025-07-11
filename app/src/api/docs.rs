use utoipa::OpenApi;

use super::oauth::User;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::oauth::github_auth,
        crate::api::oauth::github_callback,
    ),
    components(schemas(User)),
    tags(
        (name = "github", description = "Github authentication endpoints"),
        (name = "users", description = "User management endpoints")
    )
)]
pub struct ApiDoc;
