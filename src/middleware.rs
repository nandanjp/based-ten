use axum::{
    body::Body,
    extract::{Request, State},
    middleware::Next,
    response::IntoResponse,
    Json,
};
use axum_extra::extract::CookieJar;
use http::{header, StatusCode};
use jsonwebtoken::{decode, DecodingKey, Validation};
use sqlx::PgPool;

use crate::{
    models::auth::{AuthError, TokenClaims},
    services::users::UsersService,
};

pub async fn auth(
    cookie_jar: CookieJar,
    State(pool): State<PgPool>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<AuthError>)> {
    let token = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_owned())
                    } else {
                        None
                    }
                })
        });

    let token = token.ok_or_else(|| {
        (
            StatusCode::UNAUTHORIZED,
            Json(AuthError(format!(
                "You are not logged in, please provide a token"
            ))),
        )
    })?;

    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(std::env::var("JWT_SECRET").unwrap().as_ref()),
        &Validation::default(),
    )
    .map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(AuthError(format!("Invalid token"))),
        )
    })?
    .claims;

    let user_name = claims.sub.to_owned();
    let user = UsersService::get_by_id(&pool, user_name)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AuthError(format!(
                    "Failed to retrieve user from database: {e}"
                ))),
            )
        })?;

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}
