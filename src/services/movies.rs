use crate::{
    models::movies::{CreateMovie, ErrorMovie, Movie, QueryMovie, UpdateMovie},
    utils::{functions::convert_date, traits::GeneralService},
};
use axum::async_trait;

pub struct MovieService;
#[async_trait]
impl GeneralService for MovieService {
    type Response = Movie;
    type ListResponse = Vec<Movie>;
    type QueryObject = QueryMovie;
    type CreateObject = CreateMovie;
    type UpdateObject = UpdateMovie;
    type Error = ErrorMovie;

    async fn get_all(
        pool: &sqlx::PgPool,
        query_obj: Self::QueryObject,
    ) -> Result<Self::ListResponse, Self::Error> {
        match query_obj {
            _ => sqlx::query!(r#"SELECT * FROM Movies"#)
                .fetch_all(pool)
                .await
                .map(|a| {
                    a.into_iter()
                        .map(|a| Self::Response {
                            id: a.id,
                            title: a.title,
                            media_image: a.mediaimage.unwrap(),
                            created_on: a.createdon.unwrap(),
                        })
                        .collect::<Vec<Movie>>()
                })
                .map_err(|e| {
                    ErrorMovie(format!(
                        "failed to retrieve all movies due to the following error: {e:#?}"
                    ))
                }),
        }
    }

    async fn get_by_id(pool: &sqlx::PgPool, id: i32) -> Result<Self::Response, Self::Error> {
        sqlx::query!("SELECT * FROM Movies WHERE id = $1", id)
            .fetch_one(pool)
            .await
            .map(|a| Self::Response {
                id: a.id,
                title: a.title,
                media_image: a.mediaimage.unwrap(),
                created_on: a.createdon.unwrap(),
            })
            .map_err(|e| {
                ErrorMovie(format!(
                    "failed to retrieve movie with id = {id} due to the following error: {e:#?}"
                ))
            })
    }

    async fn create(
        pool: &sqlx::PgPool,
        create_obj: Self::CreateObject,
    ) -> Result<Self::Response, Self::Error> {
        let date = convert_date::<ErrorMovie>(create_obj.created_on)?;
        sqlx::query!("INSERT INTO MOVIES(id, title, mediaimage, createdon) VALUES($1, $2, $3, $4) RETURNING id, title, mediaimage, createdon", create_obj.id, create_obj.title, create_obj.media_image, date).fetch_one(pool).await.map(|a| Self::Response {
            id: a.id,
            title: a.title,
            media_image: a.mediaimage.unwrap(),
            created_on: a.createdon.unwrap()
        }).map_err(|e| ErrorMovie(format!("failed to create a movie with the given values due to the following error: {e:#?}")))
    }

    async fn update(
        pool: &sqlx::PgPool,
        update_obj: Self::UpdateObject,
        id: i32,
    ) -> Result<Self::Response, Self::Error> {
        let movie = Self::get_by_id(&pool, id).await?;
        let title = update_obj.title.unwrap_or(movie.title);
        let media_image = update_obj.media_image.unwrap_or(movie.media_image);
        let created_on = convert_date::<ErrorMovie>(
            update_obj
                .created_on
                .unwrap_or(movie.created_on.to_string()),
        )?;
        sqlx::query!("UPDATE Movies SET title = $1, mediaimage = $2, createdon = $3 WHERE id = $4 RETURNING id, title, mediaimage, createdon", title, media_image, created_on, id).fetch_one(pool).await.map(|a| Self::Response {
            id: a.id,
            title: a.title,
            media_image: a.mediaimage.unwrap(),
            created_on: a.createdon.unwrap()
        }).map_err(|e| ErrorMovie(format!("failed to update movie due to the following error: {e:#?}")))
    }

    async fn delete(pool: &sqlx::PgPool, id: i32) -> Result<Self::Response, Self::Error> {
        sqlx::query!(
            "DELETE FROM Movies WHERE id = $1 RETURNING id, title, mediaimage, createdon",
            id
        )
        .fetch_one(pool)
        .await
        .map(|a| Self::Response {
            id: a.id,
            title: a.title,
            media_image: a.mediaimage.unwrap(),
            created_on: a.createdon.unwrap(),
        })
        .map_err(|e| {
            ErrorMovie(format!(
                "failed to delete movie due to the following error: {e:#?}"
            ))
        })
    }
}
