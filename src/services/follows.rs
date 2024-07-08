use crate::models::follows::{CreateFollow, Follow, FollowError, QueryFollow};

pub struct FollowsService;
impl FollowsService {
    pub async fn get_all(
        pool: &sqlx::PgPool,
        query_obj: QueryFollow,
    ) -> Result<Vec<Follow>, FollowError> {
        match query_obj {
            QueryFollow {
                following_email: Some(following_email),
            } => sqlx::query!(r#"SELECT * FROM Follows WHERE followingEmail = $1"#, following_email)
            .fetch_all(pool)
            .await
            .map(|a| {
                a.into_iter()
                    .map(|a| Follow {
                        follower_email: a.followeremail,
                        following_email: a.followingemail,
                    })
                    .collect::<Vec<Follow>>()
            })
            .map_err(|e| {
                FollowError(format!(
                    "failed to retrieve followers where followingEmail = {following_email} due to the following error: {e:#?}"
                ))
            }),
            _ => sqlx::query!(r#"SELECT * FROM Follows"#)
            .fetch_all(pool)
            .await
            .map(|a| {
                a.into_iter()
                    .map(|a| Follow {
                        follower_email: a.followeremail,
                        following_email: a.followingemail,
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

    pub async fn get_by_id(pool: &sqlx::PgPool, email: String) -> Result<Vec<Follow>, FollowError> {
        sqlx::query!(r#"SELECT * FROM Follows WHERE followerEmail = $1"#, email)
            .fetch_all(pool)
            .await
            .map(|a| {
                a.into_iter()
                    .map(|a| Follow {
                        follower_email: a.followeremail,
                        following_email: a.followingemail,
                    })
                    .collect::<Vec<Follow>>()
            })
            .map_err(|e| {
                FollowError(format!(
                    "failed to retrieve following of a user with email = {email} due to the following error: {e:#?}"
                ))
            })
    }

    pub async fn create(
        pool: &sqlx::PgPool,
        create_obj: CreateFollow,
    ) -> Result<Follow, FollowError> {
        sqlx::query!(r#"INSERT INTO Follows(followerEmail, followingEmail) VALUES($1, $2) RETURNING followerEmail, followingEmail"#, create_obj.follower_email, create_obj.following_email)
        .fetch_one(pool).await.map(|a| Follow {
            follower_email: a.followeremail,
            following_email: a.followingemail,
        }).map_err(|e| FollowError(format!("failed to create follow due to the following error: {e:#?}")))
    }

    pub async fn delete(
        pool: &sqlx::PgPool,
        follower_email: String,
        following_email: String,
    ) -> Result<Follow, FollowError> {
        sqlx::query!(r#"DELETE FROM Follows WHERE followerEmail = $1 AND followingEmail = $2 RETURNING followerEmail, followingEmail"#, follower_email, following_email)
        .fetch_one(pool).await.map(|a| Follow {
            follower_email: a.followeremail,
            following_email: a.followingemail,
        }).map_err(|e| FollowError(format!("failed to delete follow with followerEmail = {follower_email} and followingEmail = {following_email} due to the following error: {e:#?}")))
    }
}
