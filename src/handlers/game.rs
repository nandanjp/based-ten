use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use http::StatusCode;
use serde::Serialize;
use sqlx::PgPool;

use crate::{
    models::game::{CreateGame, GameSerial, QueryGame, UpdateGame},
    services::game::GameService,
    utils::traits::{GeneralService, IntoSerial},
};

#[derive(Debug, Serialize)]
struct GameResponse {
    success: bool,
    game: Option<GameSerial>,
    error: Option<String>,
}

#[derive(Debug, Serialize)]
struct ListGameResponse {
    success: bool,
    games: Option<Vec<GameSerial>>,
    error: Option<String>,
}

pub async fn get_all_games(
    State(pool): State<PgPool>,
    Query(query): Query<QueryGame>,
) -> impl IntoResponse {
    match GameService::get_all(&pool, query).await {
        Ok(games) => (
            StatusCode::OK,
            Json(ListGameResponse {
                success: true,
                games: Some(
                    games
                        .into_iter()
                        .map(|g| g.to_serial())
                        .collect::<Vec<GameSerial>>(),
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
    Json(create): Json<CreateGame>,
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
    Json(update): Json<UpdateGame>,
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
