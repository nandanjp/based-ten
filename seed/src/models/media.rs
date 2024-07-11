use serde::Deserialize;
use time::Date;

use crate::utils::convert_date;

use super::traits::Commit;

#[derive(Debug, Deserialize)]
pub struct Anime {
    pub anime_id: i32,
    pub title: String,
    pub image: String,
    pub released_on: String,
    pub episodes: i32,
}

impl Commit for Anime {
    type Value = Self;
    async fn commit(
        pool: &sqlx::PgPool,
        values: Vec<Self::Value>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let anime_ids = values
            .iter()
            .map(|a| a.anime_id as i64)
            .collect::<Vec<i64>>();
        let titles = values
            .iter()
            .map(|a| a.title.clone())
            .collect::<Vec<String>>();
        let images = values
            .iter()
            .map(|a| a.image.clone())
            .collect::<Vec<String>>();
        let episodes = values
            .iter()
            .map(|a| a.episodes as i64)
            .collect::<Vec<i64>>();
        let dates = values
            .iter()
            .map(|a| match convert_date(a.released_on.clone()) {
                Ok(date) => date,
                Err(e) => {
                    panic!(
                        "failed to parse the date due to the following error: {e:#?}, anime = {}",
                        a.anime_id
                    )
                }
            })
            .collect::<Vec<Date>>();
        sqlx::query!(
            r#"
                INSERT INTO Anime(id, title, mediaimage, numepisodes, createdon)
                SELECT * FROM UNNEST($1::int8[], $2::text[], $3::text[], $4::int8[], $5::Date[])
            "#,
            &anime_ids[..],
            &titles[..],
            &images[..],
            &episodes[..],
            &dates[..] as &[Date]
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct Movie {
    pub movie_id: i32,
    pub title: String,
    pub image: String,
    pub released_on: String,
}

impl Commit for Movie {
    type Value = Self;
    async fn commit(
        pool: &sqlx::PgPool,
        values: Vec<Self::Value>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let movie_ids = values
            .iter()
            .map(|m| m.movie_id as i64)
            .collect::<Vec<i64>>();
        let titles = values
            .iter()
            .map(|m| m.title.clone())
            .collect::<Vec<String>>();
        let images = values
            .iter()
            .map(|m| m.image.clone())
            .collect::<Vec<String>>();
        let dates = values
            .iter()
            .map(|m| {
                convert_date(m.released_on.clone()).expect("failed to convert the string to a date")
            })
            .collect::<Vec<Date>>();
        let _ = sqlx::query!(
            r#"
            INSERT INTO Movies(id, title, mediaimage, createdon)
            SELECT * FROM UNNEST($1::int8[], $2::text[], $3::text[], $4::Date[])
        "#,
            &movie_ids[..],
            &titles[..],
            &images[..],
            &dates[..] as &[Date]
        )
        .execute(pool)
        .await?;
        Ok(())
    }
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

impl Commit for Song {
    type Value = Self;

    async fn commit(
        pool: &sqlx::PgPool,
        values: Vec<Self::Value>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let song_ids = values
            .iter()
            .map(|s| s.song_id as i64)
            .collect::<Vec<i64>>();
        let titles = values
            .iter()
            .map(|s| s.title.clone())
            .collect::<Vec<String>>();
        let writers = values
            .iter()
            .map(|s| s.writer.clone())
            .collect::<Vec<String>>();
        let albums = values
            .iter()
            .map(|s| s.album.clone())
            .collect::<Vec<String>>();
        let images = values
            .iter()
            .map(|s| s.image.clone())
            .collect::<Vec<String>>();
        let dates = values
            .iter()
            .map(|s| convert_date(s.released_in_year.clone()).expect("failed to parse date"))
            .collect::<Vec<Date>>();
        let _ = sqlx::query!(r#"
                INSERT INTO Songs(id, title, author, album, mediaimage, createdon)
                SELECT * FROM UNNEST($1::int8[], $2::text[], $3::text[], $4::text[], $5::text[], $6::Date[])
            "#, &song_ids[..], &titles[..], &writers[..], &albums[..], &images[..], &dates[..] as &[Date]).execute(pool).await?;
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct Games {
    pub game_id: i32,
    pub title: String,
    pub image: String,
    pub released_on: String,
    pub platform: String,
}

impl Commit for Games {
    type Value = Self;

    async fn commit(
        pool: &sqlx::PgPool,
        values: Vec<Self::Value>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let game_ids = values
            .iter()
            .map(|g| g.game_id as i64)
            .collect::<Vec<i64>>();
        let titles = values
            .iter()
            .map(|g| g.title.clone())
            .collect::<Vec<String>>();
        let images = values
            .iter()
            .map(|g| g.image.clone())
            .collect::<Vec<String>>();
        let platforms = values
            .iter()
            .map(|g| g.platform.clone())
            .collect::<Vec<String>>();
        let dates = values
            .iter()
            .map(|g| convert_date(g.released_on.clone()).expect("failed to parse date"))
            .collect::<Vec<Date>>();

        let _ = sqlx::query!(
            r#"
                INSERT INTO VideoGames(id, title, mediaimage, console, createdon)
                SELECT * FROM UNNEST($1::int8[], $2::text[], $3::text[], $4::text[], $5::Date[])
            "#,
            &game_ids[..],
            &titles[..],
            &images[..],
            &platforms[..],
            &dates[..] as &[Date]
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}
