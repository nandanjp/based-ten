use serde::{Deserialize, Serialize};
use time::Date;

use crate::utils::traits::{Error, IntoSerial};

pub struct Game {
    pub id: i32,
    pub title: String,
    pub media_image: String,
    pub console: String,
    pub created_on: Date,
}

#[derive(Clone, Debug, Serialize)]
pub struct GameSerial {
    pub id: i32,
    pub title: String,
    pub media_image: String,
    pub console: String,
    pub created_on: String,
}

impl IntoSerial for Game {
    type Serial = GameSerial;
    fn to_serial(&self) -> Self::Serial {
        Self::Serial {
            id: self.id,
            title: self.title.clone(),
            media_image: self.media_image.clone(),
            console: self.console.clone(),
            created_on: self.created_on.to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct QueryGame {
    pub console: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateGame {
    pub id: i32,
    pub title: String,
    pub media_image: String,
    pub console: String,
    pub created_on: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateGame {
    pub title: Option<String>,
    pub media_image: Option<String>,
    pub console: Option<String>,
    pub created_on: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ErrorGame(pub String);
impl Error for ErrorGame {
    fn new(err: String) -> Self {
        Self(err)
    }
}

impl std::fmt::Display for ErrorGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "failed to retrieve game due to the following error: {:#?}",
            self.0
        )
    }
}
