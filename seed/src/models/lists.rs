use serde::Deserialize;

use super::list_type::ListType;

#[derive(Debug, Deserialize)]
pub struct List {
    pub user_email: String,
    pub list_name: String,
    pub list_type: ListType,
}

#[derive(Debug, Deserialize)]
pub struct ListItem {
    pub user_email: String,
    pub list_name: String,
    pub rank_in_list: i32,
    pub item_id: i32,
}
