use crate::models::likes::{QueryLike, LikeSerial, CreateLike};
use crate::services::likes::LikesService;
use crate::utils::traits::IntoSerial;
use axum::extract::{Json, Path, Query, State};
use axum::response::IntoResponse;
use http::StatusCode;
use serde::Serialize;
use sqlx::PgPool;

#[derive(Debug, Serialize)]
struct LikesResponse {
    success: bool,
    like: Option<LikeSerial>,
    error: Option<String>,
}

#[derive(Debug, Serialize)]
struct ListLikesResponse {
    success: bool,
    likes: Option<Vec<LikeSerial>>,
    error: Option<String>,
}

pub async fn get_all_likes(
    State(pool): State<PgPool>,
    Query(query): Query<QueryLike>,
) -> impl IntoResponse {
    match LikesService::get_all(&pool, query).await {
        Ok(likes) => (
            StatusCode::OK,
            Json(ListLikesResponse {
                success: true,
                likes: Some(
                    likes
                        .into_iter()
                        .map(|a| a.to_serial())
                        .collect::<Vec<LikeSerial>>(),
                ),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(ListLikesResponse {
                success: false,
                likes: None,
                error: Some(format!(
                    "failed to retrieve likes due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn get_likes_by_id(State(pool): State<PgPool>, Path(email): Path<String>) -> impl IntoResponse {
    match LikesService::get_by_id(&pool, email).await {
        Ok(likes) => (
            StatusCode::OK,
            Json(ListLikesResponse {
                success: true,
                likes: Some(
                    likes
                        .into_iter()
                        .map(|a| a.to_serial())
                        .collect::<Vec<LikeSerial>>(),
                ),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(ListLikesResponse {
                success: false,
                likes: None,
                error: Some(format!(
                    "failed to retrieve a user's likes due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn create_like(
    State(pool): State<PgPool>,
    Json(create): Json<CreateLike>,
) -> impl IntoResponse {
    match LikesService::create(&pool, create).await {
        Ok(like) => (
            StatusCode::CREATED,
            Json(LikesResponse {
                success: true,
                like: Some(like.to_serial()),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(LikesResponse {
                success: false,
                like: None,
                error: Some(format!(
                    "failed to create like due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn delete_like(State(pool): State<PgPool>, Path((liker_email, liking_email, list_name)): Path<(String, String, String)>)
-> impl IntoResponse {
    match LikesService::delete(&pool, liker_email, liking_email, list_name).await {
        Ok(like) => (
            StatusCode::OK,
            Json(LikesResponse {
                success: true,
                like: Some(like.to_serial()),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(LikesResponse {
                success: false,
                like: None,
                error: Some(format!(
                    "failed to delete like due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}
