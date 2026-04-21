use std::collections::HashSet;

use actix_web::Error;
use actix_web::dev::ServiceRequest;
use actix_web::web::Data;

use crate::db::AppState;

/// Extracts permissions by validating the Bearer token against Vaultara.
///
/// No token → empty set (public endpoints pass through; protected routes get 403).
/// Invalid or expired token → 401.
pub async fn extract_permissions(req: &ServiceRequest) -> Result<HashSet<String>, Error> {
    let state = req
        .app_data::<Data<AppState>>()
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("missing app state"))?;

    let token = match bearer_token(req) {
        Some(t) => t,
        None => return Ok(HashSet::new()),
    };

    let validation = state
        .vaultara
        .validate_token(&token)
        .await
        .map_err(|_| actix_web::error::ErrorUnauthorized("invalid token"))?;

    if validation.valid {
        return Ok(validation.permissions.into_iter().collect());
    }

    Err(actix_web::error::ErrorUnauthorized("invalid token"))
}

fn bearer_token(req: &ServiceRequest) -> Option<String> {
    req.headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .map(|s| s.to_string())
}
