use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Game {
    pub id: i32,
    pub title: String,
    pub mediaimage: String,
    pub console: String,
    pub createdon: Option<NaiveDate>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GameSortKey {
    Title,
    Console,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct QueryGame {
    pub title: Option<String>,
    pub console: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub sort_key: Option<GameSortKey>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateGame {
    pub id: i32,
    pub title: String,
    pub media_image: String,
    pub console: String,
    pub created_on: NaiveDate,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateGame {
    pub title: Option<String>,
    pub media_image: Option<String>,
    pub console: Option<String>,
    pub created_on: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ErrorGame(pub String);

impl std::fmt::Display for ErrorGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
