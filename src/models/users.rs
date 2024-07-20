use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::utils::traits::{Error, IntoSerial};

pub struct User {
    pub email: String,
    pub user_name: String,
    pub password: String,
    pub created_at: OffsetDateTime,
}

impl IntoSerial for User {
    type Serial = UserSerial;

    fn to_serial(&self) -> Self::Serial {
        Self::Serial {
            email: self.email.clone(),
            user_name: self.user_name.clone(),
            password: self.password.clone(),
            created_at: self.created_at.to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct UserSerial {
    pub email: String,
    pub user_name: String,
    pub password: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct QueryUser {}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub user_name: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateUser {
    pub email: Option<String>,
    pub user_name: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UserError(pub String);
impl Error for UserError {
    fn new(err: String) -> Self {
        Self(err)
    }
}
impl std::fmt::Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "failed to retrieve user due to the following error: {:#?}",
            self.0
        )
    }
}
