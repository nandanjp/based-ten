use crate::services::followmutual::FollowMutualService;
use crate::utils::response::get_list_response;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use http::StatusCode;
use sqlx::PgPool;

pub async fn get_mutual_follows_by_id(
    State(pool): State<PgPool>,
    Path(follower_email): Path<String>,
) -> impl IntoResponse {
    get_list_response(FollowMutualService::get_by_mutual_follower(&pool, follower_email).await.map_err(|e| format!("failed to retrieve a user's mutual followers due to the following error: {e:#?}")), StatusCode::OK, StatusCode::BAD_REQUEST)
}
