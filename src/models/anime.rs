use serde::{Deserialize, Serialize};
use sqlx::types::time::Date;

use crate::utils::traits::{Error, IntoSerial};

pub struct Anime {
    pub id: i32,
    pub title: String,
    pub media_image: String,
    pub num_episodes: i32,
    pub created_on: Date,
}

impl IntoSerial for Anime {
    type Serial = AnimeSerial;

    fn to_serial(&self) -> Self::Serial {
        Self::Serial {
            id: self.id,
            title: self.title.clone(),
            media_image: self.media_image.clone(),
            num_episodes: self.num_episodes,
            created_on: self.created_on.to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct AnimeSerial {
    pub id: i32,
    pub title: String,
    pub media_image: String,
    pub num_episodes: i32,
    pub created_on: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct AnimeQuery {
    pub num_episodes: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateAnime {
    pub id: i32,
    pub title: String,
    pub media_image: String,
    pub num_episodes: i32,
    pub created_on: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateAnime {
    pub title: Option<String>,
    pub media_image: Option<String>,
    pub num_episodes: Option<i32>,
    pub created_on: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AnimeError(pub String);
impl Error for AnimeError {
    fn new(err: String) -> Self {
        Self(err)
    }
}
impl std::fmt::Display for AnimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "failed to retrieve anime due to the following error: {:#?}",
            self.0
        )
    }
}
