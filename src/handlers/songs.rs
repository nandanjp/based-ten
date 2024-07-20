use crate::{
    models::songs::{CreateSong, QuerySong, UpdateSong},
    services::songs::SongService,
    utils::{
        response::{get_list_response, get_one_response},
        traits::GeneralService,
    },
};
use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use http::StatusCode;
use sqlx::PgPool;

pub async fn get_all_songs(
    State(pool): State<PgPool>,
    Query(query): Query<QuerySong>,
) -> impl IntoResponse {
    get_list_response(
        SongService::get_all(&pool, query).await.map_err(|e| {
            format!("failed to retrieve all songs due to the following error: {e:#?}")
        }),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn get_song_by_id(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    get_one_response(
        SongService::get_by_id(&pool, id).await.map_err(|e| {
            format!("failed to retrieve song with id = {id} due to the following error: {e:#?}")
        }),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn create_song(
    State(pool): State<PgPool>,
    Json(create): Json<CreateSong>,
) -> impl IntoResponse {
    get_one_response(
        SongService::create(&pool, create).await.map_err(|e| {
            format!("failed to create song with given details due to the following error: {e:#?}")
        }),
        StatusCode::CREATED,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn update_song(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(update): Json<UpdateSong>,
) -> impl IntoResponse {
    get_one_response(
        SongService::update(&pool, update, id).await.map_err(|e| {
            format!("failed to update song with given details due to the following error: {e:#?}")
        }),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn delete_song(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    get_one_response(
        SongService::delete(&pool, id)
            .await
            .map_err(|e| format!("failed to dekete song due to the following error: {e:#?}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}
