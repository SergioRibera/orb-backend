use serde_json::json;
use uuid::Uuid;

use super::{
    model::{
        CreatePermissionRequest, CreateRoleRequest, PermissionResponse, RegisterRequest,
        RegisterResponse, RoleResponse, UserResponse,
    },
    repository::{PermissionRepository, RoleRepository, UserRepository},
};
use crate::shared::errors::AppError;

pub async fn register(
    repo: &impl UserRepository,
    vaultara_url: &str,
    vaultara_api_key: Option<&str>,
    tenant_id: Option<&str>,
    req: RegisterRequest,
) -> Result<RegisterResponse, AppError> {
    if repo.email_exists(&req.email).await? {
        return Err(AppError::Conflict("Email already in use".to_string()));
    }

    let password_hash = bcrypt::hash(&req.password, bcrypt::DEFAULT_COST)
        .map_err(|e| AppError::Internal(e.to_string()))?;

    // Create user in Vaultara if tenant_id is configured
    if let (Some(tenant), Some(api_key)) = (tenant_id, vaultara_api_key) {
        create_vaultara_user(vaultara_url, api_key, tenant, &req).await?;
    }

    let user = repo
        .create(
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

async fn create_vaultara_user(
    vaultara_url: &str,
    api_key: &str,
    tenant_id: &str,
    req: &RegisterRequest,
) -> Result<(), AppError> {
    let url = format!(
        "{}/api/v1/tenants/{}/users",
        vaultara_url.trim_end_matches('/'),
        tenant_id
    );

    let client = reqwest::Client::new();
    let body = json!({
        "username": req.email,
        "email": req.email,
        "password": req.password,
        "first_name": req.name,
        "last_name": req.first_surname.as_deref().unwrap_or(""),
        "status": "active",
        "email_verified": false,
    });

    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&body)
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("Failed to create Vaultara user: {}", e)))?;

    if !response.status().is_success() {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(AppError::Internal(format!("Vaultara user creation failed: {}", error_text)));
    }

    Ok(())
}

pub async fn list_users(repo: &impl UserRepository) -> Result<Vec<UserResponse>, AppError> {
    let users = repo.list().await?;
    Ok(users
        .into_iter()
        .map(|u| UserResponse {
            id: u.id.to_string(),
            name: u.name,
            email: u.email,
            created_at: u.created_at.to_string(),
        })
        .collect())
}

// ── Roles ─────────────────────────────────────────────────────────────────────

pub async fn create_role(
    repo: &impl RoleRepository,
    req: CreateRoleRequest,
) -> Result<RoleResponse, AppError> {
    let role = repo.create(&req.name).await?;
    Ok(RoleResponse {
        id: role.id.to_string(),
        name: role.name,
    })
}

pub async fn list_roles(repo: &impl RoleRepository) -> Result<Vec<RoleResponse>, AppError> {
    let roles = repo.list().await?;
    Ok(roles
        .into_iter()
        .map(|r| RoleResponse {
            id: r.id.to_string(),
            name: r.name,
        })
        .collect())
}

pub async fn assign_role_to_user(
    repo: &impl RoleRepository,
    user_id: Uuid,
    role_id: Uuid,
) -> Result<(), AppError> {
    repo.assign_to_user(user_id, role_id).await
}

pub async fn assign_permission_to_role(
    repo: &impl RoleRepository,
    role_id: Uuid,
    permission_id: Uuid,
) -> Result<(), AppError> {
    repo.assign_permission(role_id, permission_id).await
}

// ── Permissions ───────────────────────────────────────────────────────────────

pub async fn create_permission(
    repo: &impl PermissionRepository,
    req: CreatePermissionRequest,
) -> Result<PermissionResponse, AppError> {
    let perm = repo.create(&req.name).await?;
    Ok(PermissionResponse {
        id: perm.id.to_string(),
        name: perm.name,
    })
}

pub async fn list_permissions(
    repo: &impl PermissionRepository,
) -> Result<Vec<PermissionResponse>, AppError> {
    let perms = repo.list().await?;
    Ok(perms
        .into_iter()
        .map(|p| PermissionResponse {
            id: p.id.to_string(),
            name: p.name,
        })
        .collect())
}
