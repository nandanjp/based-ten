mod handlers;
mod middleware;
mod models;
mod routes;
mod services;
mod utils;

use axum::{routing::get, Router};
use routes::*;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{sync::Arc, time::Duration};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
struct AppState {
    db: Pool<Postgres>,
}

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
        .acquire_timeout(Duration::from_secs(10))
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
    let cors = CorsLayer::permissive();
    let app_state = Arc::new(AppState { db: pool.clone() });

    let app = Router::new()
        .route(
            "/health",
            get(|| async { "Server is healthy and running!" }),
        )
        .nest(
            "/api",
            Router::new()
                .nest("/auth", create_auth_router(app_state.clone()))
                .nest("/anime", create_anime_router())
                .nest("/movies", create_movies_router())
                .nest("/songs", create_songs_router())
                .nest("/videogames", create_games_router())
                .nest("/media", create_media_router())
                .nest("/lists", create_lists_router(app_state.clone()))
                .nest("/explore", create_explore_router(app_state.clone()))
                .nest(
                    "/listitems/user",
                    create_listitems_router(app_state.clone()),
                )
                .nest("/users", create_user_router(app_state.clone()))
                .nest("/likes", create_likes_router(app_state.clone()))
                .nest("/follow", create_follow_router(app_state.clone()))
                .nest("/groups", create_groups_router(app_state.clone())),
        )
        .with_state(app_state)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .layer(tower_http::timeout::TimeoutLayer::new(Duration::from_secs(
            10,
        )))
        .layer(tower_http::limit::RequestBodyLimitLayer::new(1024));

    tracing::debug!("Now listening on port {port}");
    axum::serve(listener, app.into_make_service())
        .await
        .expect("failed to serve the axum server on the provided tcp listener")
}
