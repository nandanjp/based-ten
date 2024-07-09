use crate::models::follows::{CreateFollow, Follow, QueryFollow};
use crate::services::follows::FollowsService;
use axum::extract::{Json, Path, Query, State};
use axum::response::IntoResponse;
use http::StatusCode;
use serde::Serialize;
use sqlx::PgPool;

#[derive(Debug, Serialize)]
struct FollowsResponse {
    success: bool,
    follow: Option<Follow>,
    error: Option<String>,
}

#[derive(Debug, Serialize)]
struct ListFollowsResponse {
    success: bool,
    follows: Option<Vec<Follow>>,
    error: Option<String>,
}

pub async fn get_all_follows(
    State(pool): State<PgPool>,
    Query(query): Query<QueryFollow>,
) -> impl IntoResponse {
    match FollowsService::get_all(&pool, query).await {
        Ok(follows) => (
            StatusCode::OK,
            Json(ListFollowsResponse {
                success: true,
                follows: Some(follows.into_iter().collect::<Vec<Follow>>()),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(ListFollowsResponse {
                success: false,
                follows: None,
                error: Some(format!(
                    "failed to retrieve follows due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn get_follows_by_id(
    State(pool): State<PgPool>,
    Path(user_name): Path<String>,
) -> impl IntoResponse {
    match FollowsService::get_by_id(&pool, user_name).await {
        Ok(follows) => (
            StatusCode::OK,
            Json(ListFollowsResponse {
                success: true,
                follows: Some(follows.into_iter().collect::<Vec<Follow>>()),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(ListFollowsResponse {
                success: false,
                follows: None,
                error: Some(format!(
                    "failed to retrieve a user's following due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn create_follow(
    State(pool): State<PgPool>,
    Json(create): Json<CreateFollow>,
) -> impl IntoResponse {
    match FollowsService::create(&pool, create).await {
        Ok(follow) => (
            StatusCode::CREATED,
            Json(FollowsResponse {
                success: true,
                follow: Some(follow),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(FollowsResponse {
                success: false,
                follow: None,
                error: Some(format!(
                    "failed to create follow due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn delete_follow(
    State(pool): State<PgPool>,
    Path(follower): Path<String>,
    Path(following): Path<String>,
) -> impl IntoResponse {
    match FollowsService::delete(&pool, follower, following).await {
        Ok(follow) => (
            StatusCode::OK,
            Json(FollowsResponse {
                success: true,
                follow: Some(follow),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(FollowsResponse {
                success: false,
                follow: None,
                error: Some(format!(
                    "failed to delete follow due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}
