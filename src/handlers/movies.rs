use std::sync::Arc;

use crate::{
    models::movies::{CreateMovie, QueryMovie, UpdateMovie},
    services::movies::MovieService,
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

pub async fn get_all_movies(
    State(pool): State<Arc<AppState>>,
    Query(query): Query<QueryMovie>,
) -> impl IntoResponse {
    get_list_response(
        MovieService::get_all(&pool.db, query).await.map_err(|e| {
            format!("failed to retrieve all movies due to the following error: {e:#?}")
        }),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn get_movie_by_id(
    State(pool): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    get_one_response(
        MovieService::get_by_id(&pool.db, id).await.map_err(|e| {
            format!("failed to retrieve movie with id = {id} due to the following error: {e:#?}")
        }),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn create_movie(
    State(pool): State<Arc<AppState>>,
    Json(create): Json<CreateMovie>,
) -> impl IntoResponse {
    get_one_response(
        MovieService::create(&pool.db, create).await.map_err(|e| {
            format!("failed to create movie with given details due to the following error: {e:#?}")
        }),
        StatusCode::CREATED,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn update_movie(
    State(pool): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(update): Json<UpdateMovie>,
) -> impl IntoResponse {
    get_one_response(
        MovieService::update(&pool.db, update, id)
            .await
            .map_err(|e| {
                format!(
                    "failed to update movie with given details due to the following error: {e:#?}"
                )
            }),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn delete_movie(
    State(pool): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    get_one_response(
        MovieService::delete(&pool.db, id).await.map_err(|e| {
            format!("failed to delete movie with id = {id} due to the following error: {e:#?}")
        }),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}
