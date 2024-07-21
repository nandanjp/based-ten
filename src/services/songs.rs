use crate::{
    models::songs::{CreateSong, ErrorSong, QuerySong, Song, SongSortKey, UpdateSong},
    utils::{functions::page_limit, traits::GeneralService},
};
use axum::async_trait;

const NUM_ROWS: i64 = 4983;

pub struct SongService;
#[async_trait]
impl GeneralService for SongService {
    type Response = Song;
    type ListResponse = Vec<Song>;
    type CreateObject = CreateSong;
    type UpdateObject = UpdateSong;
    type QueryObject = QuerySong;
    type Error = ErrorSong;

    async fn get_all(
        pool: &sqlx::PgPool,
        query_obj: Self::QueryObject,
    ) -> Result<Self::ListResponse, Self::Error> {
        let (limit, page) = page_limit(NUM_ROWS, query_obj.limit, query_obj.page);

        match query_obj {
            QuerySong {
                author: Some(author),
                sort_key: Some(key),
                ..
            } => match key {
                SongSortKey::Author => sqlx::query_as!(
                    Self::Response,
                    "SELECT * FROM Songs WHERE author = $1 ORDER BY author OFFSET $2 LIMIT $3",
                    author,
                    page * limit,
                    limit,
                )
                .fetch_all(pool)
                .await
                .map_err(|e| ErrorSong(format!("failed to retrieve all songs: {e:#?}"))),
                SongSortKey::Title => sqlx::query_as!(
                    Self::Response,
                    "SELECT * FROM Songs WHERE author = $1 ORDER BY title OFFSET $2 LIMIT $3",
                    author,
                    page * limit,
                    limit
                )
                .fetch_all(pool)
                .await
                .map_err(|e| ErrorSong(format!("failed to retrieve all songs: {e:#?}"))),
            },
            QuerySong {
                sort_key: Some(key),
                ..
            } => match key {
                SongSortKey::Author => sqlx::query_as!(
                    Self::Response,
                    "SELECT * FROM Songs ORDER BY author OFFSET $1 LIMIT $2",
                    page * limit,
                    limit,
                )
                .fetch_all(pool)
                .await
                .map_err(|e| ErrorSong(format!("failed to retrieve all songs: {e:#?}"))),
                SongSortKey::Title => sqlx::query_as!(
                    Self::Response,
                    "SELECT * FROM Songs ORDER BY title OFFSET $1 LIMIT $2",
                    page * limit,
                    limit
                )
                .fetch_all(pool)
                .await
                .map_err(|e| ErrorSong(format!("failed to retrieve all songs: {e:#?}"))),
            },
            _ => sqlx::query_as!(
                Self::Response,
                "SELECT * FROM Songs OFFSET $1 LIMIT $2",
                page * limit,
                limit
            )
            .fetch_all(pool)
            .await
            .map_err(|e| ErrorSong(format!("failed to retrieve all songs: {e:#?}"))),
        }
    }

    async fn get_by_id(pool: &sqlx::PgPool, id: i32) -> Result<Self::Response, Self::Error> {
        sqlx::query_as!(Self::Response, "SELECT * FROM Songs WHERE id = $1", id)
            .fetch_one(pool)
            .await
            .map_err(|e| ErrorSong(format!("failed to retrieve song = {id}: {e:#?}")))
    }

    async fn create(
        pool: &sqlx::PgPool,
        create_obj: Self::CreateObject,
    ) -> Result<Self::Response, Self::Error> {
        sqlx::query_as!(Self::Response, "INSERT INTO Songs(id, title, author, album, mediaimage, createdon) VALUES($1, $2, $3, $4, $5, $6) RETURNING *", create_obj.id, create_obj.title, create_obj.author, create_obj.album, create_obj.media_image, create_obj.created_on).fetch_one(pool).await.map_err(|e| ErrorSong(format!("failed to create a song: {e:#?}")))
    }

    async fn update(
        pool: &sqlx::PgPool,
        update_obj: Self::UpdateObject,
        id: i32,
    ) -> Result<Self::Response, Self::Error> {
        let song = Self::get_by_id(&pool, id).await?;
        let title = update_obj.title.unwrap_or(song.title);
        let author = update_obj.author.unwrap_or(song.author);
        let media_image = update_obj.media_image.unwrap_or(song.mediaimage);
        let created_on = update_obj.created_on.unwrap_or(song.createdon.unwrap());
        sqlx::query_as!(Self::Response, "UPDATE Songs SET title = $1, author = $2, mediaimage = $3, createdon = $4 WHERE id = $5 RETURNING *", title, author, media_image, created_on, id).fetch_one(pool).await.map_err(|e| ErrorSong(format!("failed to updated song = {id}: {e:#?}")))
    }

    async fn delete(pool: &sqlx::PgPool, id: i32) -> Result<Self::Response, Self::Error> {
        sqlx::query_as!(
            Self::Response,
            "DELETE FROM Songs WHERE id = $1 RETURNING *",
            id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| ErrorSong(format!("failed to delete song = {id}: {e:#?}")))
    }
}
