use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Song {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub album: String,
    pub mediaimage: String,
    pub createdon: Option<NaiveDate>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SongSortKey {
    Author,
    Title,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct QuerySong {
    pub title: Option<String>,
    pub author: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    #[serde(rename = "sort_by")]
    pub sort_key: Option<SongSortKey>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateSong {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub album: String,
    pub media_image: String,
    pub created_on: NaiveDate,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateSong {
    pub title: Option<String>,
    pub author: Option<String>,
    pub media_image: Option<String>,
    pub created_on: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ErrorSong(pub String);

impl std::fmt::Display for ErrorSong {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
