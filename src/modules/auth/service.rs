use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use sqlx::{Pool, Postgres};
use time::OffsetDateTime;
use uuid::Uuid;

use super::{
    model::{Claims, LoginRequest, LoginResponse, RefreshClaims, RefreshRequest, RegisterRequest, RegisterResponse},
    repository,
};
use crate::shared::errors::AppError;

const ACCESS_TOKEN_EXP_SECS: i64 = 60 * 15; // 15 min
const REFRESH_TOKEN_EXP_SECS: i64 = 60 * 60 * 24 * 7; // 7 days

fn make_access_token(user_id: &str, email: &str, secret: &str) -> Result<String, AppError> {
    let exp = (OffsetDateTime::now_utc().unix_timestamp() + ACCESS_TOKEN_EXP_SECS) as usize;
    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        exp,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(e.to_string()))
}

fn make_refresh_token(user_id: &str, secret: &str) -> Result<String, AppError> {
    let exp = (OffsetDateTime::now_utc().unix_timestamp() + REFRESH_TOKEN_EXP_SECS) as usize;
    let claims = RefreshClaims {
        sub: user_id.to_string(),
        exp,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(e.to_string()))
}

pub async fn login(
    pool: &Pool<Postgres>,
    jwt_secret: &str,
    req: LoginRequest,
) -> Result<LoginResponse, AppError> {
    let user = repository::find_by_email(pool, &req.email)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let valid = bcrypt::verify(&req.password, &user.password)
        .map_err(|e| AppError::Internal(e.to_string()))?;

    if !valid {
        return Err(AppError::Unauthorized);
    }

    let user_id = user.id.to_string();
    let access_token = make_access_token(&user_id, &user.email, jwt_secret)?;
    let refresh_token = make_refresh_token(&user_id, jwt_secret)?;

    Ok(LoginResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
    })
}

pub async fn refresh(
    pool: &Pool<Postgres>,
    jwt_secret: &str,
    req: RefreshRequest,
) -> Result<LoginResponse, AppError> {
    let token_data = decode::<RefreshClaims>(
        &req.refresh_token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| AppError::Unauthorized)?;

    let user_id = Uuid::parse_str(&token_data.claims.sub).map_err(|_| AppError::Unauthorized)?;

    let user = repository::find_by_id(pool, user_id)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let user_id_str = user.id.to_string();
    let access_token = make_access_token(&user_id_str, &user.email, jwt_secret)?;
    let refresh_token = make_refresh_token(&user_id_str, jwt_secret)?;

    Ok(LoginResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
    })
}

pub async fn register(
    pool: &Pool<Postgres>,
    req: RegisterRequest,
) -> Result<RegisterResponse, AppError> {
    if repository::email_exists(pool, &req.email).await? {
        return Err(AppError::Conflict("Email already in use".to_string()));
    }

    let password_hash = bcrypt::hash(&req.password, bcrypt::DEFAULT_COST)
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let user = repository::create_user(
        pool,
        &req.name,
        req.second_name.as_deref(),
        req.first_surname.as_deref(),
        req.second_surname.as_deref(),
        &req.email,
        &password_hash,
    )
    .await?;

    Ok(RegisterResponse {
        id: user.id.to_string(),
        email: user.email,
    })
}
