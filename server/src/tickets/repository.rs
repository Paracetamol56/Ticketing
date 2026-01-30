use rusqlite::params;
use uuid::Uuid;

use super::models::Ticket;
use crate::utils::db::Connection;

fn parse_uuid(uuid_str: &str) -> Result<Uuid, rusqlite::Error> {
    Uuid::parse_str(uuid_str).map_err(|e| {
        rusqlite::Error::FromSqlConversionFailure(
            0, // column index
            rusqlite::types::Type::Text,
            Box::new(e),
        )
    })
}

pub fn get_by_id(conn: &Connection, id: Uuid) -> Result<Ticket, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT * FROM tickets WHERE uuid = ?1;")?;
    stmt.query_row([&id.to_string()], |row| {
        Ok(Ticket {
            uuid: {
                let uuid_str: String = row.get("uuid")?;
                parse_uuid(&uuid_str)?
            },
            number: row.get("number")?,
            name: row.get("name")?,
            email: row.get("email")?,
            message: row.get("message")?,
            note: row.get("note")?,
            status: row.get("status")?,
            created_at: row.get("created_at")?,
            updated_at: row.get("updated_at")?,
            closed_at: row.get("closed_at")?,
        })
    })
}

pub fn get_all(conn: &Connection, page: u32, limit: u32) -> Result<Vec<Ticket>, rusqlite::Error> {
    let mut stmt =
        conn.prepare("SELECT * FROM tickets ORDER BY created_at DESC LIMIT ?1 OFFSET ?2;")?;
    stmt.query_map([limit, (page - 1) * limit], |row| {
        Ok(Ticket {
            uuid: {
                let uuid_str: String = row.get("uuid")?;
                parse_uuid(&uuid_str)?
            },
            number: row.get("number")?,
            name: row.get("name")?,
            email: row.get("email")?,
            message: row.get("message")?,
            note: row.get("note")?,
            status: row.get("status")?,
            created_at: row.get("created_at")?,
            updated_at: row.get("updated_at")?,
            closed_at: row.get("closed_at")?,
        })
    })
    .and_then(Iterator::collect)
}

pub fn get_count(conn: &Connection) -> Result<i64, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM tickets;")?;
    stmt.query_row([], |row| row.get(0))
}

pub fn get_count_by_status(conn: &Connection, status: &str) -> Result<i64, rusqlite::Error> {
    if !["open", "pending", "closed"].contains(&status) {
        return Err(rusqlite::Error::InvalidQuery);
    }

    let mut stmt = conn.prepare("SELECT COUNT(*) FROM tickets WHERE status = ?1;")?;
    stmt.query_row([status], |row| row.get(0))
}

pub fn get_last(conn: &Connection) -> Result<Ticket, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT * FROM tickets ORDER BY created_at DESC LIMIT 1")?;
    stmt.query_row([], |row| {
        Ok(Ticket {
            uuid: {
                let uuid_str: String = row.get("uuid")?;
                parse_uuid(&uuid_str)?
            },
            number: row.get("number")?,
            name: row.get("name")?,
            email: row.get("email")?,
            message: row.get("message")?,
            note: row.get("note")?,
            status: row.get("status")?,
            created_at: row.get("created_at")?,
            updated_at: row.get("updated_at")?,
            closed_at: row.get("closed_at")?,
        })
    })
}

pub fn get_max_number(conn: &Connection) -> Result<Option<i64>, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT MAX(number) as max FROM tickets;")?;
    stmt.query_row([], |row| row.get(0))
}

pub fn create(conn: &Connection, ticket: &Ticket) -> Result<(), rusqlite::Error> {
    let mut stmt = conn.prepare(
        "INSERT INTO tickets (uuid, number, name, email, message, note, status, created_at, updated_at, closed_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10);"
    )?;

    stmt.execute(params![
        ticket.uuid.to_string(),
        ticket.number,
        ticket.name,
        ticket.email,
        ticket.message,
        ticket.note,
        ticket.status,
        ticket.created_at,
        ticket.updated_at,
        ticket.closed_at,
    ])?;

    Ok(())
}

pub fn update(conn: &Connection, id: &Uuid, tickets: &Ticket) -> Result<(), rusqlite::Error> {
    let mut stmt = conn.prepare(
        "UPDATE tickets SET name = ?1, email = ?2, message = ?3, note = ?4, status = ?5, updated_at = ?6 WHERE uuid = ?7;"
    )?;

    stmt.execute(params![
        tickets.name,
        tickets.email,
        tickets.message,
        tickets.note,
        tickets.status,
        tickets.updated_at,
        id.to_string(),
    ])?;

    Ok(())
}

pub fn delete(conn: &Connection, id: &Uuid) -> Result<(), rusqlite::Error> {
    let mut stmt = conn.prepare("DELETE FROM tickets WHERE uuid = ?1;")?;
    stmt.execute([id.to_string()])?;
    Ok(())
}
