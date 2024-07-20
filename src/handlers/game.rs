use std::sync::Arc;

use crate::{
    models::game::{CreateGame, QueryGame, UpdateGame},
    services::game::GameService,
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

pub async fn get_all_games(
    State(pool): State<Arc<AppState>>,
    Query(query): Query<QueryGame>,
) -> impl IntoResponse {
    get_list_response(
        GameService::get_all(&pool.db, query).await.map_err(|e| {
            format!("failed to retrieve all games due to the following error: {e:#?}")
        }),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn get_game_by_id(
    State(pool): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    get_one_response(
        GameService::get_by_id(&pool.db, id).await.map_err(|e| {
            format!("failed to retrieve game with id = {id} due to the following error: {e:#?}")
        }),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn create_game(
    State(pool): State<Arc<AppState>>,
    Json(create): Json<CreateGame>,
) -> impl IntoResponse {
    get_one_response(
        GameService::create(&pool.db, create).await.map_err(|e| {
            format!("failed to create game with given details due to the following error: {e:#?}")
        }),
        StatusCode::CREATED,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn update_game(
    State(pool): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(update): Json<UpdateGame>,
) -> impl IntoResponse {
    get_one_response(
        GameService::update(&pool.db, update, id)
            .await
            .map_err(|e| {
                format!(
                "failed to update game with the given details due to the following error: {e:#?}"
            )
            }),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn delete_game(
    State(pool): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    get_one_response(
        GameService::delete(&pool.db, id)
            .await
            .map_err(|e| format!("failed to delete game due to the following error: {e:#?}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}
