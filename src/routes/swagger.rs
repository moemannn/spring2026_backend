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

        crate::handlers::groups::get_groups,
        crate::handlers::groups::get_group,
        crate::handlers::groups::add_group,
        crate::handlers::groups::edit_group,
        crate::handlers::groups::delete_group,
    )
)]
pub struct ApiDoc;

pub fn get_swagger_routes() -> Router {
    Router::new().merge(
        SwaggerUi::new("/swagger-ui")
            .url("/api-docs/openapi.json", ApiDoc::openapi()),
    )
}