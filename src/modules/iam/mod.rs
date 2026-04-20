pub mod handler;
pub mod model;
pub mod repository;
pub mod service;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(handler::register)
        .service(handler::list_users)
        .service(handler::create_role)
        .service(handler::list_roles)
        .service(handler::assign_role_to_user)
        .service(handler::assign_permission_to_role)
        .service(handler::create_permission)
        .service(handler::list_permissions);
}
