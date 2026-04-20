use uuid::Uuid;

use super::{
    model::{CreateDeviceRequest, CreateStoreRequest, DeviceResponse, StoreResponse},
    repository::{DeviceRepository, StoreRepository},
};
use crate::shared::errors::AppError;

pub async fn create_store(
    repo: &impl StoreRepository,
    req: CreateStoreRequest,
) -> Result<StoreResponse, AppError> {
    let store = repo.create(&req.name, req.address.as_deref()).await?;
    Ok(StoreResponse {
        id: store.id.to_string(),
        name: store.name,
        address: store.address,
        created_at: store.created_at.to_string(),
    })
}

pub async fn list_stores(repo: &impl StoreRepository) -> Result<Vec<StoreResponse>, AppError> {
    let stores = repo.list().await?;
    Ok(stores
        .into_iter()
        .map(|s| StoreResponse {
            id: s.id.to_string(),
            name: s.name,
            address: s.address,
            created_at: s.created_at.to_string(),
        })
        .collect())
}

pub async fn create_device(
    repo: &impl DeviceRepository,
    store_id: Uuid,
    req: CreateDeviceRequest,
) -> Result<DeviceResponse, AppError> {
    let device = repo.create(store_id, req.name.as_deref()).await?;
    Ok(DeviceResponse {
        id: device.id.to_string(),
        store_id: device.store_id.to_string(),
        name: device.name,
        created_at: device.created_at.to_string(),
    })
}

pub async fn list_devices(
    repo: &impl DeviceRepository,
    store_id: Uuid,
) -> Result<Vec<DeviceResponse>, AppError> {
    let devices = repo.list_by_store(store_id).await?;
    Ok(devices
        .into_iter()
        .map(|d| DeviceResponse {
            id: d.id.to_string(),
            store_id: d.store_id.to_string(),
            name: d.name,
            created_at: d.created_at.to_string(),
        })
        .collect())
}
