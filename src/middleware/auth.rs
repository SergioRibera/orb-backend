use std::collections::HashSet;

use actix_web::Error;
use actix_web::dev::ServiceRequest;
use actix_web::web::Data;
use jsonwebtoken::{DecodingKey, Validation, decode};

use crate::db::AppState;
use crate::modules::auth::model::Claims;

/// Extracts the set of permissions for the incoming request.
///
/// If `VAULTARA_URL` is configured, the Bearer token is validated against the
/// Vaultara server and permissions come from the token validation response.
/// Otherwise, the token is validated locally (JWT signed with `JWT_SECRET`) and
/// permissions are read directly from the `permissions` claim.
///
/// Returning `Ok(HashSet::new())` on a missing Authorization header allows
/// public endpoints to pass through; protected routes will still get 403 via
/// the `#[actix_web_grants::protect]` macro.
pub async fn extract_permissions(req: &ServiceRequest) -> Result<HashSet<String>, Error> {
    let state = req
        .app_data::<Data<AppState>>()
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("missing app state"))?;

    let token = match bearer_token(req) {
        Some(t) => t,
        None => return Ok(HashSet::new()),
    };

    if let Some(vaultara) = &state.vaultara {
        let validation = vaultara
            .validate_token(&token)
            .await
            .map_err(|_| actix_web::error::ErrorUnauthorized("invalid token"))?;

        if validation.valid {
            return Ok(validation.permissions.into_iter().collect());
        }

        return Err(actix_web::error::ErrorUnauthorized("invalid token"));
    }

    // Fallback: validate local JWT and read permissions from claims.
    decode::<Claims>(
        &token,
        &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims.permissions.into_iter().collect())
    .map_err(|_| actix_web::error::ErrorUnauthorized("invalid token"))
}

fn bearer_token(req: &ServiceRequest) -> Option<String> {
    req.headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .map(|s| s.to_string())
}
