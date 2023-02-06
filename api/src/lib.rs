use axum::{Router, routing::IntoMakeService};
use sea_orm::DatabaseConnection;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

mod error;
mod handler;
mod dto;
mod util;
mod service;

type DbConn = DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    db: DbConn,
}

pub fn app(db_conn: DbConn) -> IntoMakeService<Router<()>> {
    let state = AppState { db: db_conn };

    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http());

    Router::new()
        .nest("/", handler::routes())
        .layer(middleware_stack.into_inner())
        .with_state(state).into_make_service()
}