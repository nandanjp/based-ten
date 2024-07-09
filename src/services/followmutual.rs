use crate::models::followmutual::{FollowMutual, FollowMutualError};

pub struct FollowMutualService;
impl FollowMutualService {
    pub async fn get_by_mutual_follower(pool: &sqlx::PgPool, user_email: String) -> Result<Vec<FollowMutual>, FollowMutualError> {
        sqlx::query!(r#"WITH mutuals AS (
                        SELECT f1.followerEmail
                        FROM Follows f1, Follows f2
                        WHERE f1.followingEmail = $1
                        AND f1.followerEmail = f2.followingEmail 
                        AND f2.followerEmail = $1
                    )
                    SELECT followerEmail,
                    CASE 
                        WHEN followerEmail IN (SELECT * FROM mutuals) THEN TRUE
                        ELSE FALSE
                    END as followsBack
                    FROM Follows
                    WHERE followingEmail = $1;"#, user_email)
            .fetch_all(pool)
            .await
            .map(|a| {
                a.into_iter()
                    .map(|a| FollowMutual {
                        follower_email: a.followeremail,
                        follows_back: a.followsback.unwrap_or(false),
                    })
                    .collect::<Vec<FollowMutual>>()
            })
            .map_err(|e| {
                FollowMutualError(format!(
                    "failed to retrieve mutual followers for user email = {user_email} due to the following error: {e:#?}"
                ))
            })
    }
}
