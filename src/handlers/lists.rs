use crate::{
    models::lists::{CreateList, QueryList, UpdateList},
    services::lists::ListService,
    utils::response::{get_list_response, get_one_response},
};
use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use http::StatusCode;
use sqlx::PgPool;

pub async fn get_all_lists(
    State(pool): State<PgPool>,
    Query(query): Query<QueryList>,
) -> impl IntoResponse {
    get_list_response(
        ListService::get_all(&pool, query).await.map_err(|e| {
            format!("failed to retrieve all lists due to the following error: {e:#?}")
        }),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn get_user_lists(
    State(pool): State<PgPool>,
    Path(user_name): Path<String>,
) -> impl IntoResponse {
    get_list_response(
        ListService::get_by_email(&pool, user_name)
            .await
            .map_err(|e| {
                format!("failed to retrieve user's lists due to the following error: {e:#?}")
            }),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn get_user_list(
    State(pool): State<PgPool>,
    Path((user_name, list_name)): Path<(String, String)>,
) -> impl IntoResponse {
    get_one_response(
        ListService::get_by_user_and_listname(&pool, user_name, list_name)
            .await
            .map_err(|e| format!("failed to retrieve list due to the following error: {e:#?}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn get_user_list_items(
    State(pool): State<PgPool>,
    Path((user_name, list_name)): Path<(String, String)>,
) -> impl IntoResponse {
    get_list_response(
        ListService::get_user_list_and_items(&pool, user_name, list_name)
            .await
            .map_err(|e| {
                format!("failed to retrieve all list items due to the following error: {e:#?}")
            }),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn get_user_explore_lists(
    State(pool): State<PgPool>,
    Path(user_name): Path<String>,
) -> impl IntoResponse {
    get_list_response(
        ListService::get_explore_lists(&pool, user_name)
            .await
            .map_err(|e| {
                format!("failed to retrieve user's Explore (recommended) lists due to the following error: {e:#?}")
            }),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn create_list(
    State(pool): State<PgPool>,
    Json(create): Json<CreateList>,
) -> impl IntoResponse {
    get_one_response(
        ListService::create(&pool, create).await.map_err(|e| {
            format!("failed to create list with given details due to the following error: {e:#?}")
        }),
        StatusCode::CREATED,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn update_list(
    State(pool): State<PgPool>,
    Path((user_name, list_name)): Path<(String, String)>,
    Json(update): Json<UpdateList>,
) -> impl IntoResponse {
    get_one_response(
        ListService::update(&pool, update, user_name, list_name)
            .await
            .map_err(|e| {
                format!(
                    "failed to update list with given details due to the following error: {e:#?}"
                )
            }),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn delete_list(
    State(pool): State<PgPool>,
    Path((user_name, list_name)): Path<(String, String)>,
) -> impl IntoResponse {
    get_one_response(
        ListService::delete(&pool, user_name, list_name)
            .await
            .map_err(|e| format!("failed to delete list to the following error: {e:#?}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}
