use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use http::StatusCode;
use serde::Serialize;
use sqlx::PgPool;

use crate::{
    models::songs::{CreateSong, QuerySong, SongSerial, UpdateSong},
    services::songs::SongService,
    utils::traits::{GeneralService, IntoSerial},
};

#[derive(Debug, Serialize)]
struct SongResponse {
    success: bool,
    song: Option<SongSerial>,
    error: Option<String>,
}

#[derive(Debug, Serialize)]
struct ListSongResponse {
    success: bool,
    songs: Option<Vec<SongSerial>>,
    error: Option<String>,
}

pub async fn get_all_songs(
    State(pool): State<PgPool>,
    Query(query): Query<QuerySong>,
) -> impl IntoResponse {
    match SongService::get_all(&pool, query).await {
        Ok(songs) => (
            StatusCode::OK,
            Json(ListSongResponse {
                success: true,
                songs: Some(
                    songs
                        .into_iter()
                        .map(|s| s.to_serial())
                        .collect::<Vec<SongSerial>>(),
                ),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(ListSongResponse {
                success: false,
                songs: None,
                error: Some(format!(
                    "failed to retrieve all songs due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn get_song_by_id(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    match SongService::get_by_id(&pool, id).await {
        Ok(song) => (
            StatusCode::OK,
            Json(SongResponse {
                success: true,
                song: Some(song.to_serial()),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(SongResponse {
                success: false,
                song: None,
                error: Some(format!(
                    "failed to retrieve song with id={id} due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn create_song(
    State(pool): State<PgPool>,
    Json(create): Json<CreateSong>,
) -> impl IntoResponse {
    match SongService::create(&pool, create).await {
        Ok(song) => (
            StatusCode::OK,
            Json(SongResponse {
                success: true,
                song: Some(song.to_serial()),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(SongResponse {
                success: false,
                song: None,
                error: Some(format!(
                    "failed to create song with given details due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn update_song(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(update): Json<UpdateSong>,
) -> impl IntoResponse {
    match SongService::update(&pool, update, id).await {
        Ok(song) => (
            StatusCode::OK,
            Json(SongResponse {
                success: true,
                song: Some(song.to_serial()),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(SongResponse {
                success: false,
                song: None,
                error: Some(format!(
                    "failed to update song with given details due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn delete_song(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    match SongService::delete(&pool, id).await {
        Ok(song) => (
            StatusCode::OK,
            Json(SongResponse {
                success: true,
                song: Some(song.to_serial()),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(SongResponse {
                success: false,
                song: None,
                error: Some(format!(
                    "failed to delete song with given details due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}
