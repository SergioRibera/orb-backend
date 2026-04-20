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
    req: RegisterRequest,
) -> Result<RegisterResponse, AppError> {
    if repo.email_exists(&req.email).await? {
        return Err(AppError::Conflict("Email already in use".to_string()));
    }

    let password_hash = bcrypt::hash(&req.password, bcrypt::DEFAULT_COST)
        .map_err(|e| AppError::Internal(e.to_string()))?;

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

pub async fn list_users(repo: &impl UserRepository) -> Result<Vec<UserResponse>, AppError> {
    let users = repo.list().await?;
    Ok(users
        .into_iter()
        .map(|u| UserResponse {
            id: u.id.to_string(),
            name: u.name,
            email: u.email,
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
