use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use time::OffsetDateTime;
use uuid::Uuid;

use super::model::{Claims, LoginRequest, LoginResponse, RefreshClaims, RefreshRequest};
use crate::modules::iam::repository::UserRepository;
use crate::shared::errors::AppError;

const ACCESS_TOKEN_EXP_SECS: i64 = 60 * 15;
const REFRESH_TOKEN_EXP_SECS: i64 = 60 * 60 * 24 * 7;

fn make_access_token(
    user_id: &str,
    email: &str,
    roles: Vec<String>,
    permissions: Vec<String>,
    secret: &str,
) -> Result<String, AppError> {
    let exp = (OffsetDateTime::now_utc().unix_timestamp() + ACCESS_TOKEN_EXP_SECS) as usize;
    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        roles,
        permissions,
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
    repo: &impl UserRepository,
    jwt_secret: &str,
    req: LoginRequest,
) -> Result<LoginResponse, AppError> {
    let user = repo
        .find_by_email(&req.email)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let valid = bcrypt::verify(&req.password, &user.password)
        .map_err(|e| AppError::Internal(e.to_string()))?;

    if !valid {
        return Err(AppError::Unauthorized);
    }

    let access = repo.find_access(user.id).await?;
    let user_id = user.id.to_string();
    let access_token = make_access_token(
        &user_id,
        &user.email,
        access.roles,
        access.permissions,
        jwt_secret,
    )?;
    let refresh_token = make_refresh_token(&user_id, jwt_secret)?;

    Ok(LoginResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
    })
}

pub async fn refresh(
    repo: &impl UserRepository,
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
    let user = repo
        .find_by_id(user_id)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let access = repo.find_access(user.id).await?;
    let user_id_str = user.id.to_string();
    let access_token = make_access_token(
        &user_id_str,
        &user.email,
        access.roles,
        access.permissions,
        jwt_secret,
    )?;
    let refresh_token = make_refresh_token(&user_id_str, jwt_secret)?;

    Ok(LoginResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
    })
}
