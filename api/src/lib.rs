use axum::{routing::IntoMakeService, Router};
use sea_orm::DatabaseConnection;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

mod config;
mod db;
mod dto;
mod error;
mod extractor;
mod handler;
mod service;
mod util;

type DbConn = DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    db: DbConn,
}

pub fn app(db_conn: DbConn) -> IntoMakeService<Router<()>> {
    let state = AppState { db: db_conn };

    let middleware_stack =
        ServiceBuilder::new().layer(TraceLayer::new_for_http());

    Router::new()
        .nest("/user", handler::user::routes())
        .nest("/auth", handler::auth::routes())
        .layer(middleware_stack.into_inner())
        .with_state(state)
        .into_make_service()
}
