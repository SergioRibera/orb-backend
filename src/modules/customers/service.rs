use uuid::Uuid;

use super::{
    model::{CreateCustomerRequest, CustomerResponse},
    repository::CustomerRepository,
};
use crate::shared::errors::AppError;

fn customer_to_response(c: super::model::Customer) -> CustomerResponse {
    CustomerResponse {
        id: c.id.to_string(),
        name: c.name,
        phone: c.phone,
        email: c.email,
    }
}

pub async fn create_customer(
    repo: &impl CustomerRepository,
    req: CreateCustomerRequest,
) -> Result<CustomerResponse, AppError> {
    let c = repo
        .create(
            req.name.as_deref(),
            req.phone.as_deref(),
            req.email.as_deref(),
        )
        .await?;
    Ok(customer_to_response(c))
}

pub async fn list_customers(
    repo: &impl CustomerRepository,
) -> Result<Vec<CustomerResponse>, AppError> {
    let customers = repo.list().await?;
    Ok(customers.into_iter().map(customer_to_response).collect())
}

pub async fn get_customer(
    repo: &impl CustomerRepository,
    id: Uuid,
) -> Result<CustomerResponse, AppError> {
    let c = repo.find_by_id(id).await?.ok_or(AppError::NotFound)?;
    Ok(customer_to_response(c))
}
