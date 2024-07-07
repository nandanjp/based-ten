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
        movies::{CreateMovie, MovieError, MovieQuery, MovieSerial, UpdateMovie},
    },
    services::movies::MovieService,
    utils::traits::{GeneralService, IntoSerial},
};

#[derive(Debug, Serialize)]
struct MovieResponse {
    success: bool,
    movie: Option<MovieSerial>,
    error: Option<MovieError>,
}

#[derive(Debug, Serialize)]
struct ListMovieResponse {
    success: bool,
    movies: Option<Vec<MovieSerial>>,
    error: Option<MovieError>,
}

pub async fn get_all_movies(
    State(pool): State<PgPool>,
    Query(query): Query<MovieQuery>,
) -> impl IntoResponse {
    match MovieService::get_all(&pool, query).await {
        Ok(songs) => (
            StatusCode::OK,
            Json(ListMovieResponse {
                success: true,
                movies: Some(
                    movies
                        .into_iter()
                        .map(|s| s.to_serial())
                        .collect::<Vec<AnimeSerial>>(),
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
                    "failed to retrieve all songs due to the following error: {err:#?}"
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
                    "failed to retrieve song with id={id} due to the following error: {err:#?}"
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
                    "failed to create song with given details due to the following error: {err:#?}"
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
                    "failed to update song with given details due to the following error: {err:#?}"
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
                    "failed to update song with given details due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}
