use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub email: String,
    pub username: String,
    pub userpassword: String,
    pub createdat: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct QueryUser {}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateUser {
    pub email: String,
    #[serde(rename = "username")]
    pub user_name: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateUser {
    pub email: Option<String>,
    pub user_name: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserSchema {
    #[serde(rename = "username")]
    pub user_name: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct UserError(pub String);

impl std::fmt::Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
