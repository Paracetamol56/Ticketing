use axum::{extract, response};
use chrono::{DateTime, Utc};
use futures::StreamExt;
use mongodb::bson::{doc, oid::ObjectId};
use regex::Regex;
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{sendgrid, ticket::Ticket, AppState};

pub async fn not_found() -> StatusCode {
    StatusCode::NOT_FOUND
}

pub async fn home() -> response::Json<Value> {
    let response = json!({
        "greetings": "Welcome to the Ticketing API!",
        "routes": {
            "status": "GET /status",
            "create_ticket": "POST /ticket",
            "get_ticket": "GET /ticket/{id}"
        },
        "documentation": "https://api.ticket.matheo-galuba.com/docs"
    });

    response::Json(response)
}

// Route `/status` returning a JSON body with a status message and the uptime
pub async fn status() -> response::Json<Value> {
    let response = json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION"),
    });

    response::Json(response)
}

pub async fn statistics(
    extract::State(app_state): extract::State<AppState>,
) -> Result<response::Json<Value>, StatusCode> {
    let collection = app_state.db.collection::<Ticket>("tickets");

    let open_tickets = collection
        .count_documents(doc! { "status": "open" }, None)
        .await
        .unwrap_or(0);
    let pending_tickets = collection
        .count_documents(doc! { "status": "pending" }, None)
        .await
        .unwrap_or(0);
    let closed_tickets = collection
        .count_documents(doc! { "status": "closed" }, None)
        .await
        .unwrap_or(0);
    let last_ticket = collection
        .find_one(
            doc! {},
            mongodb::options::FindOneOptions::builder()
                .sort(doc! { "created_at": -1 })
                .build(),
        )
        .await
        .unwrap_or(None);
    let mut last_ticket_date: Option<DateTime<Utc>> = None;
    if let Some(t) = last_ticket {
        last_ticket_date = Some(t.created_at)
    }

    Ok(response::Json(json!({
        "open_tickets": open_tickets,
        "pending_tickets": pending_tickets,
        "closed_tickets": closed_tickets,
        "last_ticket": last_ticket_date,
    })))
}

#[derive(Debug, Deserialize)]
pub struct PostTicketBody {
    name: String,
    email: String,
    message: String,
}

