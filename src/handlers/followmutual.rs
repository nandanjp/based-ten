use std::sync::Arc;

use crate::models::users::User;
use crate::services::followmutual::FollowMutualService;
use crate::utils::response::get_list_response;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Extension;
use http::StatusCode;

pub async fn get_mutual_follows_by_id(
    State(pool): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    get_list_response(
        FollowMutualService::get_by_mutual_follower(&pool.db, user.username)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}
