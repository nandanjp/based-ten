use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct Movie {
    pub id: i32,
    pub title: String,
    pub mediaimage: String,
    pub createdon: Option<NaiveDate>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MovieSortKey {
    Title,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct QueryMovie {
    pub title: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub sort_key: Option<MovieSortKey>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateMovie {
    pub id: i32,
    pub title: String,
    pub media_image: String,
    pub created_on: NaiveDate,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateMovie {
    pub title: Option<String>,
    pub media_image: Option<String>,
    pub created_on: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ErrorMovie(pub String);

impl std::fmt::Display for ErrorMovie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
