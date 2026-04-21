pub mod handler;
pub mod model;
pub mod service;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").service(handler::login).service(handler::refresh));
}
