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
        MediaService::get_all(&pool.db, query).await.map_err(|e| {
            format!("failed to retrieve all media due to the following error: {e:#?}")
        }),
        StatusCode::OK,
        StatusCode::BAD_REQUEST,
    )
}

/*
pub async fn get_media_by_type(
    State(pool): State<PgPool>,
    Path(path): Path<String>,
) -> impl IntoResponse {
    match MediaService::get_by_type(&pool, path).await {
        Ok(media) => (
            StatusCode::OK,
            Json(ListMediaWithLikesResponse {
                success: true,
                media: Some(
                    media
                        .into_iter()
                        .map(|a| a.to_serial())
                        .collect::<Vec<MediaWithLikesSerial>>(),
                ),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(ListMediaWithLikesResponse {
                success: false,
                media: None,
                error: Some(format!(
                    "failed to retrieve media by type due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}
 */
