use std::sync::Arc;

use crate::{
    models::{
        listitems::{CreateListItem, UpdateListItem},
        users::User,
    },
    services::listitems::ListItemService,
    utils::response::get_one_response,
    AppState,
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Extension, Json,
};
use http::StatusCode;

pub async fn get_list_item(
    State(pool): State<Arc<AppState>>,
    Path((list_name, item_id)): Path<(String, i32)>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    get_one_response(
        ListItemService::get_list_item(&pool.db, user.username, list_name, item_id)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn create_list_item(
    State(pool): State<Arc<AppState>>,
    Json(create_obj): Json<CreateListItem>,
) -> impl IntoResponse {
    get_one_response(
        ListItemService::create_list_item(&pool.db, create_obj)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::CREATED,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn update_list_item(
    State(pool): State<Arc<AppState>>,
    Path((list_name, item_id)): Path<(String, i32)>,
    Extension(user): Extension<User>,
    Json(update_obj): Json<UpdateListItem>,
) -> impl IntoResponse {
    get_one_response(
        ListItemService::update_list_item(&pool.db, update_obj, user.username, list_name, item_id)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::CREATED,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn delete_list_item(
    State(pool): State<Arc<AppState>>,
    Path((list_name, item_id)): Path<(String, i32)>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    get_one_response(
        ListItemService::delete_list_item(&pool.db, user.username, list_name, item_id)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::CREATED,
        StatusCode::BAD_REQUEST,
    )
}
