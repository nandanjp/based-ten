use crate::{
    models::game::{CreateGame, ErrorGame, Game, GameSortKey, QueryGame, UpdateGame},
    utils::{functions::page_limit, traits::GeneralService},
};
use axum::async_trait;

const NUM_ROWS: i64 = 4999;

pub struct GameService;
#[async_trait]
impl GeneralService for GameService {
    type Response = Game;
    type ListResponse = Vec<Game>;
    type CreateObject = CreateGame;
    type UpdateObject = UpdateGame;
    type QueryObject = QueryGame;
    type Error = ErrorGame;

    async fn get_all(
        pool: &sqlx::PgPool,
        query_obj: Self::QueryObject,
    ) -> Result<Self::ListResponse, Self::Error> {
        let (limit, page) = page_limit(NUM_ROWS, query_obj.limit, query_obj.page);

        match query_obj {
            QueryGame {
                console: Some(console), sort_key: Some(key), ..
            } => match key {
                GameSortKey::Console => sqlx::query_as!(
                    Self::Response,
                    "SELECT * FROM VideoGames WHERE console = $1 ORDER BY console OFFSET $2 LIMIT $3",
                    console,
                    page * limit,
                    limit
                )
                .fetch_all(pool)
                .await
                .map_err(|e| ErrorGame(format!("failed to get all video games: {e:#?}"))),
                GameSortKey::Title => sqlx::query_as!(
                    Self::Response,
                    "SELECT * FROM VideoGames WHERE console = $1 ORDER BY title OFFSET $2 LIMIT $3",
                    console,
                    page * limit,
                    limit
                )
                .fetch_all(pool)
                .await
                .map_err(|e| ErrorGame(format!("failed to get all video games: {e:#?}"))),
            },
            QueryGame {
                sort_key: Some(key), ..
            } => match key {
                GameSortKey::Console => sqlx::query_as!(
                    Self::Response,
                    "SELECT * FROM VideoGames ORDER BY console OFFSET $1 LIMIT $2",
                    page * limit,
                    limit
                )
                .fetch_all(pool)
                .await
                .map_err(|e| ErrorGame(format!("failed to get all video games: {e:#?}"))),
                GameSortKey::Title => sqlx::query_as!(
                    Self::Response,
                    "SELECT * FROM VideoGames ORDER BY title OFFSET $1 LIMIT $2",
                    page * limit,
                    limit
                )
                .fetch_all(pool)
                .await
                .map_err(|e| ErrorGame(format!("failed to get all video games: {e:#?}"))),
            },
            _ => sqlx::query_as!(Self::Response, "SELECT * FROM VideoGames OFFSET $1 LIMIT $2", page * limit, limit)
                .fetch_all(pool)
                .await
                .map_err(|e| ErrorGame(format!("failed to get all video games: {e:#?}"))),
        }
    }

    async fn get_by_id(pool: &sqlx::PgPool, id: i32) -> Result<Self::Response, Self::Error> {
        sqlx::query_as!(Self::Response, "SELECT * FROM VideoGames WHERE id = $1", id)
            .fetch_one(pool)
            .await
            .map_err(|e| ErrorGame(format!("failed to get video game = {id}: {e:#?}")))
    }

    async fn create(
        pool: &sqlx::PgPool,
        create_obj: Self::CreateObject,
    ) -> Result<Self::Response, Self::Error> {
        sqlx::query_as!(Self::Response, "INSERT INTO VideoGames(id, title, mediaimage, createdon, console) VALUES($1, $2, $3, $4, $5) RETURNING *", create_obj.id, create_obj.title, create_obj.media_image, create_obj.created_on, create_obj.console).fetch_one(pool).await.map_err(|e| ErrorGame(format!("failed to create video game: {e:#?}")))
    }

    async fn update(
        pool: &sqlx::PgPool,
        update_obj: Self::UpdateObject,
        id: i32,
    ) -> Result<Self::Response, Self::Error> {
        let game = Self::get_by_id(&pool, id).await?;

        let title = update_obj.title.unwrap_or(game.title);
        let media_image = update_obj.media_image.unwrap_or(game.mediaimage);
        let console = update_obj.console.unwrap_or(game.console);
        let date = update_obj.created_on.unwrap_or(game.createdon.unwrap());
        sqlx::query_as!(Self::Response, "UPDATE VideoGames SET title = $1, mediaimage = $2, console = $3, createdon = $4 WHERE id = $5 RETURNING *", title, media_image, console, date, id).fetch_one(pool).await.map_err(|e| ErrorGame(format!("failed to update video game: {e:#?}")))
    }

    async fn delete(pool: &sqlx::PgPool, id: i32) -> Result<Self::Response, Self::Error> {
        sqlx::query_as!(
            Self::Response,
            "DELETE FROM VideoGames WHERE id = $1 RETURNING *",
            id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| ErrorGame(format!("failed to delete video game = {id}: {e:#?}")))
    }
}
