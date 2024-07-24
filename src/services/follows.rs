use crate::models::follows::{CreateFollow, Follow, FollowError, QueryFollow};

pub struct FollowsService;
impl FollowsService {
    pub async fn get_all(
        pool: &sqlx::PgPool,
        query_obj: QueryFollow,
    ) -> Result<Vec<Follow>, FollowError> {
        match query_obj {
            QueryFollow {
                following: Some(following),
            } => sqlx::query_as!(
                Follow,
                r#"SELECT * FROM Follows WHERE following = $1"#,
                following
            )
            .fetch_all(pool)
            .await
            .map_err(|e| FollowError(format!("failed to retrieve followers: {e:#?}"))),
            _ => sqlx::query_as!(Follow, r#"SELECT * FROM Follows"#)
                .fetch_all(pool)
                .await
                .map_err(|e| FollowError(format!("failed to retrieve followers: {e:#?}"))),
        }
    }

    pub async fn get_by_id(
        pool: &sqlx::PgPool,
        follower: String,
    ) -> Result<Vec<Follow>, FollowError> {
        sqlx::query_as!(
            Follow,
            r#"SELECT * FROM Follows WHERE follower = $1"#,
            follower
        )
        .fetch_all(pool)
        .await
        .map_err(|e| {
            FollowError(format!(
                "failed to retrieve following of {follower}: {e:#?}"
            ))
        })
    }

    pub async fn create(
        pool: &sqlx::PgPool,
        username: String,
        create_obj: CreateFollow,
    ) -> Result<Follow, FollowError> {
        sqlx::query_as!(
            Follow,
            r#"INSERT INTO Follows(follower, following) VALUES($1, $2) RETURNING *"#,
            username,
            create_obj.following
        )
        .fetch_one(pool)
        .await
        .map_err(|e| FollowError(format!("failed to create follow: {e:#?}")))
    }

    pub async fn delete(
        pool: &sqlx::PgPool,
        follower: String,
        following: String,
    ) -> Result<Follow, FollowError> {
        sqlx::query_as!(
            Follow,
            r#"DELETE FROM Follows WHERE follower = $1 AND following = $2 RETURNING *"#,
            follower,
            following
        )
        .fetch_one(pool)
        .await
        .map_err(|e| FollowError(format!("failed to delete follow: {e:#?}")))
    }
}
