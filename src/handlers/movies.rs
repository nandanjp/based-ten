use crate::{
    models::movies::{CreateMovie, QueryMovie, UpdateMovie},
    services::movies::MovieService,
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

pub async fn get_all_movies(
    State(pool): State<PgPool>,
    Query(query): Query<QueryMovie>,
) -> impl IntoResponse {
    get_list_response(
        MovieService::get_all(&pool, query).await.map_err(|e| {
            format!("failed to retrieve all movies due to the following error: {e:#?}")
        }),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn get_movie_by_id(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    get_one_response(
        MovieService::get_by_id(&pool, id).await.map_err(|e| {
            format!("failed to retrieve movie with id = {id} due to the following error: {e:#?}")
        }),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn create_movie(
    State(pool): State<PgPool>,
    Json(create): Json<CreateMovie>,
) -> impl IntoResponse {
    get_one_response(
        MovieService::create(&pool, create).await.map_err(|e| {
            format!("failed to create movie with given details due to the following error: {e:#?}")
        }),
        StatusCode::CREATED,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn update_movie(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(update): Json<UpdateMovie>,
) -> impl IntoResponse {
    get_one_response(
        MovieService::update(&pool, update, id).await.map_err(|e| {
            format!("failed to update movie with given details due to the following error: {e:#?}")
        }),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn delete_movie(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    get_one_response(
        MovieService::delete(&pool, id).await.map_err(|e| {
            format!("failed to delete movie with id = {id} due to the following error: {e:#?}")
        }),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}
