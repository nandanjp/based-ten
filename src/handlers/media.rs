use crate::models::media::{self, MediaSerial};
use crate::services::media::MediaService;
use crate::utils::traits::{IntoSerial};
use axum::extract::{Json, State};
use axum::response::IntoResponse;
use http::StatusCode;
use serde::Serialize;
use sqlx::PgPool;

#[derive(Debug, Serialize)]
struct MediaResponse {
    success: bool,
    media: Option<MediaSerial>,
    error: Option<String>,
}

#[derive(Debug, Serialize)]
struct ListMediaResponse {
    success: bool,
    media: Option<Vec<MediaSerial>>,
    error: Option<String>,
}

pub async fn get_all_media(
    State(pool): State<PgPool>
) -> impl IntoResponse {
    match MediaService::get_all(&pool).await {
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
