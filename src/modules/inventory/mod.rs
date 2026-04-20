pub mod handler;
pub mod model;
pub mod repository;
pub mod service;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(handler::create_movement)
        .service(handler::list_movements_by_store)
        .service(handler::get_stock);
}
