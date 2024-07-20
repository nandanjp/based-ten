use std::sync::Arc;

use crate::models::users::{CreateUser, QueryUser, UpdateUser};
use crate::services::users::UsersService;
use crate::utils::response::{get_list_response, get_one_response};
use crate::AppState;
use axum::extract::{Json, Path, Query, State};
use axum::response::IntoResponse;
use http::StatusCode;

pub async fn get_all_users(
    State(pool): State<Arc<AppState>>,
    Query(query): Query<QueryUser>,
) -> impl IntoResponse {
    get_list_response(
        UsersService::get_all(&pool.db, query).await.map_err(|e| {
            format!("failed to retrieve all users due to the following error: {e:#?}")
        }),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn get_user_by_id(
    State(pool): State<Arc<AppState>>,
    Path(user_name): Path<String>,
) -> impl IntoResponse {
    get_one_response(
        UsersService::get_by_id(&pool.db, user_name)
            .await
            .map_err(|e| format!("failed to retrieve a user due to the following error: {e:#?}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn create_user(
    State(pool): State<Arc<AppState>>,
    Json(create): Json<CreateUser>,
) -> impl IntoResponse {
    get_one_response(
        UsersService::create(&pool.db, create).await.map_err(|e| {
            format!(
                "failed to create a user with the given details due to the following error: {e:#?}"
            )
        }),
        StatusCode::CREATED,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn update_user(
    State(pool): State<Arc<AppState>>,
    Path(user_name): Path<String>,
    Json(update): Json<UpdateUser>,
) -> impl IntoResponse {
    get_one_response(
        UsersService::update(&pool.db, update, user_name)
            .await
            .map_err(|e| {
                format!(
                "failed to update a user with the given details due to the following error: {e:#?}"
            )
            }),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn delete_user(
    State(pool): State<Arc<AppState>>,
    Path(user_name): Path<String>,
) -> impl IntoResponse {
    get_one_response(
        UsersService::delete(&pool.db, user_name)
            .await
            .map_err(|e| format!("failed to delete user due to the following error: {e:#?}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}
