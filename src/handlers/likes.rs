use std::sync::Arc;

use crate::models::likes::{CreateLike, QueryLike};
use crate::models::users::User;
use crate::services::likes::LikesService;
use crate::utils::response::{get_list_response, get_one_response};
use crate::AppState;
use axum::extract::{Json, Path, Query, State};
use axum::response::IntoResponse;
use axum::Extension;
use http::StatusCode;

pub async fn get_all_likes(
    State(pool): State<Arc<AppState>>,
    Query(query): Query<QueryLike>,
) -> impl IntoResponse {
    get_list_response(
        LikesService::get_all(&pool.db, query)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn get_likes_by_id(
    State(pool): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    get_list_response(
        LikesService::get_by_id(&pool.db, user.username)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn create_like(
    State(pool): State<Arc<AppState>>,
    Json(create): Json<CreateLike>,
) -> impl IntoResponse {
    get_one_response(
        LikesService::create(&pool.db, create)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::CREATED,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn delete_user_likes(
    State(pool): State<Arc<AppState>>,
    Path((liking, list_name)): Path<(String, String)>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    get_one_response(
        LikesService::delete(&pool.db, user.username, liking, list_name)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

pub async fn delete_like(
    State(pool): State<Arc<AppState>>,
    Path((liker, liking, list_name)): Path<(String, String, String)>,
) -> impl IntoResponse {
    get_one_response(
        LikesService::delete(&pool.db, liker, liking, list_name)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}
