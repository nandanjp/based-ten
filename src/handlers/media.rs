use crate::models::media::{MediaSerial, QueryMedia};
use crate::services::media::MediaService;
use crate::utils::traits::IntoSerial;
use axum::extract::{Json, Query, State};
use axum::response::IntoResponse;
use http::StatusCode;
use serde::Serialize;
use sqlx::PgPool;

#[derive(Debug, Serialize)]
struct ListMediaResponse {
    success: bool,
    media: Option<Vec<MediaSerial>>,
    error: Option<String>,
}

pub async fn get_all_media(
    State(pool): State<PgPool>,
    Query(query): Query<QueryMedia>,
) -> impl IntoResponse {
    match MediaService::get_all(&pool, query).await {
        Ok(media) => (
            StatusCode::OK,
            Json(ListMediaResponse {
                success: true,
                media: Some(
                    media
                        .into_iter()
                        .map(|a| a.to_serial())
                        .collect::<Vec<MediaSerial>>(),
                ),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(ListMediaResponse {
                success: false,
                media: None,
                error: Some(format!(
                    "failed to retrieve all anime due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}
