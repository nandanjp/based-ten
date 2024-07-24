use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct Anime {
    pub id: i32,
    pub title: String,
    pub mediaimage: String,
    pub numepisodes: i32,
    pub createdon: Option<NaiveDate>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AnimeSortKey {
    NumEpisodes,
    Title,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AnimeQuery {
    pub title: Option<String>,
    pub num_episodes: Option<i32>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    #[serde(rename = "sort_by")]
    pub sort_key: Option<AnimeSortKey>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateAnime {
    pub id: i32,
    pub title: String,
    pub media_image: String,
    pub num_episodes: i32,
    pub created_on: NaiveDate,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateAnime {
    pub title: Option<String>,
    pub media_image: Option<String>,
    pub num_episodes: Option<i32>,
    pub created_on: Option<NaiveDate>,
}

#[derive(Debug, Clone)]
pub struct AnimeError(pub String);

impl std::fmt::Display for AnimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
