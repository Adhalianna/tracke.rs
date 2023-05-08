pub mod auth;
pub mod error;
pub mod json;
pub mod prelude;
pub mod response;
pub mod services;

use std::net::ToSocketAddrs;

use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};

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

    let app = services::app_services()
        .layer(axum::middleware::from_fn(crate::auth::layer::require_jwt))
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_methods(tower_http::cors::AllowMethods::any())
                .allow_origin(tower_http::cors::AllowOrigin::any())
                .allow_headers(tower_http::cors::preflight_request_headers().collect::<Vec<_>>()),
        )
        .with_state(state);

    let server_address = {
        #[cfg(feature = "local-dev")]
        let mut address = dotenvy::var("API_SERVER_ADDRESS")
                .expect("API_SERVER_ADDRESS environment variable must be set")
                .to_socket_addrs().expect("failed to parse or look up the value of API_SERVER_ADDRESS as a valid valid socket address");
        #[cfg(not(feature = "local-dev"))]
        let mut address = std::env::var("API_SERVER_ADDRESS")
                .expect("API_SERVER_ADDRESS environment variable must be set")
                .to_socket_addrs().expect("failed to parse or look up the value of API_SERVER_ADDRESS as a valid socket address");

        address.next().expect(
            "failed to find any matching socket addresses for the value of API_SERVER_ADDRESS",
        )
    };

    println!("launching the server at: {server_address}");

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

    println!("  signal received, starting graceful shutdown");
}
