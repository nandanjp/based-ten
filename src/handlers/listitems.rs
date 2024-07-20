use std::sync::Arc;

use crate::{
    models::listitems::{CreateListItem, UpdateListItem},
    services::listitems::ListItemService,
    utils::response::get_one_response,
    AppState,
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use http::StatusCode;

pub async fn get_list_item(
    State(pool): State<Arc<AppState>>,
    Path((user_name, list_name, item_id)): Path<(String, String, i32)>,
) -> impl IntoResponse {
    get_one_response(
        ListItemService::get_list_item(&pool.db, user_name, list_name, item_id)
            .await
            .map_err(|e| {
                format!("failed to retrieve list items due to the following error: {e:#?}")
            }),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn create_list_item(
    State(pool): State<Arc<AppState>>,
    Json(create_obj): Json<CreateListItem>,
) -> impl IntoResponse {
    get_one_response(ListItemService::create_list_item(&pool.db, create_obj).await.map_err(|e| format!("failed to create list item with the given values due to the following error: {e:#?}")), StatusCode::CREATED, StatusCode::BAD_REQUEST)
}

pub async fn update_list_item(
    State(pool): State<Arc<AppState>>,
    Path((user_name, list_name, item_id)): Path<(String, String, i32)>,
    Json(update_obj): Json<UpdateListItem>,
) -> impl IntoResponse {
    get_one_response(ListItemService::update_list_item(&pool.db, update_obj, user_name, list_name, item_id).await.map_err(|e| format!("failed to update list item with the given values due to the following error: {e:#?}")), StatusCode::CREATED, StatusCode::BAD_REQUEST)
}

pub async fn delete_list_item(
    State(pool): State<Arc<AppState>>,
    Path((user_name, list_name, item_id)): Path<(String, String, i32)>,
) -> impl IntoResponse {
    get_one_response(
        ListItemService::delete_list_item(&pool.db, user_name, list_name, item_id)
            .await
            .map_err(|e| format!("failed to delete list item due to the following error: {e:#?}")),
        StatusCode::CREATED,
        StatusCode::BAD_REQUEST,
    )
}
