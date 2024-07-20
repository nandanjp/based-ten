use crate::{
    models::songs::{CreateSong, ErrorSong, QuerySong, Song, UpdateSong},
    utils::{functions::convert_date, traits::GeneralService},
};
use axum::async_trait;

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
        match query_obj {
            QuerySong {
                author: Some(author),
            } => sqlx::query!("SELECT * FROM Songs WHERE author = $1", author)
                .fetch_all(pool)
                .await
                .map(|a| {
                    a.into_iter()
                        .map(|a| Self::Response {
                            id: a.id,
                            title: a.title,
                            media_image: a.mediaimage.unwrap(),
                            created_on: a.createdon.unwrap(),
                            author: a.author.unwrap(),
                        })
                        .collect::<Vec<Song>>()
                })
                .map_err(|e| {
                    ErrorSong(format!(
                        "failed to retrieve all songs due to the following error: {e:#?}"
                    ))
                }),
            _ => sqlx::query!("SELECT * FROM Songs")
                .fetch_all(pool)
                .await
                .map(|a| {
                    a.into_iter()
                        .map(|a| Self::Response {
                            id: a.id,
                            title: a.title,
                            media_image: a.mediaimage.unwrap(),
                            created_on: a.createdon.unwrap(),
                            author: a.author.unwrap(),
                        })
                        .collect::<Vec<Song>>()
                })
                .map_err(|e| {
                    ErrorSong(format!(
                        "failed to retrieve all songs due to the following error: {e:#?}"
                    ))
                }),
        }
    }

    async fn get_by_id(pool: &sqlx::PgPool, id: i32) -> Result<Self::Response, Self::Error> {
        sqlx::query!("SELECT * FROM Songs WHERE id = $1", id)
            .fetch_one(pool)
            .await
            .map(|a| Self::Response {
                id: a.id,
                title: a.title,
                media_image: a.mediaimage.unwrap(),
                author: a.author.unwrap(),
                created_on: a.createdon.unwrap(),
            })
            .map_err(|e| {
                ErrorSong(format!(
                    "failed to retrieve song with id = {id} due to the following error: {e:#?}"
                ))
            })
    }

    async fn create(
        pool: &sqlx::PgPool,
        create_obj: Self::CreateObject,
    ) -> Result<Self::Response, Self::Error> {
        let created_on = convert_date::<ErrorSong>(create_obj.created_on)?;
        sqlx::query!("INSERT INTO Songs(id, title, author, mediaimage, createdon) VALUES($1, $2, $3, $4, $5) RETURNING id, title, author, mediaimage, createdon", create_obj.id, create_obj.title, create_obj.author, create_obj.media_image, created_on).fetch_one(pool).await.map(|a| Self::Response {
            id: a.id,
            title: a.title,
            author: a.author.unwrap(),
            media_image: a.mediaimage.unwrap(),
            created_on: a.createdon.unwrap()
        }).map_err(|e| ErrorSong(format!("failed to create a song with the given values due to the following error: {e:#?}")))
    }

    async fn update(
        pool: &sqlx::PgPool,
        update_obj: Self::UpdateObject,
        id: i32,
    ) -> Result<Self::Response, Self::Error> {
        let song = Self::get_by_id(&pool, id).await?;
        let title = update_obj.title.unwrap_or(song.title);
        let author = update_obj.author.unwrap_or(song.author);
        let media_image = update_obj.media_image.unwrap_or(song.media_image);
        let created_on = convert_date::<ErrorSong>(
            update_obj.created_on.unwrap_or(song.created_on.to_string()),
        )?;

        sqlx::query!("UPDATE Songs SET title = $1, author = $2, mediaimage = $3, createdon = $4 WHERE id = $5 RETURNING id, title, author, mediaimage, createdon", title, author, media_image, created_on, id).fetch_one(pool).await.map(|a| Self::Response {
            id: a.id,
            title: a.title,
            author: a.author.unwrap(),
            media_image: a.mediaimage.unwrap(),
            created_on: a.createdon.unwrap()
        }).map_err(|e| ErrorSong(format!("failed to updated song with id = {id} due to the following error: {e:#?}")))
    }

    async fn delete(pool: &sqlx::PgPool, id: i32) -> Result<Self::Response, Self::Error> {
        sqlx::query!(
            "DELETE FROM Songs WHERE id = $1 RETURNING id, title, author, mediaimage, createdon",
            id
        )
        .fetch_one(pool)
        .await
        .map(|a| Self::Response {
            id: a.id,
            title: a.title,
            author: a.author.unwrap(),
            media_image: a.mediaimage.unwrap(),
            created_on: a.createdon.unwrap(),
        })
        .map_err(|e| {
            ErrorSong(format!(
                "failed to delete song with id = {id} due to the following error: {e:#?}"
            ))
        })
    }
}
