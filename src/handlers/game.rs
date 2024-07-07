use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use http::StatusCode;
use serde::Serialize;
use sqlx::PgPool;

use crate::{
    models::{
        anime::AnimeSerial,
        game::{CreateGame, GameError, GameQuery, GameSerial, UpdateGame},
    },
    services::game::GameService,
    utils::traits::{GeneralService, IntoSerial},
};

#[derive(Debug, Serialize)]
struct GameResponse {
    success: bool,
    game: Option<GameSerial>,
    error: Option<GameError>,
}

#[derive(Debug, Serialize)]
struct ListGameResponse {
    success: bool,
    games: Option<Vec<GameSerial>>,
    error: Option<GameError>,
}

pub async fn get_all_games(
    State(pool): State<PgPool>,
    Query(query): Query<GameQuery>,
) -> impl IntoResponse {
    match GameService::get_all(&pool, query).await {
        Ok(games) => (
            StatusCode::OK,
            Json(ListGameResponse {
                success: true,
                games: Some(
                    games
                        .into_iter()
                        .map(|s| s.to_serial())
                        .collect::<Vec<AnimeSerial>>(),
                ),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(ListGameResponse {
                success: false,
                games: None,
                error: Some(format!(
                    "failed to retrieve all songs due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn get_game_by_id(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    match GameService::get_by_id(&pool, id).await {
        Ok(game) => (
            StatusCode::OK,
            Json(GameResponse {
                success: true,
                game: Some(game.to_serial()),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(GameResponse {
                success: false,
                game: None,
                error: Some(format!(
                    "failed to retrieve song with id={id} due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn create_game(
    State(pool): State<PgPool>,
    Json(create): Json<CreateSong>,
) -> impl IntoResponse {
    match GameService::create(&pool, create).await {
        Ok(game) => (
            StatusCode::OK,
            Json(GameResponse {
                success: true,
                game: Some(game.to_serial()),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(GameResponse {
                success: false,
                game: None,
                error: Some(format!(
                    "failed to create song with given details due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn update_game(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(update): Json<UpdateSong>,
) -> impl IntoResponse {
    match GameService::update(&pool, update, id).await {
        Ok(game) => (
            StatusCode::OK,
            Json(GameResponse {
                success: true,
                game: Some(game.to_serial()),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(GameResponse {
                success: false,
                game: None,
                error: Some(format!(
                    "failed to update song with given details due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn delete_game(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    match GameService::delete(&pool, id).await {
        Ok(song) => (
            StatusCode::OK,
            Json(ListSongResponse {
                success: true,
                songs: Some(song.to_serial()),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(ListSongResponse {
                success: false,
                songs: None,
                error: Some(format!(
                    "failed to update song with given details due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}
