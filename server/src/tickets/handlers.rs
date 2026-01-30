use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use uuid::Uuid;

use crate::tickets::repository as tickets_repo;
use crate::utils::brevo::{send_notification, send_ticket};
use crate::utils::db::Connection;
use crate::utils::pagination::{PaginatedResponse, PaginationQuery};
use crate::Pool;

pub async fn get_all(db: web::Data<Pool>, query: web::Query<PaginationQuery>) -> impl Responder {
    let conn: Connection = db.get().expect("Failed to get DB connection");
    let query = query.into_inner();
    if let Err(e) = query.validate() {
        return HttpResponse::BadRequest().body(format!("Invalid pagination parameters: {}", e));
    }

    match tickets_repo::get_all(&conn, query.page(), query.limit()) {
        Ok(tickets) => HttpResponse::Ok().json(PaginatedResponse::new(
            tickets,
            query.page(),
            query.limit(),
            tickets_repo::get_count(&conn).unwrap_or(0) as u64,
        )),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error fetching tickets: {}", e))
        }
    }
}

pub async fn get_by_id(db: web::Data<Pool>, path: web::Path<Uuid>) -> impl Responder {
    let id = path.into_inner();
    let conn: Connection = db.get().expect("Failed to get DB connection");

    match tickets_repo::get_by_id(&conn, id) {
        Ok(ticket) => HttpResponse::Ok().json(ticket),
        Err(rusqlite::Error::QueryReturnedNoRows) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error fetching ticket: {}", e)),
    }
}

pub async fn get_stats(db: web::Data<Pool>) -> impl Responder {
    let conn: Connection = db.get().expect("Failed to get DB connection");

    let open_tickets = tickets_repo::get_count_by_status(&conn, "open").unwrap_or(0);
    let pending_tickets = tickets_repo::get_count_by_status(&conn, "pending").unwrap_or(0);
    let closed_tickets = tickets_repo::get_count_by_status(&conn, "closed").unwrap_or(0);
    let total_tickets = open_tickets + pending_tickets + closed_tickets;
    let last_ticket = tickets_repo::get_last(&conn);

    let last_at = match last_ticket {
        Ok(ticket) => Some(ticket.created_at),
        Err(rusqlite::Error::QueryReturnedNoRows) => None,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Error fetching last ticket: {}", e));
        }
    };

    let res = serde_json::json!({
        "open": open_tickets,
        "pending": pending_tickets,
        "closed": closed_tickets,
        "total": total_tickets,
        "last_at": last_at,
    });

    HttpResponse::Ok().json(res)
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

    let max_number = match tickets_repo::get_max_number(&conn) {
        Ok(number) => number.unwrap_or(0),
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Error fetching max ticket number: {}", e))
        }
    };

    let ticket = crate::tickets::models::Ticket {
        uuid: Uuid::new_v4(),
        number: max_number as u32 + 1,
        name: body.name.clone(),
        email: body.email.clone(),
        message: body.message.clone(),
        note: None,
        status: "open".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: None,
        closed_at: None,
    };

    match tickets_repo::create(&conn, &ticket) {
        Ok(()) => {
            let _ = send_ticket(&ticket).await;
            HttpResponse::Created().json(&ticket)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error creating ticket: {}", e)),
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

    // Fetch the existing ticket
    let mut ticket = match tickets_repo::get_by_id(&conn, id) {
        Ok(ticket) => ticket,
        Err(rusqlite::Error::QueryReturnedNoRows) => return HttpResponse::NotFound().finish(),
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Error fetching ticket: {}", e))
        }
    };

    ticket.note = body.note.or(ticket.note);
    ticket.status = body.status.unwrap_or(ticket.status);
    ticket.updated_at = Some(chrono::Utc::now());

    match tickets_repo::update(&conn, &id, &ticket) {
        Ok(()) => {
            if body.notify {
                let _ = send_notification(&ticket).await;
            }
            HttpResponse::NoContent().finish()
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error updating ticket: {}", e)),
    }
}

pub async fn delete_ticket(db: web::Data<Pool>, path: web::Path<Uuid>) -> impl Responder {
    let id = path.into_inner();
    let conn: Connection = db.get().expect("Failed to get DB connection");

    match tickets_repo::delete(&conn, &id) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(rusqlite::Error::QueryReturnedNoRows) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error deleting ticket: {}", e)),
    }
}
