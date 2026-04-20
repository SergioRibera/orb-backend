pub mod handler;
pub mod model;
pub mod repository;
pub mod service;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(handler::create_category)
        .service(handler::list_categories)
        .service(handler::create_product)
        .service(handler::update_product)
        .service(handler::list_products)
        .service(handler::assign_category)
        .service(handler::remove_category)
        .service(handler::set_price);
}
