use std::sync::Arc;
use axum::Router;
use tower::ServiceBuilder;
use tower_http::request_id::{SetRequestIdLayer, PropagateRequestIdLayer, MakeRequestUuid};
use tower_http::trace::TraceLayer;

use crate::routes;
use crate::state::AppState;

pub fn app(state: Arc<AppState>) -> Router {
    Router::new()
        .merge(routes::default::get_default_routes(state.clone()))
        .merge(routes::swagger::get_swagger_routes())
        .layer(
            ServiceBuilder::new()
                .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
                .layer(PropagateRequestIdLayer::x_request_id())
                .layer(TraceLayer::new_for_http())
        )
}