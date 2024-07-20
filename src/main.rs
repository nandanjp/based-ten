mod handlers;
mod middleware;
mod models;
mod services;
mod utils;
use handlers::{
    anime::{create_anime, delete_anime, get_all_anime, get_anime_by_id, update_anime},
    auth::{login_user_handler, logout_handler, register_user_handler},
    followmutual::get_mutual_follows_by_id,
    follows::{create_follow, delete_follow, get_all_follows, get_follows_by_id},
    game::{create_game, delete_game, get_all_games, get_game_by_id, update_game},
    groups::{
        create_groups, delete_groups, get_all_groups, get_circles_by_id, get_group_member_lists,
        get_groups_by_id,
    },
    likes::{create_like, delete_like, get_all_likes, get_likes_by_id},
    listitems::{create_list_item, delete_list_item, get_list_item, update_list_item},
    lists::{
        create_list, delete_list, get_all_lists, get_list_and_items, get_some_top_lists,
        get_user_explore_lists, get_user_list, get_user_list_items, get_user_lists, update_list,
    },
    media::get_all_media,
    movies::{create_movie, delete_movie, get_all_movies, get_movie_by_id, update_movie},
    songs::{create_song, delete_song, get_all_songs, get_song_by_id, update_song},
    users::{create_user, delete_user, get_all_users, get_user_by_id, update_user},
};

use axum::{
    middleware as axum_middleware,
    routing::{delete, get, patch, post},
    Router,
};
use http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use middleware::auth;
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

    let cors = CorsLayer::new()
        .allow_origin(
            format!("http://localhost:{port}")
                .as_str()
                .parse::<HeaderValue>()
                .unwrap(),
        )
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app_state = Arc::new(AppState { db: pool.clone() });

    let app = Router::new()
        .route(
            "/health",
            get(|| async { "Server is healthy and running!" }),
        )
        .nest(
            "/api",
            Router::new()
                .nest(
                    "/auth",
                    Router::new()
                        .route("/register", post(register_user_handler))
                        .route("/login", post(login_user_handler))
                        .route(
                            "/logout",
                            get(logout_handler).route_layer(axum_middleware::from_fn_with_state(
                                app_state.clone(),
                                auth,
                            )),
                        ),
                )
                .nest(
                    "/anime",
                    Router::new()
                        .route("/", get(get_all_anime))
                        .route("/", post(create_anime))
                        .route("/:id", get(get_anime_by_id))
                        .route("/:id", patch(update_anime))
                        .route("/:id", delete(delete_anime)),
                )
                .nest(
                    "/movies",
                    Router::new()
                        .route("/", get(get_all_movies))
                        .route("/", post(create_movie))
                        .route("/:id", get(get_movie_by_id))
                        .route("/:id", patch(update_movie))
                        .route("/:id", delete(delete_movie)),
                )
                .nest(
                    "/songs",
                    Router::new()
                        .route("/", get(get_all_songs))
                        .route("/", post(create_song))
                        .route("/:id", get(get_song_by_id))
                        .route("/:id", patch(update_song))
                        .route("/:id", delete(delete_song)),
                )
                .nest(
                    "/videogames",
                    Router::new()
                        .route("/", get(get_all_games))
                        .route("/", post(create_game))
                        .route("/:id", get(get_game_by_id))
                        .route("/:id", patch(update_game))
                        .route("/:id", delete(delete_game)),
                )
                .nest(
                    "/media",
                    Router::new().route("/", get(get_all_media)), //.route("/:type", get(get_media_by_type)),
                )
                .nest(
                    "/lists",
                    Router::new()
                        .route("/", get(get_all_lists))
                        .route("/", post(create_list))
                        .route("/view/:list_name", get(get_list_and_items))
                        .route("/top", get(get_some_top_lists))
                        .nest(
                            "/:user_name",
                            Router::new()
                                .route("/", get(get_user_lists))
                                .route("/:list_name", get(get_user_list))
                                .route("/:list_name/items", get(get_user_list_items))
                                .route("/:list_name", patch(update_list))
                                .route("/:list_name", delete(delete_list)),
                        ),
                )
                .nest(
                    "/explore",
                    Router::new().route("/:user_name", get(get_user_explore_lists)),
                )
                .nest(
                    "/listitems",
                    Router::new()
                        .route("/:user_name/:listname/:item_id", get(get_list_item))
                        .route("/:user_name/:listname/:item_id", post(create_list_item))
                        .route("/:user_name/:listname/:item_id", patch(update_list_item))
                        .route("/:user_name/:listname/:item_id", delete(delete_list_item)),
                )
                .nest(
                    "/users",
                    Router::new()
                        .route("/", get(get_all_users))
                        .route("/", post(create_user))
                        .route("/:user_name", get(get_user_by_id))
                        .route("/:user_name", patch(update_user))
                        .route("/:user_name", delete(delete_user)),
                )
                .nest(
                    "/likes",
                    Router::new()
                        .route("/", get(get_all_likes))
                        .route("/", post(create_like))
                        .route("/:user_name", get(get_likes_by_id))
                        .route("/:likeremail/:likingemail/:listname", delete(delete_like)),
                )
                .nest(
                    "/follow",
                    Router::new()
                        .route("/", get(get_all_follows))
                        .route("/", post(create_follow))
                        .route("/:user_name", get(get_follows_by_id))
                        .route("/:followeremail/:followingemail", delete(delete_follow))
                        .route("/mutual/:useremail", get(get_mutual_follows_by_id)),
                )
                .nest(
                    "/groups",
                    Router::new()
                        .route("/", get(get_all_groups))
                        .route("/:gid/:groupName/:ownedBy", post(create_groups))
                        .route("/:gid", get(get_groups_by_id))
                        .route("/:gid", delete(delete_groups))
                        .route("/:gid/lists", get(get_group_member_lists))
                        .route("/:gid/circles", get(get_circles_by_id)),
                ),
        )
        .with_state(app_state)
        .layer(TraceLayer::new_for_http())
        .layer(tower_http::timeout::TimeoutLayer::new(Duration::from_secs(
            10,
        )))
        .layer(tower_http::limit::RequestBodyLimitLayer::new(1024))
        .layer(cors);

    axum::serve(listener, app.into_make_service())
        .await
        .expect("failed to serve the axum server on the provided tcp listener")
}
