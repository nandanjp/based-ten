use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct FollowMutual {
    pub follower: String,
    pub followsback: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct FollowMutualError(pub String);

impl std::fmt::Display for FollowMutualError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
