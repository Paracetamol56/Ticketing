use super::handlers;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/status").route(web::get().to(handlers::get_status)));
}
