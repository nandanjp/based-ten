use std::sync::Arc;

use crate::models::users::{CreateUser, QueryUser, UpdateUser, User};
use crate::services::users::UsersService;
use crate::utils::response::{get_list_response, get_one_response};
use crate::AppState;
use axum::extract::{Json, Query, State};
use axum::response::IntoResponse;
use axum::Extension;
use http::StatusCode;

pub async fn get_all_users(
    State(pool): State<Arc<AppState>>,
    Query(query): Query<QueryUser>,
) -> impl IntoResponse {
    get_list_response(
        UsersService::get_all(&pool.db, query)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn get_user_by_username(
    State(pool): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    get_one_response(
        UsersService::get_by_id(&pool.db, user.username)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn create_user(
    State(pool): State<Arc<AppState>>,
    Json(create): Json<CreateUser>,
) -> impl IntoResponse {
    get_one_response(
        UsersService::create(&pool.db, create)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::CREATED,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn update_user(
    State(pool): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Json(update): Json<UpdateUser>,
) -> impl IntoResponse {
    get_one_response(
        UsersService::update(&pool.db, update, user.username)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn delete_user(
    State(pool): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    get_one_response(
        UsersService::delete(&pool.db, user.username)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}
