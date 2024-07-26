use super::traits::Commit;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct User {
    pub user_email: String,
    pub user_name: String,
    pub password: String,
}

impl Commit for User {
    type Value = Self;

    async fn commit(
        pool: &sqlx::PgPool,
        values: Vec<Self::Value>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let user_emails = values
            .iter()
            .map(|u| u.user_email.clone())
            .collect::<Vec<String>>();
        let user_names = values
            .iter()
            .map(|u| u.user_name.clone())
            .collect::<Vec<String>>();
        let passwords = values
            .iter()
            .map(|u| {
                let salt = SaltString::generate(&mut OsRng);
                let password = u.password.clone();
                Argon2::default()
                    .hash_password(password.as_bytes(), &salt)
                    .map(|p| p.to_string())
                    .map_err(|_| {
                        <&str as Into<Box<dyn std::error::Error>>>::into("failed to hash password")
                    })
            })
            .collect::<Result<Vec<String>, Box<dyn std::error::Error>>>()
            .map_err(|_| {
                <&str as Into<Box<dyn std::error::Error>>>::into("failed to hash password")
            })?;
        let _ = sqlx::query!(
            r#"
            INSERT INTO Users(email, username, userpassword)
            SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[])
        "#,
            &user_emails[..],
            &user_names[..],
            &passwords[..]
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct Follows {
    pub follower: String,
    pub following: String,
}

impl Commit for Follows {
    type Value = Self;

    async fn commit(
        pool: &sqlx::PgPool,
        values: Vec<Self::Value>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let followers = values
            .iter()
            .map(|f| f.follower.clone())
            .collect::<Vec<String>>();
        let followings = values
            .iter()
            .map(|f| f.following.clone())
            .collect::<Vec<String>>();
        let _ = sqlx::query!(
            r#"
            INSERT INTO Follows(follower, following)
            SELECT * FROM UNNEST($1::text[], $2::text[])
        "#,
            &followers[..],
            &followings[..]
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct GroupMember {
    pub member_name: String,
    pub group_id: i32,
}

impl Commit for GroupMember {
    type Value = Self;

    async fn commit(
        pool: &sqlx::PgPool,
        values: Vec<Self::Value>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let group_ids = values
            .iter()
            .map(|g| g.group_id as i64)
            .collect::<Vec<i64>>();
        let user_names = values
            .iter()
            .map(|g| g.member_name.clone())
            .collect::<Vec<String>>();
        let _ = sqlx::query!(
            r#"
            INSERT INTO GroupMembers(gid, username)
            SELECT * FROM UNNEST($1::int8[], $2::text[])
        "#,
            &group_ids[..],
            &user_names[..]
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct Group {
    pub group_id: i32,
    pub name: String,
    pub user_name: String,
}

impl Commit for Group {
    type Value = Self;

    async fn commit(
        pool: &sqlx::PgPool,
        values: Vec<Self::Value>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let group_names = values
            .iter()
            .map(|g| g.name.clone())
            .collect::<Vec<String>>();
        let owners = values
            .iter()
            .map(|g| g.user_name.clone())
            .collect::<Vec<String>>();
        let _ = sqlx::query!(
            r#"
            INSERT INTO Groups(groupname, ownedby)
            SELECT * FROM UNNEST($1::text[], $2::text[])
        "#,
            &group_names[..],
            &owners[..]
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct Likes {
    pub liker_name: String,
    pub liking_name: String,
    pub list_name: String,
}

impl Commit for Likes {
    type Value = Self;

    async fn commit(
        pool: &sqlx::PgPool,
        values: Vec<Self::Value>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let liker_names = values
            .iter()
            .map(|l| l.liker_name.clone())
            .collect::<Vec<String>>();
        let liking_names = values
            .iter()
            .map(|l| l.liking_name.clone())
            .collect::<Vec<String>>();
        let list_names = values
            .iter()
            .map(|l| l.list_name.clone())
            .collect::<Vec<String>>();
        let _ = sqlx::query!(
            r#"
            INSERT INTO Likes(likername, likingname, listname)
            SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[])
        "#,
            &liker_names[..],
            &liking_names[..],
            &list_names[..]
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}
