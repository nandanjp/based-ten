use std::sync::Arc;

use crate::{
    models::{
        lists::{CreateList, QueryList, UpdateList},
        users::User,
    },
    services::lists::ListService,
    utils::response::{get_list_response, get_one_response},
    AppState,
};
use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Extension, Json,
};
use http::StatusCode;

pub async fn get_all_lists(
    State(pool): State<Arc<AppState>>,
    Query(query): Query<QueryList>,
) -> impl IntoResponse {
    get_list_response(
        ListService::get_all(&pool.db, query)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn get_user_lists(
    State(pool): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    get_list_response(
        ListService::get_by_email(&pool.db, user.username)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn get_user_list(
    State(pool): State<Arc<AppState>>,
    Path(list_name): Path<String>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    get_one_response(
        ListService::get_by_user_and_listname(&pool.db, user.username, list_name)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn get_user_list_items(
    State(pool): State<Arc<AppState>>,
    Path(list_name): Path<String>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    match ListService::get_user_list_and_items(&pool.db, user.username, list_name)
        .await
        .map_err(|e| format!("{e}"))
    {
        Ok(result) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "success": true,
                "response": result,
                "error": false
            })),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "success": false,
                "response": "None",
                "error": err
            })),
        ),
    }
}

pub async fn get_user_explore_lists(
    State(pool): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    get_list_response(
        ListService::get_explore_lists(&pool.db, user.username)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn get_some_top_lists(
    State(pool): State<Arc<AppState>>,
    Query(query): Query<QueryList>,
) -> impl IntoResponse {
    get_list_response(
        ListService::get_top_lists(&pool.db, query)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn create_list(
    State(pool): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Json(create): Json<CreateList>,
) -> impl IntoResponse {
    get_one_response(
        ListService::create(&pool.db, user.username, create)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::CREATED,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn update_list(
    State(pool): State<Arc<AppState>>,
    Path(list_name): Path<String>,
    Extension(user): Extension<User>,
    Json(update): Json<UpdateList>,
) -> impl IntoResponse {
    get_one_response(
        ListService::update(&pool.db, update, user.username, list_name)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn delete_list(
    State(pool): State<Arc<AppState>>,
    Path(list_name): Path<String>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    get_one_response(
        ListService::delete(&pool.db, user.username, list_name)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}
