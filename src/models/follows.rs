use crate::utils::traits::Error;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize)]
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
impl Error for FollowError {
    fn new(err: String) -> Self {
        Self(err)
    }
}
impl std::fmt::Display for FollowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "failed to retrieve follow due to the following error: {:#?}",
            self.0
        )
    }
}
