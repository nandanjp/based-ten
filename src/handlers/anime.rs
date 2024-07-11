use crate::models::anime::{AnimeQuery, CreateAnime, UpdateAnime};
use crate::services::anime::AnimeService;
use crate::utils::response::{get_list_response, get_one_response};
use crate::utils::traits::GeneralService;
use axum::extract::{Json, Path, Query, State};
use axum::response::IntoResponse;
use http::StatusCode;
use sqlx::PgPool;

pub async fn get_all_anime(
    State(pool): State<PgPool>,
    Query(query): Query<AnimeQuery>,
) -> impl IntoResponse {
    get_list_response(
        AnimeService::get_all(&pool, query).await.map_err(|e| {
            format!("failed to retrieve all anime due to the following error: {e:#?}")
        }),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn get_anime_by_id(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    get_one_response(
        AnimeService::get_by_id(&pool, id)
            .await
            .map_err(|e| format!("failed to retrieve an anime due to the following error: {e:#?}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn create_anime(
    State(pool): State<PgPool>,
    Json(create): Json<CreateAnime>,
) -> impl IntoResponse {
    get_one_response(
        AnimeService::create(&pool, create)
            .await
            .map_err(|e| format!("failed to create anime due to the following error: {e:#?}")),
        StatusCode::CREATED,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn update_anime(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(update): Json<UpdateAnime>,
) -> impl IntoResponse {
    get_one_response(
        AnimeService::update(&pool, update, id)
            .await
            .map_err(|e| format!("failed to update anime due to the following error: {e:#?}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn delete_anime(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    get_one_response(
        AnimeService::delete(&pool, id)
            .await
            .map_err(|e| format!("failed to delete anime due to the following error: {e:#?}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}
