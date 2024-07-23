use crate::models::users::{CreateUser, QueryUser, UpdateUser, User, UserError};

pub struct UsersService;
impl UsersService {
    pub async fn get_all(
        pool: &sqlx::PgPool,
        query_obj: QueryUser,
    ) -> Result<Vec<User>, UserError> {
        match query_obj {
            _ => sqlx::query_as!(User, r#"SELECT * FROM Users"#)
                .fetch_all(pool)
                .await
                .map_err(|e| UserError(format!("failed to retrieve all users: {e:#?}"))),
        }
    }

    pub async fn get_by_id(pool: &sqlx::PgPool, user_name: String) -> Result<User, UserError> {
        sqlx::query_as!(
            User,
            r#"SELECT * FROM Users WHERE username = $1"#,
            user_name
        )
        .fetch_one(pool)
        .await
        .map_err(|e| UserError(format!("failed to retrieve a user {user_name}: {e:#?}")))
    }

    pub async fn does_user_exist(
        pool: &sqlx::PgPool,
        user_name: String,
    ) -> Result<Option<bool>, String> {
        sqlx::query!(
            "SELECT EXISTS(SELECT 1 FROM Users WHERE username = $1)",
            user_name
        )
        .fetch_one(pool)
        .await
        .map(|r| r.exists)
        .map_err(|e| format!("Database error: {e}"))
    }

    pub async fn create(pool: &sqlx::PgPool, create_obj: CreateUser) -> Result<User, UserError> {
        sqlx::query_as!(
            User,
            r#"INSERT INTO Users(email, userName, userPassword) VALUES($1, $2, $3) RETURNING *"#,
            create_obj.email,
            create_obj.user_name,
            create_obj.password
        )
        .fetch_one(pool)
        .await
        .map_err(|e| UserError(format!("failed to create user: {e:#?}")))
    }

    pub async fn update(
        pool: &sqlx::PgPool,
        update_obj: UpdateUser,
        user_name: String,
    ) -> Result<User, UserError> {
        let user = Self::get_by_id(&pool, user_name.clone()).await?;
        let new_email = update_obj.email.unwrap_or(user.email);
        let user_name = update_obj.user_name.unwrap_or(user.username);
        let password = update_obj.password.unwrap_or(user.userpassword);

        sqlx::query_as!(User, r#"UPDATE Users SET email = $1, userName = $2, userPassword = $3 WHERE username = $4 RETURNING *"#, new_email, user_name, password, user_name)
        .fetch_one(pool).await.map_err(|e| UserError(format!("failed to update user = {user_name}: {e:#?}")))
    }

    pub async fn delete(pool: &sqlx::PgPool, user_name: String) -> Result<User, UserError> {
        sqlx::query_as!(
            User,
            r#"DELETE FROM Users WHERE userName = $1 RETURNING *"#,
            user_name
        )
        .fetch_one(pool)
        .await
        .map_err(|e| UserError(format!("failed to delete user = {user_name}: {e:#?}")))
    }
}
