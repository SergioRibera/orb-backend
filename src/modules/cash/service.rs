use rust_decimal::Decimal;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use uuid::Uuid;

use super::{
    model::{CashSession, CloseSessionRequest, OpenSessionRequest, SessionResponse},
    repository::CashRepository,
};
use crate::shared::errors::AppError;

fn session_to_response(s: &CashSession) -> SessionResponse {
    SessionResponse {
        id: s.id.to_string(),
        store_id: s.store_id.to_string(),
        user_id: s.user_id.to_string(),
        device_id: s.device_id.to_string(),
        status: if s.closed_at.is_none() {
            "open".into()
        } else {
            "closed".into()
        },
        opened_at: s.opened_at.to_string(),
        closed_at: s.closed_at.map(|t| t.to_string()),
        opening_amount: s.opening_amount.to_f64().unwrap_or(0.0),
        closing_amount: s.closing_amount.as_ref().and_then(|d| d.to_f64()),
        expected_amount: s.expected_amount.as_ref().and_then(|d| d.to_f64()),
    }
}

pub async fn open_session(
    repo: &impl CashRepository,
    req: OpenSessionRequest,
) -> Result<SessionResponse, AppError> {
    let store_id = Uuid::parse_str(&req.store_id)
        .map_err(|_| AppError::Internal("Invalid store_id".into()))?;
    let user_id =
        Uuid::parse_str(&req.user_id).map_err(|_| AppError::Internal("Invalid user_id".into()))?;
    let device_id = Uuid::parse_str(&req.device_id)
        .map_err(|_| AppError::Internal("Invalid device_id".into()))?;
    let opening_amount = Decimal::from_f64(req.opening_amount).unwrap_or_default();

    let session = repo
        .open(store_id, user_id, device_id, opening_amount)
        .await?;
    Ok(session_to_response(&session))
}

pub async fn close_session(
    repo: &impl CashRepository,
    id: Uuid,
    req: CloseSessionRequest,
) -> Result<SessionResponse, AppError> {
    let closing_amount = Decimal::from_f64(req.closing_amount).unwrap_or_default();
    let expected_amount = Decimal::from_f64(req.expected_amount).unwrap_or_default();
    let session = repo.close(id, closing_amount, expected_amount).await?;
    Ok(session_to_response(&session))
}

pub async fn get_session(
    repo: &impl CashRepository,
    id: Uuid,
) -> Result<SessionResponse, AppError> {
    let session = repo.find_by_id(id).await?.ok_or(AppError::NotFound)?;
    Ok(session_to_response(&session))
}

pub async fn list_sessions_by_store(
    repo: &impl CashRepository,
    store_id: Uuid,
) -> Result<Vec<SessionResponse>, AppError> {
    let sessions = repo.list_by_store(store_id).await?;
    Ok(sessions.iter().map(session_to_response).collect())
}
