use actix_web::{
    HttpResponse, get, post,
    web::{Data, Json, Path},
};
use uuid::Uuid;

use super::{
    model::{CashSession, CloseSessionRequest, OpenSessionRequest, SessionResponse},
    service,
};
use crate::db::AppState;
use crate::shared::errors::AppError;
use crate::shared::repository::PgRepository;

#[utoipa::path(
    post,
    path = "/api/v1/cash/sessions",
    tag = "Cash",
    request_body = OpenSessionRequest,
    responses(
        (status = 201, description = "Session opened", body = SessionResponse),
        (status = 401, description = "Unauthorized"),
    )
)]
#[post("/cash/sessions")]
pub async fn open_session(
    state: Data<AppState>,
    body: Json<OpenSessionRequest>,
) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<CashSession>::new(state.db.clone());
    let res = service::open_session(&repo, body.into_inner()).await?;
    Ok(HttpResponse::Created().json(res))
}

#[utoipa::path(
    post,
    path = "/api/v1/cash/sessions/{id}/close",
    tag = "Cash",
    params(("id" = Uuid, Path, description = "Session ID")),
    request_body = CloseSessionRequest,
    responses(
        (status = 200, description = "Session closed", body = SessionResponse),
        (status = 404, description = "Session not found"),
    )
)]
#[post("/cash/sessions/{id}/close")]
pub async fn close_session(
    state: Data<AppState>,
    path: Path<Uuid>,
    body: Json<CloseSessionRequest>,
) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<CashSession>::new(state.db.clone());
    let id = path.into_inner();
    let res = service::close_session(&repo, id, body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(res))
}

#[utoipa::path(
    get,
    path = "/api/v1/cash/sessions/stores/{store_id}",
    tag = "Cash",
    params(("store_id" = Uuid, Path, description = "Store ID")),
    responses(
        (status = 200, description = "List of sessions", body = Vec<SessionResponse>),
    )
)]
#[get("/cash/sessions/stores/{store_id}")]
pub async fn list_sessions(
    state: Data<AppState>,
    path: Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<CashSession>::new(state.db.clone());
    let store_id = path.into_inner();
    let res = service::list_sessions_by_store(&repo, store_id).await?;
    Ok(HttpResponse::Ok().json(res))
}

#[utoipa::path(
    get,
    path = "/api/v1/cash/sessions/{id}",
    tag = "Cash",
    params(("id" = Uuid, Path, description = "Session ID")),
    responses(
        (status = 200, description = "Session found", body = SessionResponse),
        (status = 404, description = "Session not found"),
    )
)]
#[get("/cash/sessions/{id}")]
pub async fn get_session(
    state: Data<AppState>,
    path: Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<CashSession>::new(state.db.clone());
    let id = path.into_inner();
    let res = service::get_session(&repo, id).await?;
    Ok(HttpResponse::Ok().json(res))
}
