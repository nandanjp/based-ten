use serde::{Deserialize, Serialize};

use crate::utils::traits::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Clone)]
pub struct AuthError(pub String);
impl Error for AuthError {
    fn new(err: String) -> Self {
        Self(err)
    }
}
impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unauthorized, please provide a toke {:#?}", self.0)
    }
}
