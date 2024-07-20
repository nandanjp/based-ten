use crate::models::likes::{CreateLike, Like, LikeError, QueryLike};

pub struct LikesService;
impl LikesService {
    pub async fn get_all(
        pool: &sqlx::PgPool,
        query_obj: QueryLike,
    ) -> Result<Vec<Like>, LikeError> {
        match query_obj {
            QueryLike {
                liking_name: Some(liking_name),
            } => sqlx::query!(r#"SELECT * FROM Likes WHERE likingName = $1"#, liking_name)
            .fetch_all(pool)
            .await
            .map(|a| {
                a.into_iter()
                    .map(|a| Like {
                        liker_name: a.likername,
                        liking_name: a.likingname,
                        list_name: a.listname,
                    })
                    .collect::<Vec<Like>>()
            })
            .map_err(|e| {
                LikeError(format!(
                    "failed to retrieve likes where liking_name = {liking_name} due to the following error: {e:#?}"
                ))
            }),
            _ => sqlx::query!(r#"SELECT * FROM Likes"#)
            .fetch_all(pool)
            .await
            .map(|a| {
                a.into_iter()
                    .map(|a| Like {
                        liker_name: a.likername,
                        liking_name: a.likingname,
                        list_name: a.listname,
                    })
                    .collect::<Vec<Like>>()
            })
            .map_err(|e| {
                LikeError(format!(
                    "failed to retrieve all likes due to the following error: {e:#?}"
                ))
            })
        }
    }

    pub async fn get_by_id(
        pool: &sqlx::PgPool,
        liker_name: String,
    ) -> Result<Vec<Like>, LikeError> {
        sqlx::query!(r#"SELECT * FROM Likes WHERE likerName = $1"#, liker_name)
            .fetch_all(pool)
            .await
            .map(|a| {
                a.into_iter()
                    .map(|a| Like {
                        liker_name: a.likername,
                        liking_name: a.likingname,
                        list_name: a.listname,
                    })
                    .collect::<Vec<Like>>()
            })
            .map_err(|e| {
                LikeError(format!(
                    "failed to retrieve likes of a user with user_name = {liker_name} due to the following error: {e:#?}"
                ))
            })
    }

    pub async fn create(pool: &sqlx::PgPool, create_obj: CreateLike) -> Result<Like, LikeError> {
        sqlx::query!(r#"INSERT INTO Likes(likerName, likingName, listName) VALUES($1, $2, $3) RETURNING likerName, likingName, listName"#, create_obj.liker_name, create_obj.liking_name, create_obj.list_name)
        .fetch_one(pool).await.map(|a| Like {
            liker_name: a.likername,
            liking_name: a.likingname,
            list_name: a.listname,
        }).map_err(|e| LikeError(format!("failed to create like due to the following error: {e:#?}")))
    }

    pub async fn delete(
        pool: &sqlx::PgPool,
        liker_name: String,
        liking_name: String,
        list_name: String,
    ) -> Result<Like, LikeError> {
        sqlx::query!(r#"DELETE FROM Likes WHERE likerName = $1 AND likingName = $2 AND listName = $3 RETURNING likerName, likingName, listName"#, liker_name, liking_name, list_name)
        .fetch_one(pool).await.map(|a| Like {
            liker_name: a.likername,
            liking_name: a.likingname,
            list_name: a.listname,
        }).map_err(|e| LikeError(format!("failed to delete like with likerName = {liker_name}, likingName = {liking_name}, and listName = {list_name} due to the following error: {e:#?}")))
    }
}