// Route POST `/ticket` taking a json body and returning a json body
pub async fn post_ticket(
    extract::State(app_state): extract::State<AppState>,
    extract::Json(body): extract::Json<PostTicketBody>,
) -> Result<response::Json<Value>, StatusCode> {
    // body.name must be at least 3 characters long and 100 characters at most
    let name = body.name.trim();
    if name.len() < 3 && name.len() > 100 {
        return Err(StatusCode::BAD_REQUEST);
    }
    // body.to must be a valid email address
    let email_pattern = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    if !email_pattern.is_match(&body.email) {
        return Err(StatusCode::BAD_REQUEST);
    }
    // body.message must be at least 10 characters long and 1000 characters at most
    let message = body.message.trim();
    if message.len() < 3 && message.len() > 1000 {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Create the ticket in the database
    let collection = app_state.db.collection::<Ticket>("tickets");

    // Fetch the ticket with the highest number
    let max_number_ticket = collection
        .find_one(
            doc! {},
            mongodb::options::FindOneOptions::builder()
                .sort(doc! { "number": -1 })
                .build(),
        )
        .await;
    let mut ticket_number: u32 = 1;
    match max_number_ticket {
        Ok(ticket) => {
            if let Some(t) = ticket {
                ticket_number = t.number + 1;
            }
        }
        Err(e) => {
            eprintln!("Unable to fetch ticket");
            eprintln!("{:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    let mut ticket = Ticket::new(
        ticket_number,
        name.to_owned(),
        body.email.clone(),
        message.to_owned(),
    );
    let insert_result = collection.insert_one(&ticket, None).await;
    if let Err(e) = insert_result {
        eprintln!("Unable to insert ticket");
        eprintln!("{:?}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    // Add the id to the ticket
    ticket.id = Some(
        insert_result
            .unwrap()
            .inserted_id
            .as_object_id()
            .unwrap()
            .clone(),
    );

    // Send the email
    let recipient: sendgrid::User = sendgrid::User {
        name: name.to_owned(),
        email: body.email,
    };

    let sending = sendgrid::send_ticket(&ticket).await;

    match sending {
        Ok(_) => {
            let response = json!({
                "email": {
                    "status": "sent",
                    "recipient": &recipient,
                },
                "ticket": {
                    "id": ticket.id.unwrap().to_hex(),
                    "number": &ticket.number,
                    "name": &ticket.name,
                    "email": &ticket.email,
                    "message": &ticket.message,
                    "status": &ticket.status,
                    "created_at": ticket.created_at,
                }
            });
            Ok(response::Json(response))
        }
        Err(e) => {
            eprintln!("Unable to send email");
            eprintln!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Route GET `/ticket/:id` returning a json body
pub async fn get_ticket(
    extract::State(app_state): extract::State<AppState>,
    extract::Path(id): extract::Path<String>,
) -> Result<response::Json<Value>, StatusCode> {
    let collection = app_state.db.collection::<Ticket>("tickets");

    let object_id = ObjectId::parse_str(&id.as_str());
    if let Err(_) = object_id {
        eprintln!("Invalid id (provided id = {})", &id);
        return Err(StatusCode::BAD_REQUEST);
    }

    let ticket = collection
        .find_one(doc! { "_id": object_id.unwrap() }, None)
        .await;

    match ticket {
        Ok(ticket) => {
            if let Some(t) = ticket {
                let response = json!({
                    "id": t.id.unwrap().to_hex(),
                    "number": t.number,
                    "name": t.name,
                    "email": t.email,
                    "message": t.message,
                    "status": t.status,
                    "created_at": t.created_at,
                    "updated_at": t.updated_at,
                    "closed_at": t.closed_at
                });
                Ok(response::Json(response))
            } else {
                Err(StatusCode::NOT_FOUND)
            }
        }
        Err(e) => {
            eprintln!("Unable to fetch ticket");
            eprintln!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct GetTicketPageQuery {
    page: u32,
    limit: u32,
    status: Option<String>,
}

// Route GET `/tickets` returning a json body
pub async fn get_ticket_page(
    extract::State(app_state): extract::State<AppState>,
    extract::Query(query): extract::Query<GetTicketPageQuery>,
) -> Result<response::Json<Value>, StatusCode> {
    // query.page must be at least 1
    if query.page < 1 {
        return Err(StatusCode::BAD_REQUEST);
    }
    // query.limit must be at least 1 and at most 100
    if query.limit < 1 || query.limit > 100 {
        return Err(StatusCode::BAD_REQUEST);
    }
    // query.status must be either "open", "pending", or "closed"
    let status_pattern = Regex::new(r"^(open|pending|closed)$").unwrap();
    if let Some(status) = &query.status {
        if !status_pattern.is_match(&status) {
            return Err(StatusCode::BAD_REQUEST);
        }
    }

    let collection = app_state.db.collection::<Ticket>("tickets");

    let mut filter = doc! {};
    if let Some(status) = &query.status {
        filter = doc! { "status": status };
    }

    let cursor = collection
        .find(
            filter.clone(),
            mongodb::options::FindOptions::builder()
                // closed ones at the end and ascending numbers
                .sort(doc! { "number": -1 })
                .skip(Some(((query.page - 1) * query.limit) as u64))
                .limit(Some(query.limit as i64))
                .build(),
        )
        .await;

    let total_tickets = collection
        .count_documents(filter.clone(), None)
        .await
        .unwrap();

    match cursor {
        Ok(mut cursor) => {
            let mut tickets_vec: Vec<Value> = Vec::new();
            while let Some(ticket) = cursor.next().await {
                if let Ok(t) = ticket {
                    tickets_vec.push(json!({
                        "id": t.id.unwrap().to_hex(),
                        "number": t.number,
                        "name": t.name,
                        "email": t.email,
                        "message": t.message,
                        "status": t.status,
                        "note": t.note,
                        "created_at": t.created_at,
                        "updated_at": t.updated_at,
                        "closed_at": t.closed_at,
                    }));
                }
            }
            let response = json!({
                "tickets": tickets_vec,
                "count": total_tickets,
                "max_page": (total_tickets as f64 / query.limit as f64).ceil() as u32,
                "page": query.page,
                "limit": query.limit,
                "status": query.status,
            });
            Ok(response::Json(response))
        }
        Err(e) => {
            eprintln!("Unable to fetch tickets");
            eprintln!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PutTicketBody {
    status: String,
    note: Option<String>,
    notify: bool,
}

// Route PUT `/ticket/:id` taking a json body and returning a json body
pub async fn put_ticket(
    extract::State(app_state): extract::State<AppState>,
    extract::Path(id): extract::Path<String>,
    extract::Json(body): extract::Json<PutTicketBody>,
) -> Result<response::Json<Value>, StatusCode> {
    // body.status must be either "open", "pending", or "closed"
    let status_pattern = Regex::new(r"^(open|pending|closed)$").unwrap();
    if !status_pattern.is_match(&body.status) {
        return Err(StatusCode::BAD_REQUEST);
    }
    // body.note must be at least 10 characters long and 1000 characters at most
    let mut note: Option<String> = None;
    if let Some(body_note) = body.note {
        note = Some(body_note.trim().to_owned());
        if body_note.len() < 10 && body_note.len() > 1000 {
            return Err(StatusCode::BAD_REQUEST);
        }
    }

    let collection = app_state.db.collection::<Ticket>("tickets");

    let object_id = ObjectId::parse_str(&id.as_str());
    if let Err(_) = object_id {
        eprintln!("Invalid id (provided id = {})", &id);
        return Err(StatusCode::BAD_REQUEST);
    }

    let update_result = collection.update_one(
        doc! { "_id": object_id.clone().unwrap() },
        doc! { "$set": {
            "status": &body.status,
            "note": note,
            "updated_at": Utc::now().to_rfc3339(),
            "closed_at": if body.status == "closed" { Some(Utc::now().to_rfc3339()) } else { None },
        } },
        None
    ).await;
    match update_result {
        Ok(_) => {
            let ticket = collection
                .find_one(doc! { "_id": object_id.unwrap() }, None)
                .await;
            match ticket {
                Ok(Some(t)) => {
                    if body.notify {
                        let _ = sendgrid::send_notification(&t).await;
                    }
                    Ok(response::Json(json!({
                        "id": t.id.unwrap().to_hex(),
                        "number": t.number,
                        "name": t.name,
                        "email": t.email,
                        "message": t.message,
                        "status": t.status,
                        "note": t.note,
                        "created_at": t.created_at,
                        "updated_at": t.updated_at,
                        "closed_at": t.closed_at
                    })))
                }
                _ => {
                    eprintln!("Unable to fetch ticket");
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            }
        }
        Err(e) => {
            eprintln!("Unable to update ticket");
            eprintln!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
