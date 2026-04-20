pub mod handler;
pub mod model;
pub mod repository;
pub mod service;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(handler::create_store)
        .service(handler::list_stores)
        .service(handler::create_device)
        .service(handler::list_devices);
}
