use actix_web::{
    HttpResponse, get, post,
    web::{Data, Json, Path},
};
use uuid::Uuid;

use super::{
    model::{
        CreatePermissionRequest, CreateRoleRequest, Permission, PermissionResponse,
        RegisterRequest, RegisterResponse, Role, RoleResponse, User, UserResponse,
    },
    service,
};
use crate::db::AppState;
use crate::shared::errors::AppError;
use crate::shared::repository::PgRepository;

// ── Users ─────────────────────────────────────────────────────────────────────

#[utoipa::path(
    post,
    path = "/api/v1/users",
    tag = "IAM",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered", body = RegisterResponse),
        (status = 409, description = "Email already in use"),
    )
)]
#[post("/users")]
pub async fn register(
    state: Data<AppState>,
    body: Json<RegisterRequest>,
) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<User>::new(state.db.clone());
    let res = service::register(&repo, body.into_inner()).await?;
    Ok(HttpResponse::Created().json(res))
}

#[utoipa::path(
    get,
    path = "/api/v1/users",
    tag = "IAM",
    responses(
        (status = 200, description = "List of users", body = Vec<UserResponse>),
    )
)]
#[get("/users")]
pub async fn list_users(state: Data<AppState>) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<User>::new(state.db.clone());
    let res = service::list_users(&repo).await?;
    Ok(HttpResponse::Ok().json(res))
}

// ── Roles ─────────────────────────────────────────────────────────────────────

#[utoipa::path(
    post,
    path = "/api/v1/roles",
    tag = "IAM",
    request_body = CreateRoleRequest,
    responses(
        (status = 201, description = "Role created", body = RoleResponse),
        (status = 409, description = "Conflict"),
    )
)]
#[post("/roles")]
pub async fn create_role(
    state: Data<AppState>,
    body: Json<CreateRoleRequest>,
) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<Role>::new(state.db.clone());
    let res = service::create_role(&repo, body.into_inner()).await?;
    Ok(HttpResponse::Created().json(res))
}

#[utoipa::path(
    get,
    path = "/api/v1/roles",
    tag = "IAM",
    responses(
        (status = 200, description = "List of roles", body = Vec<RoleResponse>),
    )
)]
#[get("/roles")]
pub async fn list_roles(state: Data<AppState>) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<Role>::new(state.db.clone());
    let res = service::list_roles(&repo).await?;
    Ok(HttpResponse::Ok().json(res))
}

#[utoipa::path(
    post,
    path = "/api/v1/users/{user_id}/roles/{role_id}",
    tag = "IAM",
    params(
        ("user_id" = Uuid, Path, description = "User ID"),
        ("role_id" = Uuid, Path, description = "Role ID"),
    ),
    responses(
        (status = 204, description = "Role assigned"),
        (status = 404, description = "Not found"),
    )
)]
#[post("/users/{user_id}/roles/{role_id}")]
pub async fn assign_role_to_user(
    state: Data<AppState>,
    path: Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<Role>::new(state.db.clone());
    let (user_id, role_id) = path.into_inner();
    service::assign_role_to_user(&repo, user_id, role_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[utoipa::path(
    post,
    path = "/api/v1/roles/{role_id}/permissions/{permission_id}",
    tag = "IAM",
    params(
        ("role_id" = Uuid, Path, description = "Role ID"),
        ("permission_id" = Uuid, Path, description = "Permission ID"),
    ),
    responses(
        (status = 204, description = "Permission assigned"),
        (status = 404, description = "Not found"),
    )
)]
#[post("/roles/{role_id}/permissions/{permission_id}")]
pub async fn assign_permission_to_role(
    state: Data<AppState>,
    path: Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<Role>::new(state.db.clone());
    let (role_id, permission_id) = path.into_inner();
    service::assign_permission_to_role(&repo, role_id, permission_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

// ── Permissions ───────────────────────────────────────────────────────────────

#[utoipa::path(
    post,
    path = "/api/v1/permissions",
    tag = "IAM",
    request_body = CreatePermissionRequest,
    responses(
        (status = 201, description = "Permission created", body = PermissionResponse),
        (status = 409, description = "Conflict"),
    )
)]
#[post("/permissions")]
pub async fn create_permission(
    state: Data<AppState>,
    body: Json<CreatePermissionRequest>,
) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<Permission>::new(state.db.clone());
    let res = service::create_permission(&repo, body.into_inner()).await?;
    Ok(HttpResponse::Created().json(res))
}

#[utoipa::path(
    get,
    path = "/api/v1/permissions",
    tag = "IAM",
    responses(
        (status = 200, description = "List of permissions", body = Vec<PermissionResponse>),
    )
)]
#[get("/permissions")]
pub async fn list_permissions(state: Data<AppState>) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<Permission>::new(state.db.clone());
    let res = service::list_permissions(&repo).await?;
    Ok(HttpResponse::Ok().json(res))
}
