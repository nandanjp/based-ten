use crate::models::followmutual::{FollowMutual, FollowMutualError};

pub struct FollowMutualService;
impl FollowMutualService {
    pub async fn get_by_mutual_follower(pool: &sqlx::PgPool, follower_email: String) -> Result<Vec<FollowMutual>, FollowMutualError> {
        sqlx::query!(r#"WITH mutuals AS (
                        SELECT f1.followingEmail
                        FROM Follows f1, Follows f2
                        WHERE f1.followerEmail = $1
                        AND f1.followingEmail = f2.followerEmail 
                        AND f2.followingEmail = $1
                    )
                    SELECT followingEmail,
                    CASE 
                        WHEN followingEmail IN (SELECT * FROM mutuals) THEN TRUE
                        ELSE FALSE
                    END as followsBack
                    FROM Follows
                    WHERE followerEmail = $1;"#, follower_email)
            .fetch_all(pool)
            .await
            .map(|a| {
                a.into_iter()
                    .map(|a| FollowMutual {
                        following_email: a.followingemail,
                        follows_back: a.followsback.unwrap_or(false),
                    })
                    .collect::<Vec<FollowMutual>>()
            })
            .map_err(|e| {
                FollowMutualError(format!(
                    "failed to retrieve mutual followers where follower_email = {follower_email} due to the following error: {e:#?}"
                ))
            })
    }
}
