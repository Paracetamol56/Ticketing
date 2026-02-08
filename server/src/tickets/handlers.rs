use actix_web::{web, HttpResponse, Responder, ResponseError};
use uuid::Uuid;

use super::service;
use crate::tickets::dto;
use crate::utils::db::Connection;
use crate::utils::pagination::PaginationQuery;
use crate::Pool;

pub async fn get_all(db: web::Data<Pool>, query: web::Query<PaginationQuery>) -> impl Responder {
    let conn: Connection = db.get().expect("Failed to get DB connection");
    let query = query.into_inner();

    if let Err(e) = query.validate() {
        return HttpResponse::BadRequest().body(format!("Invalid pagination parameters: {}", e));
    }

    match service::get_all_tickets(&conn, query.page(), query.limit()) {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => e.error_response(),
    }
}

pub async fn get_by_id(db: web::Data<Pool>, path: web::Path<Uuid>) -> impl Responder {
    let id = path.into_inner();
    let conn: Connection = db.get().expect("Failed to get DB connection");

    match service::get_ticket_by_id(&conn, id) {
        Ok(ticket) => HttpResponse::Ok().json(ticket),
        Err(e) => e.error_response(),
    }
}

pub async fn get_stats(db: web::Data<Pool>) -> impl Responder {
    let conn: Connection = db.get().expect("Failed to get DB connection");

    match service::get_ticket_stats(&conn) {
        Ok(stats) => HttpResponse::Ok().json(serde_json::json!({
            "open": stats.open,
            "pending": stats.pending,
            "closed": stats.closed,
            "total": stats.total,
            "last_at": stats.last_at,
        })),
        Err(e) => e.error_response(),
    }
}

pub async fn post_ticket(
    db: web::Data<Pool>,
    body: web::Json<dto::PostTicketRequest>,
) -> impl Responder {
    let body = body.into_inner();
    let conn: Connection = db.get().expect("Failed to get DB connection");

    match service::create_ticket(&conn, body).await {
        Ok(ticket) => HttpResponse::Created().json(ticket),
        Err(e) => e.error_response(),
    }
}

pub async fn patch_ticket(
    db: web::Data<Pool>,
    path: web::Path<Uuid>,
    body: web::Json<dto::PatchTicketRequest>,
) -> impl Responder {
    let id = path.into_inner();
    let body = body.into_inner();
    let conn: Connection = db.get().expect("Failed to get DB connection");

    match service::update_ticket(&conn, id, body).await {
        Ok(ticket) => HttpResponse::Ok().json(ticket),
        Err(e) => e.error_response(),
    }
}

pub async fn delete_ticket(db: web::Data<Pool>, path: web::Path<Uuid>) -> impl Responder {
    let id = path.into_inner();
    let conn: Connection = db.get().expect("Failed to get DB connection");

    match service::delete_ticket(&conn, id) {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(e) => e.error_response(),
    }
}

pub async fn purge_tickets(db: web::Data<Pool>) -> impl Responder {
    let conn: Connection = db.get().expect("Failed to get DB connection");

    match service::purge_tickets(&conn) {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(e) => e.error_response(),
    }
}
