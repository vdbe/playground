use axum::{Router, routing::get, response::Html};

pub(crate) fn routes() -> Router {
    Router::new().route("/", get(handler))
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
