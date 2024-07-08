use crate::models::likes::{Like, LikeError, QueryLike, CreateLike};

pub struct LikesService;
impl LikesService {

    pub async fn get_all(
        pool: &sqlx::PgPool,
        query_obj: QueryLike,
    ) -> Result<Vec<Like>, LikeError> {
        match query_obj {
            QueryLike {
                liking_email: Some(liking_email),
            } => sqlx::query!(r#"SELECT * FROM Likes WHERE likingEmail = $1"#, liking_email)
            .fetch_all(pool)
            .await
            .map(|a| {
                a.into_iter()
                    .map(|a| Like {
                        liker_email: a.likeremail,
                        liking_email: a.likingemail,
                        list_name: a.listname,
                    })
                    .collect::<Vec<Like>>()
            })
            .map_err(|e| {
                LikeError(format!(
                    "failed to retrieve likes where likingEmail = {liking_email} due to the following error: {e:#?}"
                ))
            }),
            _ => sqlx::query!(r#"SELECT * FROM Likes"#)
            .fetch_all(pool)
            .await
            .map(|a| {
                a.into_iter()
                    .map(|a| Like {
                        liker_email: a.likeremail,
                        liking_email: a.likingemail,
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

    pub async fn get_by_id(pool: &sqlx::PgPool, email: String) -> Result<Vec<Like>, LikeError> {
        sqlx::query!(r#"SELECT * FROM Likes WHERE likerEmail = $1"#, email)
            .fetch_all(pool)
            .await
            .map(|a| {
                a.into_iter()
                    .map(|a| Like {
                        liker_email: a.likeremail,
                        liking_email: a.likingemail,
                        list_name: a.listname,
                    })
                    .collect::<Vec<Like>>()
            })
            .map_err(|e| {
                LikeError(format!(
                    "failed to retrieve likes of a user with email = {email} due to the following error: {e:#?}"
                ))
            })
    }

    pub async fn create(
        pool: &sqlx::PgPool,
        create_obj: CreateLike,
    ) -> Result<Like, LikeError> {
        sqlx::query!(r#"INSERT INTO Likes(likerEmail, likingEmail, listName) VALUES($1, $2, $3) RETURNING likerEmail, likingEmail, listName"#, create_obj.liker_email, create_obj.liking_email, create_obj.list_name)
        .fetch_one(pool).await.map(|a| Like {
            liker_email: a.likeremail,
            liking_email: a.likingemail,
            list_name: a.listname,
        }).map_err(|e| LikeError(format!("failed to create like due to the following error: {e:#?}")))
    }

    pub async fn delete(pool: &sqlx::PgPool, liker_email: String, liking_email: String, list_name: String) -> Result<Like, LikeError> {
        sqlx::query!(r#"DELETE FROM Likes WHERE likerEmail = $1 AND likingEmail = $2 AND listName = $3 RETURNING likerEmail, likingEmail, listName"#, liker_email, liking_email, list_name)
        .fetch_one(pool).await.map(|a| Like {
            liker_email: a.likeremail,
            liking_email: a.likingemail,
            list_name: a.listname,
        }).map_err(|e| LikeError(format!("failed to delete like with likerEmail = {liker_email}, likingEmail = {liking_email}, and listName = {list_name} due to the following error: {e:#?}")))
    }
}
