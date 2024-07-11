use serde::{Deserialize, Serialize};

use crate::utils::traits::{Error, IntoSerial};

#[derive(Debug, Clone, Serialize)]
pub struct ListItem {
    pub user_name: String,
    pub list_name: String,
    pub ranking_in_list: i32,
    pub item_id: i32,
}

impl IntoSerial for ListItem {
    type Serial = Self;
    fn to_serial(&self) -> Self::Serial {
        Self {
            user_name: self.user_name.clone(),
            list_name: self.list_name.clone(),
            ranking_in_list: self.ranking_in_list,
            item_id: self.item_id,
        }
    }
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

impl Error for ErrorListItem {
    fn new(err: String) -> Self {
        Self(err)
    }
}

impl std::fmt::Display for ErrorListItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "failed to retrieve list item due to the following error: {:#?}",
            self.0
        )
    }
}
