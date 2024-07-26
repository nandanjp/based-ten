use std::sync::Arc;

use crate::models::groups::{CreateGroups, QueryGroups};
use crate::models::users::User;
use crate::services::groups::GroupsService;
use crate::utils::response::{get_list_response, get_one_response};
use crate::AppState;
use axum::extract::{Json, Path, Query, State};
use axum::response::IntoResponse;
use axum::Extension;
use http::StatusCode;

pub async fn get_all_groups(State(pool): State<Arc<AppState>>) -> impl IntoResponse {
    get_list_response(
        GroupsService::get_all(&pool.db)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn get_all_groups_and_members(State(pool): State<Arc<AppState>>) -> impl IntoResponse {
    get_list_response(
        GroupsService::get_groups_and_members(&pool.db)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn get_user_groups(
    State(pool): State<Arc<AppState>>,
    Path(user_name): Path<String>,
) -> impl IntoResponse {
    get_list_response(
        GroupsService::get_user_groups(&pool.db, user_name)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn get_user_member_groups(
    State(pool): State<Arc<AppState>>,
    Path(user_name): Path<String>,
) -> impl IntoResponse {
    get_list_response(
        GroupsService::get_user_member_groups(&pool.db, user_name)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn get_group_by_id(
    State(pool): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    get_one_response(
        GroupsService::get_by_id(&pool.db, id)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn get_group_members(
    State(pool): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    get_one_response(
        GroupsService::get_members(&pool.db, id)
            .await
            .map_err(|e| format!("{e}")),
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
            .map_err(|e| format!("{e}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn create_user_group(
    State(pool): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Json(create): Json<CreateGroups>,
) -> impl IntoResponse {
    get_one_response(
        GroupsService::create(&pool.db, user.username, create)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::CREATED,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn delete_group_by_id(
    State(pool): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    get_one_response(
        GroupsService::delete(&pool.db, id)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn delete_user_group(
    State(pool): State<Arc<AppState>>,
    Path(group_name): Path<String>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    get_one_response(
        GroupsService::delete_user_group(&pool.db, user.username, group_name)
            .await
            .map_err(|e| format!("{e}")),
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
            .map_err(|e| format!("{e}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn join_group(
    State(pool): State<Arc<AppState>>,
    Path(gid): Path<i32>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    get_one_response(
        GroupsService::join_group(&pool.db, user.username, gid)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::CREATED,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn unjoin_group(
    State(pool): State<Arc<AppState>>,
    Path(gid): Path<i32>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    get_one_response(
        GroupsService::unjoin_group(&pool.db, user.username, gid)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::CREATED,
        StatusCode::BAD_REQUEST,
    )
}
