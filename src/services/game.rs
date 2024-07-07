use crate::{
    models::anime::{Anime, AnimeError, AnimeQuery, CreateAnime, UpdateAnime},
    utils::{functions::convert_date, traits::GeneralService},
};
use axum::async_trait;
use serde::Deserialize;
use time::{macros::format_description, Date};

pub struct GameService;
#[async_trait]
impl GeneralService for GameService {
    type Response = Anime;
    type ListResponse = Vec<Anime>;
    type QueryObject = AnimeQuery;
    type CreateObject = CreateAnime;
    type UpdateObject = UpdateAnime;
    type Error = AnimeError;

    async fn get_all(
        pool: &sqlx::PgPool,
        query_obj: Self::QueryObject,
    ) -> Result<Self::ListResponse, Self::Error> {
        match query_obj {
            AnimeQuery {
                num_episodes: Some(num_episodes),
            } => sqlx::query!(
                r#"SELECT * FROM Anime WHERE numepisodes = $1"#,
                num_episodes
            )
            .fetch_all(pool)
            .await
            .map(|a| {
                a.into_iter()
                    .map(|a| Self::Response {
                        id: a.id,
                        title: a.title,
                        media_image: a.mediaimage.unwrap(),
                        num_episodes,
                        created_on: a.createdon.unwrap(),
                    })
                    .collect::<Self::ListResponse>()
            })
            .map_err(|e| {
                AnimeError(format!(
                    "failed to retrieve all anime due to the following error: {e:#?}"
                ))
            }),
            _ => sqlx::query!(r#"SELECT * FROM Anime"#,)
                .fetch_all(pool)
                .await
                .map(|a| {
                    a.into_iter()
                        .map(|a| Self::Response {
                            id: a.id,
                            title: a.title,
                            media_image: a.mediaimage.unwrap(),
                            num_episodes: a.numepisodes.unwrap(),
                            created_on: a.createdon.unwrap(),
                        })
                        .collect::<Self::ListResponse>()
                })
                .map_err(|e| {
                    AnimeError(format!(
                        "failed to retrieve all anime due to the following error: {e:#?}"
                    ))
                }),
        }
    }

    async fn get_by_id(pool: &sqlx::PgPool, id: i32) -> Result<Self::Response, Self::Error> {
        sqlx::query!(r#"SELECT * FROM Anime WHERE id = $1"#, id)
            .fetch_one(pool)
            .await
            .map(|a| Self::Response {
                id: a.id,
                title: a.title,
                media_image: a.mediaimage.unwrap(),
                num_episodes: a.numepisodes.unwrap(),
                created_on: a.createdon.unwrap(),
            })
            .map_err(|e| {
                AnimeError(format!(
                    "failed to retrieve an anime with id = {id} due to the following error: {e:#?}"
                ))
            })
    }

    async fn create(
        pool: &sqlx::PgPool,
        create_obj: Self::CreateObject,
    ) -> Result<Self::Response, Self::Error> {
        let date = convert_date::<AnimeError>(
            update_obj
                .created_on
                .unwrap_or(anime.created_on.to_string()),
        )?;
        sqlx::query!(r#"INSERT INTO Anime(id, title, mediaimage, numepisodes, createdon) VALUES($1, $2, $3, $4, $5) RETURNING id, title, mediaimage, numepisodes, createdon"#, create_obj.id, create_obj.title, create_obj.media_image, create_obj.num_episodes, date).fetch_one(pool).await.map(|a| Self::Response {
            id: a.id,
            num_episodes: a.numepisodes.unwrap(),
            title: a.title,
            media_image: a.mediaimage.unwrap(),
            created_on: a.createdon.unwrap()
        }).map_err(|e| AnimeError(format!("failed to create anime with the provided details due to the following error: {e:#?}")))
    }

    async fn update(
        pool: &sqlx::PgPool,
        update_obj: Self::UpdateObject,
        id: i32,
    ) -> Result<Self::Response, Self::Error> {
        let anime = sqlx::query!(r#"SELECT * FROM Anime WHERE id = $1"#, id)
            .fetch_one(pool)
            .await
            .map(|a| Self::Response {
                id: a.id,
                title: a.title,
                num_episodes: a.numepisodes.unwrap(),
                media_image: a.mediaimage.unwrap(),
                created_on: a.createdon.unwrap(),
            })
            .map_err(|e| {
                AnimeError(format!(
                    "failed to find an anime with the id = {id} due to the following error: {e:#?}"
                ))
            })?;

        let title = update_obj.title.unwrap_or(anime.title);
        let num_episodes = update_obj.num_episodes.unwrap_or(anime.num_episodes);
        let media_image = update_obj.media_image.unwrap_or(anime.media_image);
        let date = convert_date::<AnimeError>(
            update_obj
                .created_on
                .unwrap_or(anime.created_on.to_string()),
        )?;

        sqlx::query!(r#"UPDATE Anime SET title = $1, numepisodes = $2, mediaimage = $3, createdon = $4 WHERE id = $5 RETURNING id, title, numepisodes, mediaimage, createdon"#, title, num_episodes, media_image, date, id).fetch_one(pool).await.map(|a| Self::Response {
            id: a.id,
            title: a.title,
            media_image: a.mediaimage.unwrap(),
            num_episodes: a.numepisodes.unwrap(),
            created_on: a.createdon.unwrap()
        }).map_err(|e| AnimeError(format!("failed to update anime with the provided details due to the following error: {e:#?}")))
    }

    async fn delete(pool: &sqlx::PgPool, id: i32) -> Result<Self::Response, Self::Error> {
        sqlx::query!(r#"DELETE FROM Anime WHERE id = $1 RETURNING id, title, numepisodes, mediaimage, createdon"#, id).fetch_one(pool).await.map(|a| Self::Response {
            id: a.id,
            title: a.title,
            media_image: a.mediaimage.unwrap(),
            num_episodes: a.numepisodes.unwrap(),
            created_on: a.createdon.unwrap()
        }).map_err(|e| AnimeError(format!("failed to delete anime with the given id = {id} due to the following error: {e:#?}")))
    }
}
