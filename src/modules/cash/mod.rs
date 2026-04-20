pub mod handler;
pub mod model;
pub mod repository;
pub mod service;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(handler::open_session)
        .service(handler::close_session)
        .service(handler::list_sessions)
        .service(handler::get_session);
}
