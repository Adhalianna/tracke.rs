pub mod db;
pub mod docs;
pub mod error;
pub mod prelude;
pub mod services;

use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};
use std::net::SocketAddr;

#[derive(Clone)]
pub struct AppState {
    pub db: deadpool::managed::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
}

#[tokio::main]
pub async fn main() {
    let database_connection_pool = {
        let db_url = {
            // Fetch the variable from an appropriate source depending on the target
            // environment:
            #[cfg(feature = "local-dev")]
            let url = dotenvy::var("DATABASE_URL")
                .expect("DATABASE_URL environment variable must be set in the .env file");
            #[cfg(not(feature = "local-dev"))]
            let url = std::env::var("DATABASE_URL")
                .expect("DATABASE_URL environment variable must be set");
            url
        };

        let pool_config =
            AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(&db_url);

        let pool = diesel_async::pooled_connection::deadpool::Pool::builder(pool_config)
            .build()
            .expect("Failed to build database connections' pool");

        pool
    };

    let state = AppState {
        db: database_connection_pool,
    };

    let app = services::app_services().with_state(state);

    let server_address = SocketAddr::from(([0, 0, 0, 0], 4000));

    println!("launching the server at: {server_address} ...");

    axum::Server::bind(&server_address)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_handler())
        .await
        .expect("failed to start the http server");
}

pub async fn shutdown_handler() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
        },
        _ = terminate => {
        },
    }

    println!("  Signal received, starting graceful shutdown.");
}
