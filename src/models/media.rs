use serde::{Deserialize, Serialize};
use sqlx::types::time::Date;
use crate::utils::traits::{Error, IntoSerial};

pub struct Media {
    pub id: i32,
    pub title: String,
    pub media_image: String,
    pub created_on: Date,
    pub media_type: String
}

impl IntoSerial for Media {
    type Serial = MediaSerial;

    fn to_serial(&self) -> Self::Serial {
        Self::Serial {
            id: self.id,
            title: self.title.clone(),
            media_image: self.media_image.clone(),
            created_on: self.created_on.to_string(),
            media_type: self.media_type.clone(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct MediaSerial {
    pub id: i32,
    pub title: String,
    pub media_image: String,
    pub created_on: String,
    #[serde(rename = "type")]
    pub media_type: String
}

#[derive(Debug, Clone)]
pub struct MediaError(pub String);
impl Error for MediaError {
    fn new(err: String) -> Self {
        Self(err)
    }
}
impl std::fmt::Display for MediaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "failed to retrieve media due to the following error: {:#?}",
            self.0
        )
    }
}
