use serde::Deserialize;

// anime_id,Name,Score,Genres,English name,Japanese name,sypnopsis,Type,Episodes,Aired,Premiered,Producers,Licensors,Studios,Source,Duration,Rating,Ranked,Popularity,Members,Favorites,Watching,Completed,On-Hold,Dropped

// id,slug,name,metacritic,released,tba,updated,website,rating,rating_top,playtime,achievements_count,ratings_count,suggestions_count,game_series_count,reviews_count,platforms,developers,genres,publishers,esrb_rating,added_status_yet,added_status_owned,added_status_beaten,added_status_toplay,added_status_dropped,added_status_playing

// id,name,album,album_id,artists,artist_ids,track_number,disc_number,explicit,danceability,energy,key,loudness,mode,speechiness,acousticness,instrumentalness,liveness,valence,tempo,duration_ms,time_signature,year,release_date

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
