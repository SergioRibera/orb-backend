use rust_decimal::Decimal;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use uuid::Uuid;

use super::{
    model::{
        CategoryResponse, CreateCategoryRequest, CreateProductRequest, ProductPriceResponse,
        ProductResponse, SetProductPriceRequest, UpdateProductRequest,
    },
    repository::{CategoryRepository, ProductRepository},
};
use crate::shared::errors::AppError;

pub async fn create_category(
    repo: &impl CategoryRepository,
    req: CreateCategoryRequest,
) -> Result<CategoryResponse, AppError> {
    let cat = repo.create(&req.name).await?;
    Ok(CategoryResponse {
        id: cat.id.to_string(),
        name: cat.name,
    })
}

pub async fn list_categories(
    repo: &impl CategoryRepository,
) -> Result<Vec<CategoryResponse>, AppError> {
    let cats = repo.list().await?;
    Ok(cats
        .into_iter()
        .map(|c| CategoryResponse {
            id: c.id.to_string(),
            name: c.name,
        })
        .collect())
}

pub async fn create_product(
    repo: &impl ProductRepository,
    req: CreateProductRequest,
) -> Result<ProductResponse, AppError> {
    let cost = req.cost.and_then(Decimal::from_f64);
    let p = repo
        .create(
            &req.name,
            req.barcode.as_deref(),
            req.description.as_deref(),
            cost,
        )
        .await?;
    if let Some(cat_ids) = req.category_ids {
        for id_str in cat_ids {
            if let Ok(cat_id) = Uuid::parse_str(&id_str) {
                let _ = repo.assign_category(p.id, cat_id).await;
            }
        }
    }
    Ok(ProductResponse {
        id: p.id.to_string(),
        name: p.name,
        barcode: p.barcode,
        description: p.description,
        cost: p.cost.and_then(|d| d.to_f64()),
    })
}

pub async fn update_product(
    repo: &impl ProductRepository,
    id: Uuid,
    req: UpdateProductRequest,
) -> Result<ProductResponse, AppError> {
    let cost = req.cost.and_then(Decimal::from_f64);
    let p = repo
        .update(
            id,
            req.name.as_deref(),
            req.barcode.as_deref(),
            req.description.as_deref(),
            cost,
        )
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(ProductResponse {
        id: p.id.to_string(),
        name: p.name,
        barcode: p.barcode,
        description: p.description,
        cost: p.cost.and_then(|d| d.to_f64()),
    })
}

pub async fn list_products(
    repo: &impl ProductRepository,
) -> Result<Vec<ProductResponse>, AppError> {
    let products = repo.list().await?;
    Ok(products
        .into_iter()
        .map(|p| ProductResponse {
            id: p.id.to_string(),
            name: p.name,
            barcode: p.barcode,
            description: p.description,
            cost: p.cost.and_then(|d| d.to_f64()),
        })
        .collect())
}

pub async fn assign_category(
    repo: &impl ProductRepository,
    product_id: Uuid,
    category_id: Uuid,
) -> Result<(), AppError> {
    repo.assign_category(product_id, category_id).await
}

pub async fn remove_category(
    repo: &impl ProductRepository,
    product_id: Uuid,
    category_id: Uuid,
) -> Result<(), AppError> {
    repo.remove_category(product_id, category_id).await
}

pub async fn set_price(
    repo: &impl ProductRepository,
    product_id: Uuid,
    store_id: Uuid,
    req: SetProductPriceRequest,
) -> Result<ProductPriceResponse, AppError> {
    let price = Decimal::from_f64(req.price).unwrap_or_default();
    let pp = repo.set_price(product_id, store_id, price).await?;
    Ok(ProductPriceResponse {
        id: pp.id.to_string(),
        product_id: pp.product_id.to_string(),
        store_id: pp.store_id.to_string(),
        price: pp.price.to_f64().unwrap_or(0.0),
    })
}
