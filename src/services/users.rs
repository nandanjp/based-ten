use crate::models::users::{User, UserError, QueryUser, CreateUser, UpdateUser};

pub struct UsersService;
impl UsersService {

    pub async fn get_all(
        pool: &sqlx::PgPool,
        query_obj: QueryUser,
    ) -> Result<Vec<User>, UserError> {
        match query_obj {
            _ => sqlx::query!(r#"SELECT * FROM Users"#)
            .fetch_all(pool)
            .await
            .map(|a| {
                a.into_iter()
                    .map(|a| User {
                        email: a.email,
                        display_name: a.displayname,
                        password: a.userpassword,
                        created_at: a.createdat.unwrap(),
                    })
                    .collect::<Vec<User>>()
            })
            .map_err(|e| {
                UserError(format!(
                    "failed to retrieve all users due to the following error: {e:#?}"
                ))
            })
        }
    }

    pub async fn get_by_id(pool: &sqlx::PgPool, email: String) -> Result<User, UserError> {
        sqlx::query!(r#"SELECT * FROM Users WHERE email = $1"#, email)
            .fetch_one(pool)
            .await
            .map(|a| User {
                email: a.email,
                display_name: a.displayname,
                password: a.userpassword,
                created_at: a.createdat.unwrap(),
            })
            .map_err(|e| {
                UserError(format!(
                    "failed to retrieve a user with email = {email} due to the following error: {e:#?}"
                ))
            })
    }

    pub async fn create(
        pool: &sqlx::PgPool,
        create_obj: CreateUser,
    ) -> Result<User, UserError> {
        sqlx::query!(r#"INSERT INTO Users(email, displayName, userPassword) VALUES($1, $2, $3) RETURNING email, displayName, userPassword, createdAt"#, create_obj.email, create_obj.display_name, create_obj.password)
        .fetch_one(pool).await.map(|a| User {
            email: a.email,
            display_name: a.displayname,
            password: a.userpassword,
            created_at: a.createdat.unwrap(),
        }).map_err(|e| UserError(format!("failed to create user due to the following error: {e:#?}")))
    }

    pub async fn update(
        pool: &sqlx::PgPool,
        update_obj: UpdateUser,
        email: String,
    ) -> Result<User, UserError> {
        let user = Self::get_by_id(&pool, email.clone()).await?;
        let new_email = update_obj.email.unwrap_or(user.email);
        let display_name = update_obj.display_name.unwrap_or(user.display_name);
        let password = update_obj.password.unwrap_or(user.password);

        sqlx::query!(r#"UPDATE Users SET email = $1, displayName = $2, userPassword = $3 WHERE email = $4 RETURNING email, displayName, userPassword, createdAt"#, new_email, display_name, password, email)
        .fetch_one(pool).await.map(|a| User {
            email: a.email,
            display_name: a.displayname,
            password: a.userpassword,
            created_at: a.createdat.unwrap(),
        }).map_err(|e| UserError(format!("failed to update user with the given email = {email} due to the following error: {e:#?}")))
    }

    pub async fn delete(pool: &sqlx::PgPool, email: String) -> Result<User, UserError> {
        sqlx::query!(r#"DELETE FROM Users WHERE email = $1 RETURNING email, displayName, userPassword, createdAt"#, email).fetch_one(pool).await.map(|a| User {
            email: a.email,
            display_name: a.displayname,
            password: a.userpassword,
            created_at: a.createdat.unwrap(),
        }).map_err(|e| UserError(format!("failed to delete user with the given email = {email} due to the following error: {e:#?}")))
    }
}
