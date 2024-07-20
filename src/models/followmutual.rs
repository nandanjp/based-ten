use crate::utils::traits::{Error, IntoSerial};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct FollowMutual {
    pub follower: String,
    pub follows_back: bool,
}

impl IntoSerial for FollowMutual {
    type Serial = Self;
    fn to_serial(&self) -> Self::Serial {
        Self {
            follower: self.follower.clone(),
            follows_back: self.follows_back.clone(),
        }
    }
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
