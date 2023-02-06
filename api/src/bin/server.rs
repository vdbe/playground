use std::{net::{IpAddr, SocketAddr}, time::Duration};

use clap::Parser;
use sea_orm::{DatabaseConnection, DbErr, ConnectOptions, Database};
use tokio::signal;
use tracing::log;
use tracing_subscriber::EnvFilter;

use migration::{Migrator, MigratorTrait};

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

async fn db() -> Result<DatabaseConnection, DbErr> {
    let mut opt =
        ConnectOptions::new("postgres://playground:toor123@localhost/playground".to_owned());

    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);
        //.set_schema_search_path("my_schema".into());

    Database::connect(opt).await
}

async fn db_migration(db_connection: &DatabaseConnection) -> Result<(), DbErr> {
    Migrator::up(db_connection, None).await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();

    let args = Config::parse();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let db_connection = db().await.expect("Failed to connect to the db");

    db_migration(&db_connection).await?;

    // build our application with a route
    let app = api::app(db_connection);

    // run it
    let addr = SocketAddr::from((args.host, args.port));
    tracing::info!("listening on {}", addr);
    let server = axum::Server::bind(&addr)
        .serve(app)
        .with_graceful_shutdown(shutdown_signal());

    if let Err(err) = server.await {
        tracing::error!("server error: {:?}", err);
    }

    Ok(())
}

