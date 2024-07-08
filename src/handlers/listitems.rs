use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use http::StatusCode;
use serde::Serialize;
use sqlx::PgPool;

use crate::{
    models::listitems::{CreateListItem, ListItem, UpdateListItem},
    services::listitems::ListItemService,
};

#[derive(Debug, Serialize)]
struct ListItemResponse {
    success: bool,
    list_item: Option<ListItem>,
    error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListListItemResponse {
    pub success: bool,
    pub list_items: Option<Vec<ListItem>>,
    pub error: Option<String>,
}

pub async fn get_list_item(
    State(pool): State<PgPool>,
    Path((email, listname, item_id)): Path<(String, String, i32)>,
) -> impl IntoResponse {
    match ListItemService::get_list_item(&pool, email, listname, item_id).await {
        Ok(list_item) => (
            StatusCode::OK,
            Json(ListItemResponse {
                success: true,
                list_item: Some(list_item),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(ListItemResponse {
                success: false,
                list_item: None,
                error: Some(format!(
                    "failed to retrieve list item due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn create_list_item(
    State(pool): State<PgPool>,
    Json(create_obj): Json<CreateListItem>,
) -> impl IntoResponse {
    match ListItemService::create_list_item(&pool, create_obj).await {
        Ok(list_item) => (
            StatusCode::CREATED,
            Json(ListItemResponse {
                success: true,
                list_item: Some(list_item),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(ListItemResponse {
                success: false,
                list_item: None,
                error: Some(format!(
                    "failed to create list item with the given values due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn update_list_item(
    State(pool): State<PgPool>,
    Path((email, listname, item_id)): Path<(String, String, i32)>,
    Json(update_obj): Json<UpdateListItem>,
) -> impl IntoResponse {
    match ListItemService::update_list_item(&pool, update_obj, email, listname, item_id).await {
        Ok(list_item) => (
            StatusCode::OK,
            Json(ListItemResponse {
                success: true,
                list_item: Some(list_item),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(ListItemResponse {
                success: false,
                list_item: None,
                error: Some(format!(
                    "failed to update list item due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn delete_list_item(
    State(pool): State<PgPool>,
    Path((email, listname, item_id)): Path<(String, String, i32)>,
) -> impl IntoResponse {
    match ListItemService::delete_list_item(&pool, email, listname, item_id).await {
        Ok(list_item) => (
            StatusCode::OK,
            Json(ListItemResponse {
                success: true,
                list_item: Some(list_item),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(ListItemResponse {
                success: false,
                list_item: None,
                error: Some(format!(
                    "failed to delete list item due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}
