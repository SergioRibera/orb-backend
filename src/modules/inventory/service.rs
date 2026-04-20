use rust_decimal::Decimal;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use uuid::Uuid;

use super::{
    model::{CreateMovementRequest, MovementResponse, StockResponse},
    repository::InventoryRepository,
};
use crate::shared::errors::AppError;

pub async fn create_movement(
    repo: &impl InventoryRepository,
    req: CreateMovementRequest,
) -> Result<MovementResponse, AppError> {
    let product_id = Uuid::parse_str(&req.product_id)
        .map_err(|_| AppError::Internal("Invalid product_id".into()))?;
    let store_id = Uuid::parse_str(&req.store_id)
        .map_err(|_| AppError::Internal("Invalid store_id".into()))?;
    let reference_id = req
        .reference_id
        .as_deref()
        .map(Uuid::parse_str)
        .transpose()
        .map_err(|_| AppError::Internal("Invalid reference_id".into()))?;
    let raw = Decimal::from_f64(req.quantity).unwrap_or_default();
    let quantity = match req.r#type.as_str() {
        "sale" | "return_out" => -raw,
        "adjustment" => {
            let current =
                Decimal::from_f64(repo.stock_by_product_store(product_id, store_id).await?)
                    .unwrap_or_default();
            raw - current
        }
        _ => raw, // purchase, return_in
    };

    let m = repo
        .create(product_id, store_id, &req.r#type, quantity, reference_id)
        .await?;

    Ok(MovementResponse {
        id: m.id.to_string(),
        product_id: m.product_id.to_string(),
        store_id: m.store_id.to_string(),
        r#type: m.r#type,
        quantity: m.quantity.to_f64().unwrap_or(0.0),
        reference_id: m.reference_id.map(|u| u.to_string()),
    })
}

pub async fn list_by_store(
    repo: &impl InventoryRepository,
    store_id: Uuid,
) -> Result<Vec<MovementResponse>, AppError> {
    let movements = repo.list_by_store(store_id).await?;
    Ok(movements
        .into_iter()
        .map(|m| MovementResponse {
            id: m.id.to_string(),
            product_id: m.product_id.to_string(),
            store_id: m.store_id.to_string(),
            r#type: m.r#type,
            quantity: m.quantity.to_f64().unwrap_or(0.0),
            reference_id: m.reference_id.map(|u| u.to_string()),
        })
        .collect())
}

pub async fn get_stock(
    repo: &impl InventoryRepository,
    product_id: Uuid,
    store_id: Uuid,
) -> Result<StockResponse, AppError> {
    let quantity = repo.stock_by_product_store(product_id, store_id).await?;
    Ok(StockResponse {
        product_id: product_id.to_string(),
        store_id: store_id.to_string(),
        quantity,
    })
}
