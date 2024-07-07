use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use http::StatusCode;
use serde::Serialize;
use sqlx::PgPool;

use crate::{
    models::movies::{CreateMovie, MovieSerial, QueryMovie, UpdateMovie},
    services::movies::MovieService,
    utils::traits::{GeneralService, IntoSerial},
};

#[derive(Debug, Serialize)]
struct MovieResponse {
    success: bool,
    movie: Option<MovieSerial>,
    error: Option<String>,
}

#[derive(Debug, Serialize)]
struct ListMovieResponse {
    success: bool,
    movies: Option<Vec<MovieSerial>>,
    error: Option<String>,
}

pub async fn get_all_movies(
    State(pool): State<PgPool>,
    Query(query): Query<QueryMovie>,
) -> impl IntoResponse {
    match MovieService::get_all(&pool, query).await {
        Ok(movies) => (
            StatusCode::OK,
            Json(ListMovieResponse {
                success: true,
                movies: Some(
                    movies
                        .into_iter()
                        .map(|m| m.to_serial())
                        .collect::<Vec<MovieSerial>>(),
                ),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(ListMovieResponse {
                success: false,
                movies: None,
                error: Some(format!(
                    "failed to retrieve all movies due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn get_movie_by_id(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    match MovieService::get_by_id(&pool, id).await {
        Ok(movie) => (
            StatusCode::OK,
            Json(MovieResponse {
                success: true,
                movie: Some(movie.to_serial()),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(MovieResponse {
                success: false,
                movie: None,
                error: Some(format!(
                    "failed to retrieve movie with id={id} due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn create_movie(
    State(pool): State<PgPool>,
    Json(create): Json<CreateMovie>,
) -> impl IntoResponse {
    match MovieService::create(&pool, create).await {
        Ok(movie) => (
            StatusCode::OK,
            Json(MovieResponse {
                success: true,
                movie: Some(movie.to_serial()),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(MovieResponse {
                success: false,
                movie: None,
                error: Some(format!(
                    "failed to create movie with given details due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn update_movie(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(update): Json<UpdateMovie>,
) -> impl IntoResponse {
    match MovieService::update(&pool, update, id).await {
        Ok(movie) => (
            StatusCode::OK,
            Json(MovieResponse {
                success: true,
                movie: Some(movie.to_serial()),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(MovieResponse {
                success: false,
                movie: None,
                error: Some(format!(
                    "failed to update movie with given details due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn delete_movie(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    match MovieService::delete(&pool, id).await {
        Ok(movie) => (
            StatusCode::OK,
            Json(MovieResponse {
                success: true,
                movie: Some(movie.to_serial()),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(MovieResponse {
                success: false,
                movie: None,
                error: Some(format!(
                    "failed to update movie with given details due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}
