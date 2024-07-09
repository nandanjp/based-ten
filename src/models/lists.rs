use crate::utils::traits::Error;
use serde::{de::Visitor, Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize)]
pub struct List {
    pub email: String,
    pub list_name: String,
    pub list_type: ListType,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct QueryList {}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateList {
    pub email: String,
    pub list_name: String,
    pub list_type: ListType,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateList {
    pub list_name: Option<String>,
    pub list_type: Option<ListType>,
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, sqlx::Type)]
#[sqlx(rename_all = "lowercase", type_name = "listtype")]
pub enum ListType {
    Anime,
    Movies,
    Songs,
    VideoGames,
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum ListTypeError {
    #[error("invalid list type provided: {unknown}")]
    InvalidListType { unknown: String },
}

impl ListType {
    fn to_str(&self) -> &'static str {
        match self {
            Self::Anime => "anime",
            Self::Movies => "movies",
            Self::Songs => "songs",
            Self::VideoGames => "videogames",
        }
    }
    pub fn from_str(s: &str) -> Result<Self, ListTypeError> {
        match s.to_lowercase().as_str() {
            "anime" => Ok(ListType::Anime),
            "movies" => Ok(ListType::Movies),
            "songs" => Ok(ListType::Songs),
            "videogames" => Ok(ListType::VideoGames),
            _ => Err(ListTypeError::InvalidListType {
                unknown: String::from(s),
            }),
        }
    }
}

impl Serialize for ListType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self.to_str())
    }
}

struct ListTypeVisitor;
impl<'de> Deserialize<'de> for ListType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(ListTypeVisitor)
    }
}

impl<'de> Visitor<'de> for ListTypeVisitor {
    type Value = ListType;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "trying to convert the provided string into the 'ListType' type"
        )
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match ListType::from_str(v) {
            Ok(v) => Ok(v),
            Err(_) => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(v),
                &self,
            )),
        }
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match ListType::from_str(v) {
            Ok(v) => Ok(v),
            Err(_) => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(v),
                &self,
            )),
        }
    }
}
