use actix_web::{web, HttpResponse, Responder, ResponseError};
use serde::Deserialize;
use uuid::Uuid;

use super::service::{self, CreateTicketRequest, UpdateTicketRequest};
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

#[derive(Debug, Deserialize)]
pub struct PostTicket {
    name: String,
    email: String,
    message: String,
}

pub async fn post_ticket(db: web::Data<Pool>, body: web::Json<PostTicket>) -> impl Responder {
    let body = body.into_inner();
    let conn: Connection = db.get().expect("Failed to get DB connection");

    let req = CreateTicketRequest {
        name: body.name,
        email: body.email,
        message: body.message,
    };

    match service::create_ticket(&conn, req).await {
        Ok(ticket) => HttpResponse::Created().json(ticket),
        Err(e) => e.error_response(),
    }
}

#[derive(Debug, Deserialize)]
pub struct PatchTicket {
    pub note: Option<String>,
    pub status: Option<String>,
    pub notify: bool,
}

pub async fn patch_ticket(
    db: web::Data<Pool>,
    path: web::Path<Uuid>,
    body: web::Json<PatchTicket>,
) -> impl Responder {
    let id = path.into_inner();
    let body = body.into_inner();
    let conn: Connection = db.get().expect("Failed to get DB connection");

    let req = UpdateTicketRequest {
        note: body.note,
        status: body.status,
        notify: body.notify,
    };

    match service::update_ticket(&conn, id, req).await {
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
