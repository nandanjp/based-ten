use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::lists::ListType;

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct Media {
    pub id: Option<i32>,
    pub title: Option<String>,
    pub mediaimage: Option<String>,
    pub createdon: Option<NaiveDate>,
    #[serde(rename = "type")]
    pub typ: Option<ListType>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MediaSortKey {
    Title,
    Type,
}

#[derive(Debug, Clone, Deserialize)]
pub struct QueryMedia {
    pub media_type: Option<ListType>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub sort_key: Option<MediaSortKey>,
}

#[derive(Debug, Clone)]
pub struct MediaError(pub String);

impl std::fmt::Display for MediaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
