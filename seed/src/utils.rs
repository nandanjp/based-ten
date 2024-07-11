use std::fs::File;

use csv::ReaderBuilder;
use serde::Deserialize;
use time::{macros::format_description, Date};

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum InsertType {
    Anime,
    Games,
    Movies,
    Songs,
    Users,
    Groups,
    GroupMembers,
    Likes,
    Lists,
    ListItems,
    Follows,
}

#[derive(Clone, Copy, Debug)]
pub enum DataType {
    Dev,
    Prod,
}

impl InsertType {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "anime" => Ok(InsertType::Anime),
            "video-games" => Ok(InsertType::Games),
            "movies" => Ok(InsertType::Movies),
            "songs" => Ok(InsertType::Songs),
            "users" => Ok(InsertType::Users),
            "groups" => Ok(InsertType::Groups),
            "group-members" => Ok(InsertType::GroupMembers),
            "likes" => Ok(InsertType::Likes),
            "lists" => Ok(InsertType::Lists),
            "list-items" => Ok(InsertType::ListItems),
            "follows" => Ok(InsertType::Follows),
            _ => Err(format!("unkown table name provided: table_name = {s}")),
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            InsertType::Anime => "anime",
            InsertType::Follows => "follows",
            InsertType::Games => "video-games",
            InsertType::GroupMembers => "group-members",
            InsertType::Groups => "groups",
            InsertType::Likes => "likes",
            InsertType::ListItems => "list-items",
            InsertType::Lists => "lists",
            InsertType::Movies => "movies",
            InsertType::Songs => "songs",
            InsertType::Users => "users",
        }
    }
}

pub fn convert_date(created_on: String) -> Result<Date, String> {
    let format = format_description!("[year]-[month]-[day]");
    Date::parse(created_on.as_str(), &format).map_err(|e| {
        format!("failed to create anime due to an invalid date string provided: {e:#?}")
    })
}

pub fn read_csv<T>(file_path: &str) -> Result<Vec<T>, Box<dyn std::error::Error>>
where
    T: for<'de> Deserialize<'de>,
{
    let file = File::open(file_path)?;
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);

    let mut values = Vec::new();
    for result in reader.deserialize() {
        let a: T = result?;
        values.push(a);
    }
    Ok(values)
}
