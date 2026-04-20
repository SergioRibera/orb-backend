pub mod handler;
pub mod model;
pub mod repository;
pub mod service;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(handler::create_customer)
        .service(handler::list_customers)
        .service(handler::get_customer);
}
