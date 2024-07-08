use crate::models::groups::{GroupsQuery, GroupsSerial, CreateGroups, UpdateGroups};
use crate::services::groups::GroupsService;
use crate::utils::traits::{GeneralService, IntoSerial};
use axum::extract::{Json, Path, Query, State};
use axum::response::IntoResponse;
use http::StatusCode;
use serde::Serialize;
use sqlx::PgPool;

#[derive(Debug, Serialize)]
struct GroupsResponse {
    success: bool,
    groups: Option<GroupsSerial>,
    error: Option<String>,
}

#[derive(Debug, Serialize)]
struct ListGroupsResponse {
    success: bool,
    groups: Option<Vec<GroupsSerial>>,
    error: Option<String>,
}

pub async fn get_all_groups(
    State(pool): State<PgPool>,
    Query(query): Query<GroupsQuery>,
) -> impl IntoResponse {
    match GroupsService::get_all(&pool, query).await {
        Ok(groups) => (
            StatusCode::OK,
            Json(ListGroupsResponse {
                success: true,
                groups: Some(
                    groups
                        .into_iter()
                        .map(|a| a.to_serial())
                        .collect::<Vec<GroupsSerial>>(),
                ),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(ListGroupsResponse {
                success: false,
                groups: None,
                error: Some(format!(
                    "failed to retrieve all groups due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn get_groups_by_id(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    match GroupsService::get_by_id(&pool, id).await {
        Ok(groups) => (
            StatusCode::OK,
            Json(GroupsResponse {
                success: true,
                groups: Some(groups.to_serial()),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(GroupsResponse {
                success: false,
                groups: None,
                error: Some(format!(
                    "failed to retrieve an groups due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn create_groups(
    State(pool): State<PgPool>,
    Json(create): Json<CreateGroups>,
) -> impl IntoResponse {
    match GroupsService::create(&pool, create).await {
        Ok(groups) => (
            StatusCode::CREATED,
            Json(GroupsResponse {
                success: true,
                groups: Some(groups.to_serial()),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(GroupsResponse {
                success: false,
                groups: None,
                error: Some(format!(
                    "failed to create groups due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn update_groups(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(update): Json<UpdateGroups>,
) -> impl IntoResponse {
    match GroupsService::update(&pool, update, id).await {
        Ok(groups) => (
            StatusCode::OK,
            Json(GroupsResponse {
                success: true,
                groups: Some(groups.to_serial()),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(GroupsResponse {
                success: false,
                groups: None,
                error: Some(format!(
                    "failed to update groups due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}

pub async fn delete_groups(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    match GroupsService::delete(&pool, id).await {
        Ok(groups) => (
            StatusCode::OK,
            Json(GroupsResponse {
                success: true,
                groups: Some(groups.to_serial()),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(GroupsResponse {
                success: false,
                groups: None,
                error: Some(format!(
                    "failed to delete groups due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}
