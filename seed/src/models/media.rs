use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Anime {
    pub anime_id: i32,
    pub title: String,
    pub image: String,
    pub released_on: String,
    pub episodes: i32,
}

#[derive(Debug, Deserialize)]
pub struct Movie {
    pub movie_id: i32,
    pub title: String,
    pub image: String,
    pub released_on: String,
}

#[derive(Debug, Deserialize)]
pub struct Song {
    pub song_id: i32,
    pub title: String,
    pub writer: String,
    pub album: String,
    pub image: String,
    pub released_in_year: String,
}

#[derive(Debug, Deserialize)]
pub struct Games {
    pub game_id: i32,
    pub title: String,
    pub image: String,
    pub released_on: String,
    pub platform: String,
}
