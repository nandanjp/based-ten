use crate::models::users::{CreateUser, QueryUser, UpdateUser, UserSerial};
use crate::services::users::UsersService;
use crate::utils::traits::IntoSerial;
use axum::extract::{Json, Path, Query, State};
use axum::response::IntoResponse;
use http::StatusCode;
use serde::Serialize;
use sqlx::PgPool;

#[derive(Debug, Serialize)]
struct UsersResponse {
    success: bool,
    user: Option<UserSerial>,
    error: Option<String>,
}

#[derive(Debug, Serialize)]
struct ListUsersResponse {
    success: bool,
    user: Option<Vec<UserSerial>>,
    error: Option<String>,
}

pub async fn get_all_users(
    State(pool): State<PgPool>,
    Query(query): Query<QueryUser>,
) -> impl IntoResponse {
    match UsersService::get_all(&pool, query).await {
        Ok(users) => (
            StatusCode::OK,
            Json(ListUsersResponse {
                success: true,
                user: Some(
                    users
                        .into_iter()
                        .map(|a| a.to_serial())
                        .collect::<Vec<UserSerial>>(),
                ),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(ListUsersResponse {
                success: false,
                user: None,
                error: Some(format!(
                    "failed to retrieve all users due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn get_user_by_id(
    State(pool): State<PgPool>,
    Path(user_name): Path<String>,
) -> impl IntoResponse {
    match UsersService::get_by_id(&pool, user_name).await {
        Ok(user) => (
            StatusCode::OK,
            Json(UsersResponse {
                success: true,
                user: Some(user.to_serial()),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(UsersResponse {
                success: false,
                user: None,
                error: Some(format!(
                    "failed to retrieve a user due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn create_user(
    State(pool): State<PgPool>,
    Json(create): Json<CreateUser>,
) -> impl IntoResponse {
    match UsersService::create(&pool, create).await {
        Ok(user) => (
            StatusCode::CREATED,
            Json(UsersResponse {
                success: true,
                user: Some(user.to_serial()),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(UsersResponse {
                success: false,
                user: None,
                error: Some(format!(
                    "failed to create user due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn update_user(
    State(pool): State<PgPool>,
    Path(user_name): Path<String>,
    Json(update): Json<UpdateUser>,
) -> impl IntoResponse {
    match UsersService::update(&pool, update, user_name).await {
        Ok(user) => (
            StatusCode::OK,
            Json(UsersResponse {
                success: true,
                user: Some(user.to_serial()),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(UsersResponse {
                success: false,
                user: None,
                error: Some(format!(
                    "failed to update user due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn delete_user(
    State(pool): State<PgPool>,
    Path(user_name): Path<String>,
) -> impl IntoResponse {
    match UsersService::delete(&pool, user_name).await {
        Ok(user) => (
            StatusCode::OK,
            Json(UsersResponse {
                success: true,
                user: Some(user.to_serial()),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(UsersResponse {
                success: false,
                user: None,
                error: Some(format!(
                    "failed to delete user due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}
