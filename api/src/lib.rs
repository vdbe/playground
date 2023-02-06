use axum::Router;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

mod handler;

pub fn app() -> Router {
    let middleware_stack = ServiceBuilder::new().layer(TraceLayer::new_for_http());

    Router::new()
        .nest("/", handler::routes())
        .layer(middleware_stack.into_inner())
}
