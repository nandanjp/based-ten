use std::time::Duration;

use axum::{routing::get, Router};
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("expected environment variables/failed to parse the .env file");

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "based_ten=debug,axum-debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_str = std::env::var("DATABASE_URL")
        .expect("Could not find a DATABASE_URL")
        .to_string();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_str)
        .await
        .expect(&format!(
            "failed to connect to the database using the provided connection string: {db_str}"
        ));

    let port = std::env::var("PORT")
        .unwrap()
        .parse::<u32>()
        .expect("failed to retrieve a valid port");
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{port}"))
        .await
        .expect("failed to retrieve a tcp listener: could not start up a server on the given port");

    tracing::debug!("Now listening on port {port}");

    let app = Router::new()
        .route(
            "/health",
            get(|| async { "Server is healthy and running!" }),
        )
        .layer(tower_http::timeout::TimeoutLayer::new(Duration::from_secs(
            10,
        )))
        .layer(tower_http::limit::RequestBodyLimitLayer::new(1024))
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_methods([
                    http::Method::GET,
                    http::Method::POST,
                    http::Method::PATCH,
                    http::Method::DELETE,
                ])
                .allow_origin(tower_http::cors::Any),
        );

    axum::serve(listener, app.with_state(pool).into_make_service())
        .await
        .expect("failed to serve the axum server on the provided tcp listener")
}
