use crate::{
    models::game::{CreateGame, ErrorGame, Game, QueryGame, UpdateGame},
    utils::{functions::convert_date, traits::GeneralService},
};
use axum::async_trait;

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
        match query_obj {
            QueryGame {
                console: Some(console),
            } => sqlx::query!("SELECT * FROM VideoGames WHERE console = $1", console)
                .fetch_all(pool)
                .await
                .map(|a| {
                    a.into_iter()
                        .map(|a| Self::Response {
                            id: a.id,
                            title: a.title,
                            media_image: a.mediaimage.unwrap(),
                            created_on: a.createdon.unwrap(),
                            console: a.console.unwrap(),
                        })
                        .collect::<Vec<Game>>()
                })
                .map_err(|e| {
                    ErrorGame(format!(
                        "failed to retrieve all video games due to the following error: {e:#?}"
                    ))
                }),
            _ => sqlx::query!("SELECT * FROM VideoGames")
                .fetch_all(pool)
                .await
                .map(|a| {
                    a.into_iter()
                        .map(|a| Self::Response {
                            id: a.id,
                            title: a.title,
                            media_image: a.mediaimage.unwrap(),
                            created_on: a.createdon.unwrap(),
                            console: a.console.unwrap(),
                        })
                        .collect::<Vec<Game>>()
                })
                .map_err(|e| {
                    ErrorGame(format!(
                        "failed to retrieve all video games due to the following error: {e:#?}"
                    ))
                }),
        }
    }

    async fn get_by_id(pool: &sqlx::PgPool, id: i32) -> Result<Self::Response, Self::Error> {
        sqlx::query!("SELECT * FROM VideoGames WHERE id = $1", id).fetch_one(pool).await.map(|a| Self::Response {
            id: a.id,
            title: a.title,
            media_image: a.mediaimage.unwrap(),
            created_on: a.createdon.unwrap(),
            console: a.console.unwrap()
        }).map_err(|e| ErrorGame(format!("failed to retrieve video game with id = {id} due to the following error: {e:#?}")))
    }

    async fn create(
        pool: &sqlx::PgPool,
        create_obj: Self::CreateObject,
    ) -> Result<Self::Response, Self::Error> {
        let date = convert_date::<ErrorGame>(create_obj.created_on)?;
        sqlx::query!("INSERT INTO VideoGames(id, title, mediaimage, createdon, console) VALUES($1, $2, $3, $4, $5) RETURNING id, title, mediaimage, createdon, console", create_obj.id, create_obj.title, create_obj.media_image, date, create_obj.console).fetch_one(pool).await.map(|a| Self::Response {
            id: a.id,
            title: a.title,
            media_image: a.mediaimage.unwrap(),
            console: a.console.unwrap(),
            created_on: a.createdon.unwrap()
        }).map_err(|e| ErrorGame(format!("failed to create video game with the given values due to the following error: {e:#?}")))
    }

    async fn update(
        pool: &sqlx::PgPool,
        update_obj: Self::UpdateObject,
        id: i32,
    ) -> Result<Self::Response, Self::Error> {
        let game = Self::get_by_id(&pool, id).await?;

        let title = update_obj.title.unwrap_or(game.title);
        let media_image = update_obj.media_image.unwrap_or(game.media_image);
        let console = update_obj.console.unwrap_or(game.console);
        let date = convert_date::<ErrorGame>(
            update_obj.created_on.unwrap_or(game.created_on.to_string()),
        )?;
        sqlx::query!("UPDATE VideoGames SET title = $1, mediaimage = $2, console = $3, createdon = $4 WHERE id = $5 RETURNING id, title, mediaimage, console, createdon", title, media_image, console, date, id).fetch_one(pool).await.map(|a| Self::Response {
            id: a.id,
            title: a.title,
            media_image: a.mediaimage.unwrap(),
            console: a.console.unwrap(),
            created_on: a.createdon.unwrap()
        }).map_err(|e| ErrorGame(format!("failed to create video game with the given values due to the following error: {e:#?}")))
    }

    async fn delete(pool: &sqlx::PgPool, id: i32) -> Result<Self::Response, Self::Error> {
        sqlx::query!("DELETE FROM VideoGames WHERE id = $1 RETURNING id, title, mediaimage, console, createdon", id).fetch_one(pool).await.map(|a| Self::Response {
            id: a.id,
            title: a.title,
            media_image: a.mediaimage.unwrap(),
            console: a.console.unwrap(),
            created_on: a.createdon.unwrap(),
        }).map_err(|e| ErrorGame(format!("failed to delete video game with id = {id} due to the following error: {e:#?}")))
    }
}
