use serde::{Deserialize, Serialize};
use sqlx::types::time::Date;

pub struct Anime {
    pub id: i32,
    pub title: String,
    pub media_image: String,
    pub num_episodes: i32,
    pub created_on: Date,
}

impl Anime {
    pub fn to_serial(&self) -> AnimeSerial {
        AnimeSerial {
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

#[derive(Clone, Debug, Deserialize)]
pub struct CreateAnime {
    pub title: String,
    pub media_image: String,
    pub num_episodes: i32,
}
