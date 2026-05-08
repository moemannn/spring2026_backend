use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::users::get_users,
        crate::handlers::users::get_user,
        crate::handlers::users::add_user,
        crate::handlers::users::edit_user,
        crate::handlers::users::delete_user,

        crate::handlers::servers::get_servers,
        crate::handlers::servers::get_server,
        crate::handlers::servers::add_server,
        crate::handlers::servers::edit_server,
        crate::handlers::servers::delete_server,
    )
)]
pub struct ApiDoc;

pub fn get_swagger_routes() -> Router {
    Router::new().merge(
        SwaggerUi::new("/swagger-ui")
            .url("/api-docs/openapi.json", ApiDoc::openapi()),
    )
}