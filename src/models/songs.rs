use serde::{Deserialize, Serialize};
use time::Date;

use crate::utils::traits::{Error, IntoSerial};

pub struct Song {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub media_image: String,
    pub created_on: Date,
}

#[derive(Debug, Clone, Serialize)]
pub struct SongSerial {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub media_image: String,
    pub created_on: String,
}

impl IntoSerial for Song {
    type Serial = SongSerial;
    fn to_serial(&self) -> Self::Serial {
        Self::Serial {
            author: self.author.clone(),
            created_on: self.created_on.to_string(),
            id: self.id,
            media_image: self.media_image.clone(),
            title: self.title.clone(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct QuerySong {
    pub author: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateSong {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub media_image: String,
    pub created_on: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateSong {
    pub title: Option<String>,
    pub author: Option<String>,
    pub media_image: Option<String>,
    pub created_on: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ErrorSong(pub String);

impl Error for ErrorSong {
    fn new(err: String) -> Self {
        Self(err)
    }
}

impl std::fmt::Display for ErrorSong {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "failed to retrieve song due to the following error: {:#?}",
            self.0
        )
    }
}
