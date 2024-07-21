use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct Group {
    pub gid: i32,
    pub groupname: String,
    pub ownedby: String,
}

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct GroupRecursive {
    pub gid: Option<i32>,
    pub groupname: Option<String>,
    pub ownedby: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct QueryGroups {
    pub order_by_author: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GroupMember {
    pub username: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateGroups {
    pub gid: i32,
    pub group_name: String,
}

#[derive(Debug, Clone)]
pub struct GroupsError(pub String);

impl std::fmt::Display for GroupsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
