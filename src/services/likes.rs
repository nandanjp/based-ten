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
            } => sqlx::query_as!(
                Like,
                r#"SELECT * FROM Likes WHERE likingName = $1"#,
                liking_name
            )
            .fetch_all(pool)
            .await
            .map_err(|e| LikeError(format!("failed to retrieve of {liking_name}: {e:#?}"))),
            _ => sqlx::query_as!(Like, r#"SELECT * FROM Likes"#)
                .fetch_all(pool)
                .await
                .map_err(|e| LikeError(format!("failed to retrieve all likes: {e:#?}"))),
        }
    }

    pub async fn get_by_id(
        pool: &sqlx::PgPool,
        liker_name: String,
    ) -> Result<Vec<Like>, LikeError> {
        sqlx::query_as!(
            Like,
            r#"SELECT * FROM Likes WHERE likerName = $1"#,
            liker_name
        )
        .fetch_all(pool)
        .await
        .map_err(|e| {
            LikeError(format!(
                "failed to retrieve likes of user {liker_name}: {e:#?}"
            ))
        })
    }

    pub async fn create(
        pool: &sqlx::PgPool,
        username: String,
        create_obj: CreateLike,
    ) -> Result<Like, LikeError> {
        sqlx::query_as!(
            Like,
            r#"INSERT INTO Likes(likerName, likingName, listName) VALUES($1, $2, $3) RETURNING *"#,
            username,
            create_obj.liking_name,
            create_obj.list_name
        )
        .fetch_one(pool)
        .await
        .map_err(|e| LikeError(format!("failed to create like: {e:#?}")))
    }

    pub async fn delete(
        pool: &sqlx::PgPool,
        liker_name: String,
        liking_name: String,
        list_name: String,
    ) -> Result<Like, LikeError> {
        sqlx::query_as!(Like, r#"DELETE FROM Likes WHERE likerName = $1 AND likingName = $2 AND listName = $3 RETURNING *"#, liker_name, liking_name, list_name)
        .fetch_one(pool)
        .await
        .map_err(|e| LikeError(format!("failed to delete: {e:#?}")))
    }
}
