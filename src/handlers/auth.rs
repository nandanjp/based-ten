use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{extract::State, response::IntoResponse, Json};
use axum_extra::extract::cookie::{Cookie, SameSite};
use http::{header, Response, StatusCode};
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::PgPool;

use crate::{
    models::{
        auth::TokenClaims,
        users::{CreateUser, LoginUserSchema},
    },
    services::users::UsersService,
};

pub async fn register_user_handler(
    State(pool): State<PgPool>,
    Json(body): Json<CreateUser>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let exists = sqlx::query!(
        "SELECT EXISTS(SELECT 1 FROM Users WHERE username = $1)",
        body.user_name.clone()
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "success": false,
                "response": false,
                "error": format!("Database error: {}", e)
            })),
        )
    })?
    .exists;

    if let Some(true) = exists {
        return Err((
            StatusCode::CONFLICT,
            Json(serde_json::json!({
                "success": false,
                "response": false,
                "error": format!("User with the given user_name already exists!")
            })),
        ));
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "success": false,
                    "response": false,
                    "error": format!("Error while hashing password {}", e)
                })),
            )
        })
        .map(|hash| hash.to_string())?;

    let body = CreateUser {
        email: body.email.to_owned(),
        password: hashed_password,
        user_name: body.user_name.to_owned(),
    };
    let user = UsersService::create(&pool, body).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "success": false,
                "response": false,
                "error": format!("Failed to create user due to the following error: {}", e)
            })),
        )
    })?;

    Ok(Json(serde_json::json!({
        "success": false,
        "response": serde_json::json!({
            "email": user.email.to_owned(),
            "user_name": user.user_name.to_owned(),
            "password": user.password.to_owned(),
            "created_at": user.created_at.to_string(),
        }),
        "error": format!("User with the given user_name already exists!")
    })))
}

pub async fn login_user_handler(
    State(pool): State<PgPool>,
    Json(body): Json<LoginUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user = UsersService::get_by_id(&pool, body.user_name.clone())
        .await
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "success": false,
                    "response": false,
                    "error": format!("Invalid email or password: {}", e)
                })),
            )
        })?;

    if !match PasswordHash::new(&user.password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    } {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "success": false,
                "response": false,
                "error": format!("Invalid email or password")
            })),
        ));
    }

    let now = chrono::Utc::now();
    let claims = TokenClaims {
        sub: user.user_name.clone(),
        exp: (now + chrono::Duration::minutes(60)).timestamp() as usize,
        iat: now.timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(std::env::var("JWT_SECRET").unwrap().as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build(("token", token.to_owned()))
        .path("/")
        .max_age(time::Duration::hours(1))
        .same_site(SameSite::Lax)
        .http_only(true);

    let mut response = Response::new(
        serde_json::json!({
            "success": true,
            "token": token
        })
        .to_string(),
    );
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    Ok(response)
}

pub async fn logout_handler() -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let cookie = Cookie::build(("token", ""))
        .path("/")
        .max_age(time::Duration::hours(-1))
        .same_site(SameSite::Lax)
        .http_only(true);

    let mut response = Response::new(serde_json::json!({"success": true}).to_string());
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    Ok(response)
}
