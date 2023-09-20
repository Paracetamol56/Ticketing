use axum::{extract, response};
use chrono::Utc;
use mongodb::bson::{doc, oid::ObjectId};
use regex::Regex;
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{ticket::Ticket, AppState, sendgrid};



pub async fn home() -> &'static str {
    "Welcome to the ticketing system API made by Mathéo GALUBA with Rust, Axum, MongoDB, and Shuttle!"
}

// Route `/status` returning a JSON body with a status message and the uptime
pub async fn status() -> response::Json<Value> {
    let response = json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION"),
    });

    response::Json(response)
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
    extract::Json(body): extract::Json<PostTicketBody>
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
    if message.len() < 10 && message.len() > 1000 {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    // Create the ticket in the database
    let collection = app_state.database.collection::<Ticket>("tickets");

    // Fetch the ticket with the highest number
    let max_number_ticket = collection.find_one(
        doc! {},
        mongodb::options::FindOneOptions::builder().sort(doc! { "number": -1 }).build()
    ).await;
    let mut ticket_number: u32 = 1;
    match max_number_ticket {
        Ok(ticket) => {
            if let Some(t) = ticket {
                ticket_number = t.number + 1;
            }
        },
        Err(e) => {
            eprintln!("Unable to fetch ticket");
            eprintln!("{:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    let ticket = Ticket::new(ticket_number, name.to_owned(), body.email.clone(), message.to_owned());
    let insert_result = collection.insert_one(&ticket, None).await;
    if let Err(e) = insert_result {
        eprintln!("Unable to insert ticket");
        eprintln!("{:?}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    
    // Send the email
    let recipient: sendgrid::User = sendgrid::User {
        name: name.to_owned(),
        email: body.email,
    };
    
    let sending = sendgrid::send_ticket(&app_state.secret_store, &recipient).await;

    match sending {
        Ok(_) => {
            let response = json!({
                "email": {
                    "status": "sent",
                    "recipient": &recipient,
                },
                "ticket": {
                    "id": insert_result.unwrap().inserted_id.as_object_id().unwrap().to_hex(),
                    "number": ticket_number,
                    "name": &ticket.name,
                    "email": &ticket.email,
                    "message": &ticket.message,
                    "status": &ticket.status,
                    "created_at": ticket.created_at,
                }
            });
            Ok(response::Json(response))
        },
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
    let collection = app_state.database.collection::<Ticket>("tickets");

    let object_id = ObjectId::parse_str(&id.as_str());
    if let Err(_) = object_id {
        eprintln!("Invalid id (provided id = {})", &id);
        return Err(StatusCode::BAD_REQUEST);
    }

    let ticket = collection.find_one(
        doc! { "_id": object_id.unwrap() },
        None
    ).await;

    match ticket {
        Ok(ticket) => {
            if let Some(t) = ticket {
                let response = json!({
                    "id": id,
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
        },
        Err(e) => {
            eprintln!("Unable to fetch ticket");
            eprintln!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PutTicketBody {
    status: String,
    note: Option<String>,
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

    let collection = app_state.database.collection::<Ticket>("tickets");

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
            let ticket = collection.find_one(
                doc! { "_id": object_id.unwrap() },
                None
            ).await;
            match ticket {
                Ok(Some(t)) => {
                    let response = json!({
                        "id": id,
                        "number": t.number,
                        "name": t.name,
                        "email": t.email,
                        "message": t.message,
                        "status": t.status,
                        "note": t.note,
                        "created_at": t.created_at,
                        "updated_at": t.updated_at,
                        "closed_at": t.closed_at,
                    });
                    Ok(response::Json(response))
                },
                _ => {
                    eprintln!("Unable to fetch ticket");
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            }
        },
        Err(e) => {
            eprintln!("Unable to update ticket");
            eprintln!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
