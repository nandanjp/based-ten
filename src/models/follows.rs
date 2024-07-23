use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct Follow {
    pub follower: String,
    pub following: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct QueryFollow {
    pub following: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateFollow {
    pub follower: String,
    pub following: String,
}

#[derive(Debug, Clone)]
pub struct FollowError(pub String);

impl std::fmt::Display for FollowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
