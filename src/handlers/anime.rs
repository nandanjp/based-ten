use crate::models::anime::{AnimeQuery, AnimeSerial, CreateAnime, UpdateAnime};
use crate::services::anime::AnimeService;
use crate::utils::traits::{GeneralService, IntoSerial};
use axum::extract::{Json, Path, Query, State};
use axum::response::IntoResponse;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Serialize)]
struct AnimeResponse {
    success: bool,
    anime: Option<AnimeSerial>,
    error: Option<String>,
}

#[derive(Debug, Serialize)]
struct ListAnimeResponse {
    success: bool,
    anime: Option<Vec<AnimeSerial>>,
    error: Option<String>,
}

pub async fn get_all_anime(
    State(pool): State<PgPool>,
    Query(query): Query<AnimeQuery>,
) -> impl IntoResponse {
    match AnimeService::get_all(&pool, query).await {
        Ok(anime) => (
            StatusCode::OK,
            Json(ListAnimeResponse {
                success: true,
                anime: Some(
                    anime
                        .into_iter()
                        .map(|a| a.to_serial())
                        .collect::<Vec<AnimeSerial>>(),
                ),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(ListAnimeResponse {
                success: false,
                anime: None,
                error: Some(format!(
                    "failed to retrieve all anime due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn get_anime_by_id(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    match AnimeService::get_by_id(&pool, id).await {
        Ok(anime) => (
            StatusCode::OK,
            Json(AnimeResponse {
                success: true,
                anime: Some(anime.to_serial()),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(AnimeResponse {
                success: false,
                anime: None,
                error: Some(format!(
                    "failed to retrieve an anime due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn create_anime(
    State(pool): State<PgPool>,
    Json(create): Json<CreateAnime>,
) -> impl IntoResponse {
    match AnimeService::create(&pool, create).await {
        Ok(anime) => (
            StatusCode::CREATED,
            Json(AnimeResponse {
                success: true,
                anime: Some(anime.to_serial()),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(AnimeResponse {
                success: false,
                anime: None,
                error: Some(format!(
                    "failed to create anime due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn update_anime(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(update): Json<UpdateAnime>,
) -> impl IntoResponse {
    match AnimeService::update(&pool, update, id).await {
        Ok(anime) => (
            StatusCode::OK,
            Json(AnimeResponse {
                success: true,
                anime: Some(anime.to_serial()),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(AnimeResponse {
                success: false,
                anime: None,
                error: Some(format!(
                    "failed to update anime due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn delete_anime(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    match AnimeService::delete(&pool, id).await {
        Ok(anime) => (
            StatusCode::OK,
            Json(AnimeResponse {
                success: true,
                anime: Some(anime.to_serial()),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(AnimeResponse {
                success: false,
                anime: None,
                error: Some(format!(
                    "failed to delete anime due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}
