use std::sync::Arc;

use crate::{
    handlers::{
        anime::{create_anime, delete_anime, get_all_anime, get_anime_by_id, update_anime},
        auth::{login_user_handler, logout_handler, register_user_handler},
        followmutual::get_mutual_follows_by_id,
        follows::{
            create_follow, delete_follow, delete_follows, get_all_follows, get_follows_by_id,
        },
        game::{create_game, delete_game, get_all_games, get_game_by_id, update_game},
        groups::{
            create_groups, delete_groups, delete_user_group, get_all_groups, get_circles_by_id,
            get_group_member_lists, get_group_members, get_groups_by_id, get_user_groups,
        },
        likes::{create_like, delete_like, delete_user_likes, get_all_likes, get_likes_by_id},
        listitems::{create_list_item, delete_list_item, get_list_item, update_list_item},
        lists::{
            create_list, delete_list, get_all_lists, get_some_top_lists, get_user_explore_lists,
            get_user_list, get_user_list_items, get_user_lists, update_list,
        },
        media::get_all_media,
        movies::{create_movie, delete_movie, get_all_movies, get_movie_by_id, update_movie},
        songs::{create_song, delete_song, get_all_songs, get_song_by_id, update_song},
        users::{create_user, delete_user, get_all_users, get_user_by_id, update_user},
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
            get(logout_handler).route_layer(axum_middleware::from_fn_with_state(app_state, auth)),
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
                .route("/", get(get_user_lists))
                .route("/", post(create_list))
                .route("/:list_name", get(get_user_list))
                .route("/:list_name/items", get(get_user_list_items))
                .route("/:list_name", patch(update_list))
                .route("/:list_name", delete(delete_list)),
        )
}

pub fn create_explore_router(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new().route(
        "/user",
        get(get_user_explore_lists)
            .route_layer(axum_middleware::from_fn_with_state(app_state.clone(), auth)),
    )
}

pub fn create_listitems_router(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/:list_name/:item_id", get(get_list_item))
        .route("/:list_name/:item_id", post(create_list_item))
        .route("/:list_name/:item_id", patch(update_list_item))
        .route("/:list_name/:item_id", delete(delete_list_item))
        .route_layer(axum_middleware::from_fn_with_state(app_state.clone(), auth))
}

pub fn create_user_routes(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_all_users))
        .route("/", post(create_user))
        .nest(
            "/user",
            Router::new()
                .route("/", get(get_user_by_id))
                .route("/", patch(update_user))
                .route("/", delete(delete_user))
                .route_layer(axum_middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .nest(
            "/:user_name",
            Router::new()
                .route("/", get(get_user_lists))
                .route("/", post(create_list))
                .route("/:list_name", get(get_user_list))
                .route("/:list_name/items", get(get_user_list_items)),
        )
}

pub fn create_likes_routes(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_all_likes))
        .route("/", post(create_like))
        .nest(
            "/:user_name",
            Router::new()
                .route("/", get(get_likes_by_id))
                .route("/:liking/:list_name", delete(delete_user_likes)),
        )
        .route("/:liker/:liking/:list_name", delete(delete_like))
}

pub fn create_follow_routes(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_all_follows))
        .route("/", post(create_follow))
        .nest(
            "/:user_name",
            Router::new()
                .route("/", get(get_follows_by_id))
                .route("/:following", delete(delete_follow))
                .route("/mutual", get(get_mutual_follows_by_id)),
        )
        .route("/:follower/:following", delete(delete_follows))
}

pub fn create_groups_router(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_all_groups))
        .nest(
            "/:user_name",
            Router::new()
                .route("/", get(get_user_groups))
                .route("/", post(create_groups))
                .route("/:group_name", delete(delete_user_group)),
        )
        .route("/:gid", get(get_groups_by_id))
        .route("/:gid/members", get(get_group_members))
        .route("/:gid", delete(delete_groups))
        .route("/:gid/lists", get(get_group_member_lists))
        .route("/:gid/circles", get(get_circles_by_id))
}
