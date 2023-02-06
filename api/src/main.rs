use std::net::{IpAddr, SocketAddr};

use axum::Router;
use clap::Parser;
use tokio::signal;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;


mod handler;

#[derive(Debug, Parser)]
pub struct Config {
    #[clap(long, default_value = "127.0.0.1", env)]
    pub host: IpAddr,
    #[clap(long, default_value = "3000", env)]
    pub port: u16,
}

impl Config {
    pub fn host(&self) -> IpAddr {
        self.host
    }
}

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();

    let args = Config::parse();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let middleware_stack = ServiceBuilder::new().layer(TraceLayer::new_for_http());

    // build our application with a route
    let app = Router::new()
        .nest("/", handler::routes())
        .layer(middleware_stack.into_inner());

    // run it
    let addr = SocketAddr::from((args.host, args.port));
    tracing::debug!("listening on {}", addr);
    let server = axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal());

    if let Err(err) = server.await {
        tracing::error!("server error: {:?}", err);
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("signal received, starting graceful shutdown");
}
