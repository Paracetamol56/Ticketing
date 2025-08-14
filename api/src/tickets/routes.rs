use super::handlers;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/stats")
            .wrap(crate::middlewares::auth::AdminAuth)
            .route(web::get().to(handlers::get_stats)),
    );
    cfg.service(
        web::resource("/tickets")
            .route(web::post().to(handlers::post_ticket))
            .route(web::get().to(handlers::get_all).wrap(crate::middlewares::auth::AdminAuth)),
    );
    cfg.service(
        web::resource("/tickets/{id}")
            .route(web::get().to(handlers::get_by_id))
            .route(web::patch().to(handlers::patch_ticket).wrap(crate::middlewares::auth::AdminAuth))
            .route(web::delete().to(handlers::delete_ticket).wrap(crate::middlewares::auth::AdminAuth)),
    );
}
