use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct Like {
    pub likername: String,
    pub likingname: String,
    pub listname: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct QueryLike {
    pub liking_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateLike {
    pub liker_name: String,
    pub liking_name: String,
    pub list_name: String,
}

#[derive(Debug, Clone)]
pub struct LikeError(pub String);

impl std::fmt::Display for LikeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
