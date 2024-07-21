use crate::{
    models::movies::{CreateMovie, ErrorMovie, Movie, MovieSortKey, QueryMovie, UpdateMovie},
    utils::{functions::page_limit, traits::GeneralService},
};
use axum::async_trait;

const NUM_ROWS: i64 = 499;

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
        let (limit, page) = page_limit(NUM_ROWS, query_obj.limit, query_obj.page);

        match query_obj {
            QueryMovie {
                sort_key: Some(key),
                ..
            } => match key {
                MovieSortKey::Title => sqlx::query_as!(
                    Self::Response,
                    r#"SELECT * FROM Movies ORDER BY title OFFSET $1 LIMIT $2"#,
                    page * limit,
                    limit
                )
                .fetch_all(pool)
                .await
                .map_err(|e| ErrorMovie(format!("failed to retrieve all movies: {e:#?}"))),
            },
            _ => sqlx::query_as!(
                Self::Response,
                r#"SELECT * FROM Movies OFFSET $1 LIMIT $2"#,
                page * limit,
                limit
            )
            .fetch_all(pool)
            .await
            .map_err(|e| ErrorMovie(format!("failed to retrieve all movies: {e:#?}"))),
        }
    }

    async fn get_by_id(pool: &sqlx::PgPool, id: i32) -> Result<Self::Response, Self::Error> {
        sqlx::query_as!(Self::Response, "SELECT * FROM Movies WHERE id = $1", id)
            .fetch_one(pool)
            .await
            .map_err(|e| ErrorMovie(format!("failed to retrieve movie = {id}: {e:#?}")))
    }

    async fn create(
        pool: &sqlx::PgPool,
        create_obj: Self::CreateObject,
    ) -> Result<Self::Response, Self::Error> {
        sqlx::query_as!(Self::Response, "INSERT INTO MOVIES(id, title, mediaimage, createdon) VALUES($1, $2, $3, $4) RETURNING *", create_obj.id, create_obj.title, create_obj.media_image, create_obj.created_on).fetch_one(pool).await.map_err(|e| ErrorMovie(format!("failed to create a movie: {e:#?}")))
    }

    async fn update(
        pool: &sqlx::PgPool,
        update_obj: Self::UpdateObject,
        id: i32,
    ) -> Result<Self::Response, Self::Error> {
        let movie = Self::get_by_id(&pool, id).await?;
        let title = update_obj.title.unwrap_or(movie.title);
        let media_image = update_obj.media_image.unwrap_or(movie.mediaimage);
        let created_on = update_obj.created_on.unwrap_or(movie.createdon.unwrap());
        sqlx::query_as!(Self::Response, "UPDATE Movies SET title = $1, mediaimage = $2, createdon = $3 WHERE id = $4 RETURNING *", title, media_image, created_on, id).fetch_one(pool).await.map_err(|e| ErrorMovie(format!("failed to update movie: {e:#?}")))
    }

    async fn delete(pool: &sqlx::PgPool, id: i32) -> Result<Self::Response, Self::Error> {
        sqlx::query_as!(
            Self::Response,
            "DELETE FROM Movies WHERE id = $1 RETURNING *",
            id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| ErrorMovie(format!("failed to delete movie: {e:#?}")))
    }
}
