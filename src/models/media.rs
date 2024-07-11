use crate::utils::traits::{Error, IntoSerial};
use serde::{Deserialize, Serialize};
use sqlx::types::time::Date;

use super::lists::ListType;

pub struct Media {
    pub id: i32,
    pub title: String,
    pub media_image: String,
    pub created_on: Date,
    pub media_type: ListType,
}

pub struct MediaWithLikes {
    pub id: i32,
    pub title: String,
    pub media_image: String,
    pub created_on: Date,
    pub media_type: ListType,
    pub likes: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct QueryMedia {
    pub media_type: Option<ListType>,
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

impl IntoSerial for MediaWithLikes {
    type Serial = MediaWithLikesSerial;

    fn to_serial(&self) -> Self::Serial {
        Self::Serial {
            id: self.id,
            title: self.title.clone(),
            media_image: self.media_image.clone(),
            created_on: self.created_on.to_string(),
            media_type: self.media_type.clone(),
            likes: self.likes,
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
    pub media_type: ListType,
}

#[derive(Clone, Debug, Serialize)]
pub struct MediaWithLikesSerial {
    pub id: i32,
    pub title: String,
    pub media_image: String,
    pub created_on: String,
    #[serde(rename = "type")]
    pub media_type: ListType,
    pub likes: i32,
}

impl IntoSerial for MediaWithLikesSerial {
    type Serial = Self;
    fn to_serial(&self) -> Self::Serial {
        Self {
            id: self.id,
            title: self.title.clone(),
            media_image: self.media_image.clone(),
            created_on: self.created_on.clone(),
            media_type: self.media_type,
            likes: self.likes,
        }
    }
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
