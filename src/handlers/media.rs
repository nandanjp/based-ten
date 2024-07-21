use std::sync::Arc;

use crate::models::media::QueryMedia;
use crate::services::media::MediaService;
use crate::utils::response::get_list_response;
use crate::AppState;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use http::StatusCode;

pub async fn get_all_media(
    State(pool): State<Arc<AppState>>,
    Query(query): Query<QueryMedia>,
) -> impl IntoResponse {
    get_list_response(
        MediaService::get_all(&pool.db, query)
            .await
            .map_err(|e| format!("{e}")),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}
