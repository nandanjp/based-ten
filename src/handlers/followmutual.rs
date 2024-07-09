use crate::models::followmutual::FollowMutual;
use crate::services::followmutual::FollowMutualService;
use axum::extract::{Json, Path, State};
use axum::response::IntoResponse;
use http::StatusCode;
use serde::Serialize;
use sqlx::PgPool;

#[derive(Debug, Serialize)]
struct FollowMutualResponse {
    success: bool,
    followmutual: Option<FollowMutual>,
    error: Option<String>,
}

#[derive(Debug, Serialize)]
struct ListFollowMutualResponse {
    success: bool,
    followmutuals: Option<Vec<FollowMutual>>,
    error: Option<String>,
}

pub async fn get_mutual_follows_by_id(
    State(pool): State<PgPool>,
    Path(follower_email): Path<String>,
) -> impl IntoResponse {
    match FollowMutualService::get_by_mutual_follower(&pool, follower_email).await {
        Ok(followmutual) => (
            StatusCode::OK,
            Json(ListFollowMutualResponse {
                success: true,
                followmutuals: Some(
                    followmutual
                        .into_iter()
                        .collect::<Vec<FollowMutual>>(),
                ),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(ListFollowMutualResponse {
                success: false,
                followmutuals: None,
                error: Some(format!(
                    "failed to retrieve a user's mutual followers due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}
