use actix_web::{
    HttpResponse, get, post,
    web::{Data, Json, Path},
};
use uuid::Uuid;

use super::{
    model::{CreateCustomerRequest, Customer, CustomerResponse},
    service,
};
use crate::db::AppState;
use crate::shared::errors::AppError;
use crate::shared::repository::PgRepository;

#[utoipa::path(
    post,
    path = "/api/v1/customers",
    tag = "Customers",
    request_body = CreateCustomerRequest,
    responses(
        (status = 201, description = "Customer created", body = CustomerResponse),
        (status = 409, description = "Conflict"),
    )
)]
#[post("/customers")]
pub async fn create_customer(
    state: Data<AppState>,
    body: Json<CreateCustomerRequest>,
) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<Customer>::new(state.db.clone());
    let res = service::create_customer(&repo, body.into_inner()).await?;
    Ok(HttpResponse::Created().json(res))
}

#[utoipa::path(
    get,
    path = "/api/v1/customers",
    tag = "Customers",
    responses(
        (status = 200, description = "List of customers", body = Vec<CustomerResponse>),
    )
)]
#[get("/customers")]
pub async fn list_customers(state: Data<AppState>) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<Customer>::new(state.db.clone());
    let res = service::list_customers(&repo).await?;
    Ok(HttpResponse::Ok().json(res))
}

#[utoipa::path(
    get,
    path = "/api/v1/customers/{id}",
    tag = "Customers",
    params(("id" = Uuid, Path, description = "Customer ID")),
    responses(
        (status = 200, description = "Customer found", body = CustomerResponse),
        (status = 404, description = "Not found"),
    )
)]
#[get("/customers/{id}")]
pub async fn get_customer(
    state: Data<AppState>,
    path: Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<Customer>::new(state.db.clone());
    let id = path.into_inner();
    let res = service::get_customer(&repo, id).await?;
    Ok(HttpResponse::Ok().json(res))
}
