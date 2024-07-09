use crate::models::groups::{Group, CreateGroups, UpdateGroups};
use crate::models::lists::List;
use crate::services::groups::GroupsService;
use axum::extract::{Json, Path, State};
use axum::response::IntoResponse;
use http::StatusCode;
use serde::Serialize;
use sqlx::PgPool;

#[derive(Debug, Serialize)]
struct GroupsResponse {
    success: bool,
    groups: Option<Group>,
    error: Option<String>,
}

#[derive(Debug, Serialize)]
struct ListGroupsResponse {
    success: bool,
    groups: Option<Vec<Group>>,
    error: Option<String>,
}

#[derive(Debug, Serialize)]
struct GroupMemberListsResponse {
    success: bool,
    memberlists: Option<Vec<List>>,
    error: Option<String>,
}

pub async fn get_all_groups(
    State(pool): State<PgPool>,
) -> impl IntoResponse {
    match GroupsService::get_all(&pool).await {
        Ok(groups) => (
            StatusCode::OK,
            Json(ListGroupsResponse {
                success: true,
                groups: Some(
                    groups
                        .into_iter()
                        .collect::<Vec<Group>>(),
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
                groups: Some(groups),
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
                groups: Some(groups),
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
                groups: Some(groups),
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
                groups: Some(groups),
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

pub async fn get_group_member_lists(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    match GroupsService::get_member_lists(&pool, id).await {
        Ok(lists) => (
            StatusCode::OK,
            Json(GroupMemberListsResponse {
                success: true,
                memberlists: Some(lists),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(GroupMemberListsResponse {
                success: false,
                memberlists: None,
                error: Some(format!(
                    "failed to retrieve group member lists due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}
