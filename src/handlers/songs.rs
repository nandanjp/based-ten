use std::sync::Arc;

use crate::{
    models::songs::{CreateSong, QuerySong, UpdateSong},
    services::songs::SongService,
    utils::{
        response::{get_list_response, get_one_response},
        traits::GeneralService,
    },
    AppState,
};
use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use http::StatusCode;

pub async fn get_all_songs(
    State(pool): State<Arc<AppState>>,
    Query(query): Query<QuerySong>,
) -> impl IntoResponse {
    get_list_response(
        SongService::get_all(&pool.db, query)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn get_song_by_id(
    State(pool): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    get_one_response(
        SongService::get_by_id(&pool.db, id)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn create_song(
    State(pool): State<Arc<AppState>>,
    Json(create): Json<CreateSong>,
) -> impl IntoResponse {
    get_one_response(
        SongService::create(&pool.db, create)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::CREATED,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn update_song(
    State(pool): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(update): Json<UpdateSong>,
) -> impl IntoResponse {
    get_one_response(
        SongService::update(&pool.db, update, id)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn delete_song(
    State(pool): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    get_one_response(
        SongService::delete(&pool.db, id)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}
