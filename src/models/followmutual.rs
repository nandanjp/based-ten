use crate::utils::traits::Error;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize)]
pub struct FollowMutual {
    pub follower_email: String,
    pub follows_back: bool,
}

#[derive(Debug, Clone)]
pub struct FollowMutualError(pub String);
impl Error for FollowMutualError {
    fn new(err: String) -> Self {
        Self(err)
    }
}
impl std::fmt::Display for FollowMutualError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "failed to retrieve mutual follows due to the following error: {:#?}",
            self.0
        )
    }
}
