use crate::utils::traits::{Error, IntoSerial};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize)]
pub struct Like {
    pub liker_name: String,
    pub liking_name: String,
    pub list_name: String,
}

impl IntoSerial for Like {
    type Serial = Self;
    fn to_serial(&self) -> Self::Serial {
        Self {
            liker_name: self.liker_name.clone(),
            liking_name: self.liking_name.clone(),
            list_name: self.list_name.clone(),
        }
    }
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
