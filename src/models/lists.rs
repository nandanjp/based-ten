use serde::{Deserialize, Serialize};
use std::str::FromStr;
use crate::utils::traits::{Error, IntoSerial};

#[derive(Debug, Clone, Serialize)]
pub struct List {
    pub email: String,
    pub list_name: String,
    pub list_type: ListType
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct QueryList {}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateList {
    pub email: String,
    pub list_name: String,
    pub list_type: ListType
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateList {
    pub email: Option<String>,
    pub list_name: Option<String>,
    pub list_type: Option<ListType>
}

#[derive(Debug, Clone, Serialize)]
pub struct ErrorList(pub String);

impl Error for ErrorList {
    fn new(err: String) -> Self {
        Self(err)
    }
}
impl std::fmt::Display for ErrorList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "failed to retrieve list due to the following error: {:#?}",
            self.0
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, sqlx::Type, Serialize, Deserialize)]
#[sqlx(rename_all = "lowercase", type_name = "listtype")]
pub enum ListType {
    Anime,
    Movies,
    Songs,
    Videogames
}

impl FromStr for ListType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Anime" => Ok(Self::Anime),
            "Movies" => Ok(Self::Movies),
            "Songs" => Ok(Self::Songs),
            "Videogames" => Ok(Self::Videogames),
            _ => Err(format!("failed to parse the list type: {s}"))
        }
    }
}
