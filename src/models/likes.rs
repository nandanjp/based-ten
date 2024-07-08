use crate::utils::traits::Error;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize)]
pub struct Like {
    pub liker_email: String,
    pub liking_email: String,
    pub list_name: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct QueryLike {
    pub liking_email: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateLike {
    pub liker_email: String,
    pub liking_email: String,
    pub list_name: String,
}

#[derive(Debug, Clone)]
pub struct LikeError(pub String);
impl Error for LikeError {
    fn new(err: String) -> Self {
        Self(err)
    }
}
impl std::fmt::Display for LikeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "failed to retrieve like due to the following error: {:#?}",
            self.0
        )
    }
}
