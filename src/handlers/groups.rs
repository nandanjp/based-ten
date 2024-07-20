use std::sync::Arc;

use crate::models::groups::{CreateGroups, QueryGroups, UpdateGroups};
use crate::services::groups::GroupsService;
use crate::utils::response::{get_list_response, get_one_response};
use crate::AppState;
use axum::extract::{Json, Path, Query, State};
use axum::response::IntoResponse;
use http::StatusCode;

pub async fn get_all_groups(State(pool): State<Arc<AppState>>) -> impl IntoResponse {
    get_list_response(
        GroupsService::get_all(&pool.db).await.map_err(|e| {
            format!("failed to retrieve all groups due to the following error: {e:#?}")
        }),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn get_groups_by_id(
    State(pool): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    get_one_response(
        GroupsService::get_by_id(&pool.db, id)
            .await
            .map_err(|e| format!("failed to retrieve group due to the following error: {e:#?}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn get_circles_by_id(
    State(pool): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    get_list_response(
        GroupsService::get_circles_by_id(&pool.db, id)
            .await
            .map_err(|e| {
                format!("failed to retrieve circles by group id due to the following error: {e:#?}")
            }),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn create_groups(
    State(pool): State<Arc<AppState>>,
    Json(create): Json<CreateGroups>,
) -> impl IntoResponse {
    get_one_response(
        GroupsService::create(&pool.db, create)
            .await
            .map_err(|e| format!("failed to create group due to the following error: {e:#?}")),
        StatusCode::CREATED,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn update_groups(
    State(pool): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(update): Json<UpdateGroups>,
) -> impl IntoResponse {
    get_one_response(
        GroupsService::update(&pool.db, update, id)
            .await
            .map_err(|e| format!("failed to update group due to the following error: {e:#?}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn delete_groups(
    State(pool): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    get_one_response(
        GroupsService::delete(&pool.db, id)
            .await
            .map_err(|e| format!("failed to delete group due to the following error: {e:#?}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn get_group_member_lists(
    State(pool): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Query(order_by_author): Query<QueryGroups>,
) -> impl IntoResponse {
    get_list_response(
        GroupsService::get_member_lists(&pool.db, id, order_by_author)
            .await
            .map_err(|e| {
                format!("failed to retrieve group member lists due to the following error: {e:#?}")
            }),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}
