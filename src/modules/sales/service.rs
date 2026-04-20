use rust_decimal::Decimal;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use uuid::Uuid;

use super::{
    model::{
        CreateSaleRequest, PaymentResponse, SaleDetailResponse, SaleItemResponse, SaleResponse,
    },
    repository::SaleRepository,
};
use crate::shared::errors::AppError;

fn sale_to_response(s: &super::model::Sale) -> SaleResponse {
    SaleResponse {
        id: s.id.to_string(),
        store_id: s.store_id.to_string(),
        user_id: s.user_id.to_string(),
        device_id: s.device_id.to_string(),
        folio: s.folio.clone(),
        total: s.total.to_f64().unwrap_or(0.0),
        status: s.status.clone(),
    }
}

pub async fn create_sale(
    repo: &impl SaleRepository,
    req: CreateSaleRequest,
) -> Result<SaleDetailResponse, AppError> {
    let store_id = Uuid::parse_str(&req.store_id)
        .map_err(|_| AppError::Internal("Invalid store_id".into()))?;
    let user_id =
        Uuid::parse_str(&req.user_id).map_err(|_| AppError::Internal("Invalid user_id".into()))?;
    let device_id = Uuid::parse_str(&req.device_id)
        .map_err(|_| AppError::Internal("Invalid device_id".into()))?;

    let total: Decimal = req
        .items
        .iter()
        .map(|i| {
            let q = Decimal::from_f64(i.quantity).unwrap_or_default();
            let p = Decimal::from_f64(i.price).unwrap_or_default();
            q * p
        })
        .sum();

    let sale = repo
        .create(
            store_id,
            user_id,
            device_id,
            req.folio.as_deref(),
            total,
            "completed",
        )
        .await?;

    let mut item_responses = Vec::new();
    for item in &req.items {
        let product_id = Uuid::parse_str(&item.product_id)
            .map_err(|_| AppError::Internal("Invalid product_id".into()))?;
        let quantity = Decimal::from_f64(item.quantity).unwrap_or_default();
        let price = Decimal::from_f64(item.price).unwrap_or_default();
        let subtotal = quantity * price;
        let si = repo
            .add_item(sale.id, product_id, quantity, price, subtotal)
            .await?;
        item_responses.push(SaleItemResponse {
            id: si.id.to_string(),
            product_id: si.product_id.to_string(),
            quantity: si.quantity.to_f64().unwrap_or(0.0),
            price: si.price.to_f64().unwrap_or(0.0),
            subtotal: si.subtotal.to_f64().unwrap_or(0.0),
        });
    }

    let mut payment_responses = Vec::new();
    for payment in &req.payments {
        let amount = Decimal::from_f64(payment.amount).unwrap_or_default();
        let p = repo.add_payment(sale.id, &payment.method, amount).await?;
        payment_responses.push(PaymentResponse {
            id: p.id.to_string(),
            method: p.method,
            amount: p.amount.to_f64().unwrap_or(0.0),
        });
    }

    Ok(SaleDetailResponse {
        sale: sale_to_response(&sale),
        items: item_responses,
        payments: payment_responses,
    })
}

pub async fn list_by_store(
    repo: &impl SaleRepository,
    store_id: Uuid,
) -> Result<Vec<SaleResponse>, AppError> {
    let sales = repo.list_by_store(store_id).await?;
    Ok(sales.iter().map(sale_to_response).collect())
}

pub async fn get_sale_detail(
    repo: &impl SaleRepository,
    id: Uuid,
) -> Result<SaleDetailResponse, AppError> {
    let sale = repo.find_by_id(id).await?.ok_or(AppError::NotFound)?;
    let items = repo.list_items(sale.id).await?;
    let payments = repo.list_payments(sale.id).await?;

    Ok(SaleDetailResponse {
        sale: sale_to_response(&sale),
        items: items
            .into_iter()
            .map(|si| SaleItemResponse {
                id: si.id.to_string(),
                product_id: si.product_id.to_string(),
                quantity: si.quantity.to_f64().unwrap_or(0.0),
                price: si.price.to_f64().unwrap_or(0.0),
                subtotal: si.subtotal.to_f64().unwrap_or(0.0),
            })
            .collect(),
        payments: payments
            .into_iter()
            .map(|p| PaymentResponse {
                id: p.id.to_string(),
                method: p.method,
                amount: p.amount.to_f64().unwrap_or(0.0),
            })
            .collect(),
    })
}
