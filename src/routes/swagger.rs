use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::users::get_users_by_page,
        crate::handlers::users::get_user_by_id,
        crate::handlers::users::create_user,
        crate::handlers::users::update_user_by_id,
        crate::handlers::users::delete_user_by_id,

        // crate::handlers::servers::get_servers_by_page,
        // crate::handlers::servers::get_server,
        // crate::handlers::servers::add_server,
        // crate::handlers::servers::update_server,
        // crate::handlers::servers::delete_server,

        // crate::handlers::messages::get_messages,
        // crate::handlers::messages::post_message,
        // crate::handlers::messages::update_message,
        // crate::handlers::messages::delete_message,
    )
)]
pub struct ApiDoc;

pub fn get_swagger_routes() -> Router {
    Router::new().merge(
        SwaggerUi::new("/swagger-ui")
            .url("/api-docs/openapi.json", ApiDoc::openapi()),
    )
}