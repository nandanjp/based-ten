use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct ListItem {
    pub username: String,
    pub listname: String,
    pub rankinginlist: i32,
    pub itemid: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateListItem {
    pub user_name: String,
    pub list_name: String,
    pub ranking_in_list: i32,
    pub item_id: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateListItem {
    pub user_name: Option<String>,
    pub list_name: Option<String>,
    pub ranking_in_list: Option<i32>,
    pub item_id: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct ErrorListItem(pub String);

impl std::fmt::Display for ErrorListItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
