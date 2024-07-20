use serde::{Deserialize, Serialize};
use time::Date;

use crate::utils::traits::{Error, IntoSerial};

pub struct Movie {
    pub id: i32,
    pub title: String,
    pub media_image: String,
    pub created_on: Date,
}

#[derive(Debug, Clone, Serialize)]
pub struct MovieSerial {
    pub id: i32,
    pub title: String,
    pub media_image: String,
    pub created_on: String,
}

impl IntoSerial for Movie {
    type Serial = MovieSerial;
    fn to_serial(&self) -> Self::Serial {
        Self::Serial {
            id: self.id,
            title: self.title.clone(),
            media_image: self.media_image.clone(),
            created_on: self.created_on.to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct QueryMovie {}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateMovie {
    pub id: i32,
    pub title: String,
    pub media_image: String,
    pub created_on: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateMovie {
    pub title: Option<String>,
    pub media_image: Option<String>,
    pub created_on: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ErrorMovie(pub String);

impl Error for ErrorMovie {
    fn new(err: String) -> Self {
        Self(err)
    }
}
impl std::fmt::Display for ErrorMovie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "failed to retrieve movie due to the following error: {:#?}",
            self.0
        )
    }
}
