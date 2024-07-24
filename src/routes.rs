use std::sync::Arc;

use crate::{
    handlers::{
        anime::*, auth::*, followmutual::*, follows::*, game::*, groups::*, likes::*, listitems::*,
        lists::*, media::*, movies::*, songs::*, users::*,
    },
    AppState,
};

use crate::middleware::auth::auth;
use axum::{
    middleware as axum_middleware,
    routing::{delete, get, patch, post},
    Router,
};

pub fn create_auth_router(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(register_user_handler))
        .route("/login", post(login_user_handler))
        .route(
            "/logout",
            get(logout_handler)
                .route_layer(axum_middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .route(
            "/user",
            get(get_logged_user)
                .route_layer(axum_middleware::from_fn_with_state(app_state.clone(), auth)),
        )
}

pub fn create_anime_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_all_anime))
        .route("/", post(create_anime))
        .route("/:id", get(get_anime_by_id))
        .route("/:id", patch(update_anime))
        .route("/:id", delete(delete_anime))
}

pub fn create_movies_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_all_movies))
        .route("/", post(create_movie))
        .route("/:id", get(get_movie_by_id))
        .route("/:id", patch(update_movie))
        .route("/:id", delete(delete_movie))
}

pub fn create_songs_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_all_songs))
        .route("/", post(create_song))
        .route("/:id", get(get_song_by_id))
        .route("/:id", patch(update_song))
        .route("/:id", delete(delete_song))
}

pub fn create_games_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_all_games))
        .route("/", post(create_game))
        .route("/:id", get(get_game_by_id))
        .route("/:id", patch(update_game))
        .route("/:id", delete(delete_game))
}

pub fn create_media_router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(get_all_media))
}

pub fn create_lists_router(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_all_lists))
        .route("/top", get(get_some_top_lists))
        .nest(
            "/:user_name",
            Router::new()
                .route("/", post(create_list))
                .nest(
                    "/:list_name",
                    Router::new()
                        .route("/", patch(update_list))
                        .route("/", delete(delete_list)),
                )
                .route_layer(axum_middleware::from_fn_with_state(app_state.clone(), auth)),
        )
}

pub fn create_explore_router(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new().route(
        "/:user_name",
        get(get_user_explore_lists)
            .route_layer(axum_middleware::from_fn_with_state(app_state.clone(), auth)),
    )
}

pub fn create_listitems_router(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/:list_name/:item_id", get(get_list_item))
        .route("/:list_name/:item_id", post(create_list_item))
        .route(
            "/:list_name/:item_id",
            patch(update_list_item)
                .route_layer(axum_middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .route(
            "/:list_name/:item_id",
            delete(delete_list_item)
                .route_layer(axum_middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .route_layer(axum_middleware::from_fn_with_state(app_state.clone(), auth))
}

pub fn create_user_router(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_all_users))
        .route("/", post(create_user))
        .nest(
            "/:user_name",
            Router::new()
                .route("/", get(get_user_lists))
                .route("/:list_name", get(get_user_list_items))
                .route("/:list_name/type", get(get_user_list_type))
                .nest(
                    "/me",
                    Router::new()
                        .route("/", get(get_user_by_username))
                        .route("/", patch(update_user))
                        .route("/", delete(delete_user))
                        .route_layer(axum_middleware::from_fn_with_state(app_state.clone(), auth)),
                ),
        )
}

pub fn create_likes_router(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_all_likes))
        .route(
            "/",
            post(create_user_like)
                .route_layer(axum_middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .route("/:user_name", get(get_likes_by_username))
        .route(
            "/:liking/:list_name",
            delete(delete_user_likes)
                .route_layer(axum_middleware::from_fn_with_state(app_state.clone(), auth)),
        )
}

pub fn create_follow_router(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_all_follows))
        .route(
            "/",
            post(create_follow)
                .route_layer(axum_middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .route(
            "/:following",
            delete(delete_user_follow)
                .route_layer(axum_middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .nest(
            "/:user_name",
            Router::new()
                .route("/following", get(get_follows_by_username))
                .route("/mutual", get(get_mutual_follows_by_id)),
        )
}

pub fn create_groups_router(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_all_groups))
        .nest(
            "/:user_name",
            Router::new().route("/groups", get(get_user_groups)).nest(
                "/me",
                Router::new()
                    .route("/", post(create_user_group))
                    .route("/:group_name", delete(delete_user_group))
                    .route_layer(axum_middleware::from_fn_with_state(app_state.clone(), auth)),
            ),
        )
        .route("/:gid/group", get(get_group_by_id))
        .route("/:gid/members", get(get_group_members))
        .route("/:gid/lists", get(get_group_member_lists))
        .route("/:gid/circles", get(get_circles_by_id))
}
