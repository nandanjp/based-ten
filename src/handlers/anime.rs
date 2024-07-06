use crate::models::anime::{Anime, AnimeSerial};
use axum::extract::{Json, Path, Query, State};
use axum::response::IntoResponse;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Serialize)]
struct AnimeResponse {
    success: bool,
    anime: Option<AnimeSerial>,
    error: Option<String>,
}

#[derive(Debug, Serialize)]
struct ListAnimeResponse {
    success: bool,
    anime: Option<Vec<AnimeSerial>>,
    error: Option<String>,
}

#[derive(Debug, Clone)]
struct AnimeError(String);
impl std::fmt::Display for AnimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "failed to retrieve anime due to the following error: {:#?}",
            self.0
        )
    }
}

struct AnimeService;

impl AnimeService {
    async fn get_all_anime(pool: &sqlx::PgPool) -> Result<Vec<Anime>, AnimeError> {
        sqlx::query!(r#"SELECT id, title, mediaimage, numepisodes, createdon FROM Anime"#)
            .fetch_all(pool)
            .await
            .map(|a| {
                a.into_iter()
                    .map(|a| Anime {
                        id: a.id,
                        title: a.title,
                        media_image: a.mediaimage.unwrap(),
                        num_episodes: a.numepisodes.unwrap(),
                        created_on: a.createdon.unwrap(),
                    })
                    .collect::<Vec<Anime>>()
            })
            .map_err(|e| {
                AnimeError(format!(
                    "failed to retrieve all anime due to the following error: {e:#?}"
                ))
            })
    }
}

pub async fn get_all_anime(State(pool): State<PgPool>) -> impl IntoResponse {
    match AnimeService::get_all_anime(&pool).await {
        Ok(anime) => (
            StatusCode::OK,
            Json(ListAnimeResponse {
                success: true,
                anime: Some(
                    anime
                        .into_iter()
                        .map(|a| a.to_serial())
                        .collect::<Vec<AnimeSerial>>(),
                ),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(ListAnimeResponse {
                success: false,
                anime: None,
                error: Some(format!(
                    "failed to retrieve all anime due to the following error: {err:#?}"
                )),
            }),
        ),
    }
}
