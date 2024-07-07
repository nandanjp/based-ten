use std::string;

use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use http::StatusCode;
use serde::Serialize;
use sqlx::PgPool;

use crate::{
    models::lists::{CreateList, UpdateList, ErrorList, List, QueryList},
    services::lists::ListService,
    utils::traits::{IntoSerial},
};

#[derive(Debug, Serialize)]
struct ListResponse {
    success: bool,
    list: Option<List>,
    error: Option<String>,
}

#[derive(Debug, Serialize)]
struct ListListResponse {
    success: bool,
    lists: Option<Vec<List>>,
    error: Option<String>,
}

pub async fn get_all_lists(
    State(pool): State<PgPool>,
    Query(query): Query<QueryList>,
) -> impl IntoResponse {
    match ListService::get_all(&pool, query).await {
        Ok(lists) => (
            StatusCode::OK,
            Json(ListListResponse {
                success: true,
                lists: Some(
                    lists
                        .into_iter()
                        // .map(|m| m.to_serial())
                        .collect::<Vec<List>>(),
                ),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(ListListResponse {
                success: false,
                lists: None,
                error: Some(format!(
                    "failed to retrieve all lists due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn get_list_by_id(State(pool): State<PgPool>, Path(email): Path<String>, Path(listname): Path<String>) -> impl IntoResponse {
    match ListService::get_by_id(&pool, email, listname).await {
        Ok(list) => (
            StatusCode::OK,
            Json(ListResponse {
                success: true,
                list: Some(list),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(ListResponse {
                success: false,
                list: None,
                error: Some(format!(
                    "failed to retrieve list due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn create_list(
    State(pool): State<PgPool>,
    Json(create): Json<CreateList>,
) -> impl IntoResponse {
    match ListService::create(&pool, create).await {
        Ok(list) => (
            StatusCode::OK,
            Json(ListResponse {
                success: true,
                list: Some(list),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(ListResponse {
                success: false,
                list: None,
                error: Some(format!(
                    "failed to create list with given details due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn update_list(
    State(pool): State<PgPool>,
    Path(email): Path<String>, 
    Path(listname): Path<String>,
    Json(update): Json<UpdateList>,
) -> impl IntoResponse {
    match ListService::update(&pool, update, email, listname).await {
        Ok(list) => (
            StatusCode::OK,
            Json(ListResponse {
                success: true,
                list: Some(list),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(ListResponse {
                success: false,
                list: None,
                error: Some(format!(
                    "failed to update list with given details due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn delete_list(State(pool): State<PgPool>, Path(email): Path<String>, Path(listname): Path<String>) -> impl IntoResponse {
    match ListService::delete(&pool, email, listname).await {
        Ok(list) => (
            StatusCode::OK,
            Json(ListResponse {
                success: true,
                list: Some(list),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(ListResponse {
                success: false,
                list: None,
                error: Some(format!(
                    "failed to update list with given details due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}
