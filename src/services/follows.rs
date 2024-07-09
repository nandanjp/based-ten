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
            } => sqlx::query!(r#"SELECT * FROM Follows WHERE following = $1"#, following)
            .fetch_all(pool)
            .await
            .map(|a| {
                a.into_iter()
                    .map(|a| Follow {
                        follower: a.follower,
                        following: a.following,
                    })
                    .collect::<Vec<Follow>>()
            })
            .map_err(|e| {
                FollowError(format!(
                    "failed to retrieve followers where following = {following} due to the following error: {e:#?}"
                ))
            }),
            _ => sqlx::query!(r#"SELECT * FROM Follows"#)
            .fetch_all(pool)
            .await
            .map(|a| {
                a.into_iter()
                    .map(|a| Follow {
                        follower: a.follower,
                        following: a.following,
                    })
                    .collect::<Vec<Follow>>()
            })
            .map_err(|e| {
                FollowError(format!(
                    "failed to retrieve all follows due to the following error: {e:#?}"
                ))
            })
        }
    }

    pub async fn get_by_id(
        pool: &sqlx::PgPool,
        follower: String,
    ) -> Result<Vec<Follow>, FollowError> {
        sqlx::query!(r#"SELECT * FROM Follows WHERE follower = $1"#, follower)
            .fetch_all(pool)
            .await
            .map(|a| {
                a.into_iter()
                    .map(|a| Follow {
                        follower: a.follower,
                        following: a.following,
                    })
                    .collect::<Vec<Follow>>()
            })
            .map_err(|e| {
                FollowError(format!(
                    "failed to retrieve following of a user with user_name = {follower} due to the following error: {e:#?}"
                ))
            })
    }

    pub async fn create(
        pool: &sqlx::PgPool,
        create_obj: CreateFollow,
    ) -> Result<Follow, FollowError> {
        sqlx::query!(r#"INSERT INTO Follows(follower, following) VALUES($1, $2) RETURNING follower, following"#, create_obj.follower, create_obj.following)
        .fetch_one(pool).await.map(|a| Follow {
            follower: a.follower,
            following: a.following,
        }).map_err(|e| FollowError(format!("failed to create follow due to the following error: {e:#?}")))
    }

    pub async fn delete(
        pool: &sqlx::PgPool,
        follower: String,
        following: String,
    ) -> Result<Follow, FollowError> {
        sqlx::query!(r#"DELETE FROM Follows WHERE follower = $1 AND following = $2 RETURNING follower, following"#, follower, following)
        .fetch_one(pool).await.map(|a| Follow {
            follower: a.follower,
            following: a.following,
        }).map_err(|e| FollowError(format!("failed to delete follow with follower = {follower} and following = {following} due to the following error: {e:#?}")))
    }
}
