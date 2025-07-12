use utoipa::OpenApi;

use crate::db::auth::User;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::oauth::github,
        crate::api::oauth::callback,
    ),
    components(schemas(User)),
    tags(
        (name = "github", description = "Github authentication endpoints"),
        (name = "users", description = "User management endpoints")
    )
)]
pub struct ApiDoc;
