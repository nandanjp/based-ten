use std::sync::Arc;

use crate::models::follows::{CreateFollow, QueryFollow};
use crate::models::users::User;
use crate::services::follows::FollowsService;
use crate::utils::response::{get_list_response, get_one_response};
use crate::AppState;
use axum::extract::{Json, Path, Query, State};
use axum::response::IntoResponse;
use axum::Extension;
use http::StatusCode;

pub async fn get_all_follows(
    State(pool): State<Arc<AppState>>,
    Query(query): Query<QueryFollow>,
) -> impl IntoResponse {
    get_list_response(
        FollowsService::get_all(&pool.db, query)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn get_follows_by_id(
    State(pool): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    get_list_response(
        FollowsService::get_by_id(&pool.db, user.username)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn create_follow(
    State(pool): State<Arc<AppState>>,
    Json(create): Json<CreateFollow>,
) -> impl IntoResponse {
    get_one_response(
        FollowsService::create(&pool.db, create)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::CREATED,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn delete_follow(
    State(pool): State<Arc<AppState>>,
    Path(following): Path<String>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    get_one_response(
        FollowsService::delete(&pool.db, user.username, following)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn delete_follows(
    State(pool): State<Arc<AppState>>,
    Path((follower, following)): Path<(String, String)>,
) -> impl IntoResponse {
    get_one_response(
        FollowsService::delete(&pool.db, follower, following)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}
