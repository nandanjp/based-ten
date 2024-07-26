use crate::{
    models::anime::{Anime, AnimeError, AnimeQuery, AnimeSortKey, CreateAnime, UpdateAnime},
    utils::{functions::page_limit, traits::GeneralService},
};
use axum::async_trait;

const NUM_ROWS: i64 = 1860;

pub struct AnimeService;
#[async_trait]
impl GeneralService for AnimeService {
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
        let (limit, page) = page_limit(NUM_ROWS, query_obj.limit, query_obj.page);

        match query_obj {
            AnimeQuery {
                num_episodes: Some(num_episodes), sort_key: Some(key), ..
            } => match key {
                AnimeSortKey::Title => sqlx::query_as!(
                    Self::Response,
                    r#"SELECT * FROM Anime WHERE numepisodes = $1 ORDER BY title OFFSET $2 LIMIT $3"#,
                    num_episodes,
                    page * limit,
                    limit
                )
                .fetch_all(pool)
                .await
                .map_err(|e| AnimeError(format!("failed to retrieve all anime: {e:#?}"))),
                AnimeSortKey::NumEpisodes => sqlx::query_as!(
                    Self::Response,
                    r#"SELECT * FROM Anime WHERE numepisodes = $1 ORDER BY numepisodes OFFSET $2 LIMIT $3"#,
                    num_episodes,
                    page * limit,
                    limit
                )
                .fetch_all(pool)
                .await
                .map_err(|e| AnimeError(format!("failed to retrieve all anime: {e:#?}")))
            },
            AnimeQuery {
                title: Some(title), ..
            } => sqlx::query_as!(
                Self::Response,
                r#"SELECT * FROM Anime WHERE similarity(title, $1) > 0.15 ORDER BY similarity(title, $1) DESC OFFSET $2 LIMIT $3"#,
                title,
                page * limit,
                limit
            )
            .fetch_all(pool)
            .await
            .map_err(|e| AnimeError(format!("failed to retrieve all anime: {e:#?}"))),
            AnimeQuery {
                sort_key: Some(key), ..
            } => match key {
                AnimeSortKey::Title => sqlx::query_as!(
                    Self::Response,
                    r#"SELECT * FROM Anime ORDER BY title OFFSET $1 LIMIT $2"#,
                    page * limit,
                    limit
                )
                .fetch_all(pool)
                .await
                .map_err(|e| AnimeError(format!("failed to retrieve all anime: {e:#?}"))),
                AnimeSortKey::NumEpisodes => sqlx::query_as!(
                    Self::Response,
                    r#"SELECT * FROM Anime ORDER BY numepisodes OFFSET $1 LIMIT $2"#,
                    page * limit,
                    limit
                )
                .fetch_all(pool)
                .await
                .map_err(|e| AnimeError(format!("failed to retrieve all anime: {e:#?}")))
            },
            _ => sqlx::query_as!(Self::Response, r#"SELECT * FROM Anime OFFSET $1 LIMIT $2"#, page * limit, limit)
                .fetch_all(pool)
                .await
                .map_err(|e| AnimeError(format!("failed to retrieve all anime: {e:#?}"))),
        }
    }

    async fn get_by_id(pool: &sqlx::PgPool, id: i32) -> Result<Self::Response, Self::Error> {
        sqlx::query_as!(Self::Response, r#"SELECT * FROM Anime WHERE id = $1"#, id)
            .fetch_one(pool)
            .await
            .map_err(|e| AnimeError(format!("failed to retrieve anime with id = {id}: {e:#?}")))
    }

    async fn create(
        pool: &sqlx::PgPool,
        create_obj: Self::CreateObject,
    ) -> Result<Self::Response, Self::Error> {
        sqlx::query_as!(Self::Response, r#"INSERT INTO Anime(id, title, mediaimage, numepisodes, createdon) VALUES($1, $2, $3, $4, $5) RETURNING *"#, 
                create_obj.id, create_obj.title, create_obj.media_image, create_obj.num_episodes, create_obj.created_on
            )
            .fetch_one(pool)
            .await
            .map_err(|e| AnimeError(format!("failed to create anime: {e:#?}")))
    }

    async fn update(
        pool: &sqlx::PgPool,
        update_obj: Self::UpdateObject,
        id: i32,
    ) -> Result<Self::Response, Self::Error> {
        let anime = Self::get_by_id(&pool, id).await?;
        let title = update_obj.title.unwrap_or(anime.title);
        let num_episodes = update_obj.num_episodes.unwrap_or(anime.numepisodes);
        let media_image = update_obj.media_image.unwrap_or(anime.mediaimage);
        let date = update_obj.created_on.unwrap_or(anime.createdon.unwrap());

        sqlx::query_as!(Self::Response, r#"UPDATE Anime SET title = $1, numepisodes = $2, mediaimage = $3, createdon = $4 WHERE id = $5 RETURNING *"#, title, num_episodes, media_image, date, id)
        .fetch_one(pool)
        .await
        .map_err(|e| AnimeError(format!("failed to update anime: {e:#?}")))
    }

    async fn delete(pool: &sqlx::PgPool, id: i32) -> Result<Self::Response, Self::Error> {
        sqlx::query_as!(
            Self::Response,
            r#"DELETE FROM Anime WHERE id = $1 RETURNING *"#,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| AnimeError(format!("failed to delete anime: {e:#?}")))
    }
}
