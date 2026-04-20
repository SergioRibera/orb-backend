use actix_web::{
    HttpResponse, get, post,
    web::{Data, Json, Path},
};
use uuid::Uuid;

use super::{
    model::{CreatePermissionRequest, CreateRoleRequest, Permission, RegisterRequest, Role, User},
    service,
};
use crate::db::AppState;
use crate::shared::errors::AppError;
use crate::shared::repository::PgRepository;

// ── Users ─────────────────────────────────────────────────────────────────────

#[post("/users")]
pub async fn register(
    state: Data<AppState>,
    body: Json<RegisterRequest>,
) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<User>::new(state.db.clone());
    let res = service::register(&repo, body.into_inner()).await?;
    Ok(HttpResponse::Created().json(res))
}

#[get("/users")]
pub async fn list_users(state: Data<AppState>) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<User>::new(state.db.clone());
    let res = service::list_users(&repo).await?;
    Ok(HttpResponse::Ok().json(res))
}

// ── Roles ─────────────────────────────────────────────────────────────────────

#[post("/roles")]
pub async fn create_role(
    state: Data<AppState>,
    body: Json<CreateRoleRequest>,
) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<Role>::new(state.db.clone());
    let res = service::create_role(&repo, body.into_inner()).await?;
    Ok(HttpResponse::Created().json(res))
}

#[get("/roles")]
pub async fn list_roles(state: Data<AppState>) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<Role>::new(state.db.clone());
    let res = service::list_roles(&repo).await?;
    Ok(HttpResponse::Ok().json(res))
}

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

#[post("/permissions")]
pub async fn create_permission(
    state: Data<AppState>,
    body: Json<CreatePermissionRequest>,
) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<Permission>::new(state.db.clone());
    let res = service::create_permission(&repo, body.into_inner()).await?;
    Ok(HttpResponse::Created().json(res))
}

#[get("/permissions")]
pub async fn list_permissions(state: Data<AppState>) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<Permission>::new(state.db.clone());
    let res = service::list_permissions(&repo).await?;
    Ok(HttpResponse::Ok().json(res))
}
