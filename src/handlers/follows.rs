use crate::models::follows::{CreateFollow, QueryFollow};
use crate::services::follows::FollowsService;
use crate::utils::response::{get_list_response, get_one_response};
use axum::extract::{Json, Path, Query, State};
use axum::response::IntoResponse;
use http::StatusCode;
use sqlx::PgPool;

pub async fn get_all_follows(
    State(pool): State<PgPool>,
    Query(query): Query<QueryFollow>,
) -> impl IntoResponse {
    get_list_response(
        FollowsService::get_all(&pool, query)
            .await
            .map_err(|e| format!("failed to retrieve follows due to the following error: {e:#?}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn get_follows_by_id(
    State(pool): State<PgPool>,
    Path(user_name): Path<String>,
) -> impl IntoResponse {
    get_list_response(
        FollowsService::get_by_id(&pool, user_name)
            .await
            .map_err(|e| {
                format!("failed to retrieve a user's following due to the following error: {e:#?}")
            }),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn create_follow(
    State(pool): State<PgPool>,
    Json(create): Json<CreateFollow>,
) -> impl IntoResponse {
    get_one_response(
        FollowsService::create(&pool, create)
            .await
            .map_err(|e| format!("failed to create follow due to the following error: {e:#?}")),
        StatusCode::CREATED,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn delete_follow(
    State(pool): State<PgPool>,
    Path(follower): Path<String>,
    Path(following): Path<String>,
) -> impl IntoResponse {
    get_one_response(
        FollowsService::delete(&pool, follower, following)
            .await
            .map_err(|e| format!("failed to delete follow due to the following error: {e:#?}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}
