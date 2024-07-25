use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::listitems::ListItem;

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct List {
    pub username: String,
    pub listname: String,
    pub listtype: ListType,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ListWithLikes {
    pub username: String,
    pub listname: String,
    #[serde(rename = "list_type")]
    pub listtype: ListType,
    pub likes: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ListOrderedByLikes {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub mediaimage: Option<String>,
    pub createdon: Option<NaiveDate>,
    pub listtype: Option<ListType>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct QueryList {
    pub limit_num: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateList {
    pub list_name: String,
    pub list_type: ListType,
    pub list_items: Vec<ListItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateList {
    pub list_name: Option<String>,
    pub list_type: Option<ListType>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ErrorList(pub String);

impl std::fmt::Display for ErrorList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(rename_all = "lowercase", type_name = "listtype")]
#[serde(rename_all = "lowercase")]
pub enum ListType {
    Anime,
    Movies,
    Songs,
    VideoGames,
}
