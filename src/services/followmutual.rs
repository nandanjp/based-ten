use crate::models::followmutual::{FollowMutual, FollowMutualError};

pub struct FollowMutualService;
impl FollowMutualService {
    pub async fn get_by_mutual_follower(
        pool: &sqlx::PgPool,
        user_name: String,
    ) -> Result<Vec<FollowMutual>, FollowMutualError> {
        sqlx::query_as!(
            FollowMutual,
            r#"WITH mutuals AS (
                        SELECT f1.follower
                        FROM Follows f1, Follows f2
                        WHERE f1.following = $1
                        AND f1.follower = f2.following
                        AND f2.follower = $1
                    )
                    SELECT follower,
                    CASE
                        WHEN follower IN (SELECT * FROM mutuals) THEN TRUE
                        ELSE FALSE
                    END as followsBack
                    FROM Follows
                    WHERE following = $1;"#,
            user_name
        )
        .fetch_all(pool)
        .await
        .map_err(|e| {
            FollowMutualError(format!(
                "failed to retrieve mutual followers for {user_name}: {e:#?}"
            ))
        })
    }
}
