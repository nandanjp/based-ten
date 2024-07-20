use serde::{de::Visitor, Deserialize, Serialize};
use sqlx::postgres::{PgHasArrayType, PgTypeInfo};
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, sqlx::Type)]
#[sqlx(rename_all = "lowercase", type_name = "listtype")]
pub enum ListType {
    Anime,
    Movies,
    Songs,
    VideoGames,
}

impl PgHasArrayType for ListType {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        PgTypeInfo::with_name("_listtype")
    }
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
    fn from_str(s: &str) -> Result<Self, ListTypeError> {
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
