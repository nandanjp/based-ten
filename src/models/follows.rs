use crate::utils::traits::{Error, IntoSerial};
use serde::{Deserialize, Serialize};

pub struct Follow {
    pub follower_email: String,
    pub following_email: String,
}

impl IntoSerial for Follow {
    type Serial = FollowSerial;

    fn to_serial(&self) -> Self::Serial {
        Self::Serial {
            follower_email: self.follower_email.clone(),
            following_email: self.following_email.clone(),
        }
    }
}

// not necessary (?)
#[derive(Clone, Debug, Serialize)]
pub struct FollowSerial {
    pub follower_email: String,
    pub following_email: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct QueryFollow {
    pub following_email: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateFollow {
    pub follower_email: String,
    pub following_email: String,
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
