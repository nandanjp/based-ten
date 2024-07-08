use crate::utils::traits::{Error, IntoSerial};
use serde::{Deserialize, Serialize};

pub struct Like {
    pub liker_email: String,
    pub liking_email: String,
    pub list_name: String,
}

impl IntoSerial for Like {
    type Serial = LikeSerial;

    fn to_serial(&self) -> Self::Serial {
        Self::Serial {
            liker_email: self.liker_email.clone(),
            liking_email: self.liking_email.clone(),
            list_name: self.list_name.clone(),
        }
    }
}

// not necessary (?)
#[derive(Clone, Debug, Serialize)]
pub struct LikeSerial {
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
