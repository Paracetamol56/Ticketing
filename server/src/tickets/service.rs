use super::{repository, ServiceError};
use crate::tickets::models::Ticket;
use crate::utils::brevo::{send_notification, send_ticket};
use crate::utils::db::Connection;
use crate::utils::pagination::PaginatedResponse;
use uuid::Uuid;

pub struct TicketStats {
    pub open: i64,
    pub pending: i64,
    pub closed: i64,
    pub total: i64,
    pub last_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub fn get_all_tickets(
    conn: &Connection,
    page: u32,
    limit: u32,
) -> Result<PaginatedResponse<Ticket>, ServiceError> {
    let tickets = repository::get_all(conn, page, limit)?;
    let total = repository::get_count(conn).unwrap_or(0) as u32;

    Ok(PaginatedResponse::new(tickets, page, limit, total))
}

pub fn get_ticket_by_id(conn: &Connection, id: Uuid) -> Result<Ticket, ServiceError> {
    repository::get_by_id(conn, id).map_err(ServiceError::from)
}

pub fn get_ticket_stats(conn: &Connection) -> Result<TicketStats, ServiceError> {
    let open = repository::get_count_by_status(conn, "open").unwrap_or(0);
    let pending = repository::get_count_by_status(conn, "pending").unwrap_or(0);
    let closed = repository::get_count_by_status(conn, "closed").unwrap_or(0);
    let total = open + pending + closed;

    let last_at = match repository::get_last(conn) {
        Ok(ticket) => Some(ticket.created_at),
        Err(rusqlite::Error::QueryReturnedNoRows) => None,
        Err(e) => return Err(ServiceError::Database(e)),
    };

    Ok(TicketStats {
        open,
        pending,
        closed,
        total,
        last_at,
    })
}

pub struct CreateTicketRequest {
    pub name: String,
    pub email: String,
    pub message: String,
}

pub async fn create_ticket(
    conn: &Connection,
    req: CreateTicketRequest,
) -> Result<Ticket, ServiceError> {
    let max_number = repository::get_max_number(conn)?.unwrap_or(0);

    let ticket = Ticket {
        uuid: Uuid::new_v4(),
        number: max_number as u32 + 1,
        name: req.name,
        email: req.email,
        message: req.message,
        note: None,
        status: "open".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: None,
        closed_at: None,
    };

    repository::create(conn, &ticket)?;

    // Send email notification (non-blocking, ignore errors)
    let _ = send_ticket(&ticket).await;

    Ok(ticket)
}

pub struct UpdateTicketRequest {
    pub note: Option<String>,
    pub status: Option<String>,
    pub notify: bool,
}

pub async fn update_ticket(
    conn: &Connection,
    id: Uuid,
    req: UpdateTicketRequest,
) -> Result<Ticket, ServiceError> {
    let mut ticket = repository::get_by_id(conn, id)?;

    // Apply updates
    ticket.note = req.note.or(ticket.note);
    ticket.status = req.status.unwrap_or(ticket.status);
    ticket.updated_at = Some(chrono::Utc::now());

    repository::update(conn, &id, &ticket)?;

    // Send notification if requested (non-blocking, ignore errors)
    if req.notify {
        let _ = send_notification(&ticket).await;
    }

    Ok(ticket)
}

pub fn delete_ticket(conn: &Connection, id: Uuid) -> Result<(), ServiceError> {
    repository::delete(conn, &id)?;
    Ok(())
}
