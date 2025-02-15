use reqwest::{header, Client, StatusCode};
use serde::Serialize;
use serde_json::json;

use crate::ticket::Ticket;

#[derive(Debug, Serialize)]
pub struct User {
    pub name: String,
    pub email: String,
}

async fn send_email(recipient: &User, subject: &str, body: &str) -> Result<(), reqwest::Error> {
    let api_key: String = dotenv::var("SENDGRID_API_KEY").expect("SENDGRID_API_KEY must be set");
    let sender_email: String = dotenv::var("SENDER_EMAIL").expect("SENDER_EMAIL must be set");
    let sender_name: String = dotenv::var("SENDER_NAME").expect("SENDER_NAME must be set");

    let request_body = json!({
        "personalizations": [
            {
                "to": [
                    {
                        "email": recipient.email,
                        "name": recipient.name
                    }
                ],
                "subject": subject
            }
        ],
        "from": {
            "email": sender_email,
            "name": sender_name,
        },
        "content": [
            {
                "type": "text/html",
                "value": body
            }
        ]
    });

    let client = Client::new()
        .post("https://api.sendgrid.com/v3/mail/send")
        .json(&request_body)
        .bearer_auth(api_key)
        .header(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

    let response = client.send().await?;
    let message_id = response
        .headers()
        .get("x-message-id")
        .unwrap()
        .to_str()
        .unwrap();

    match response.status() {
        StatusCode::OK | StatusCode::CREATED | StatusCode::ACCEPTED => println!(
            "Email sent to {} (SendGrid id = {})",
            recipient.email, message_id
        ),
        _ => eprintln!(
            "Unable to send your email. Status code was: {}.",
            response.status()
        ),
    }

    Ok(())
}

pub async fn send_ticket(ticket: &Ticket) -> Result<(), reqwest::Error> {
    let mut body: String = include_str!("../ticket_template.html").to_owned();
    body = body
        .replace("{{name}}", &ticket.name)
        .replace("{{email}}", &ticket.email)
        .replace("{{number}}", &ticket.number.to_string())
        .replace(
            "{{link}}",
            format!(
                "https://ticket.matheo-galuba.com/?ticket={}",
                ticket.id.unwrap().to_hex()
            )
            .as_str(),
        );
    send_email(
        &User {
            name: ticket.name.clone(),
            email: ticket.email.clone(),
        },
        "Your ticket has been issued",
        body.as_ref(),
    )
    .await
}

pub async fn send_notification(ticket: &Ticket) -> Result<(), reqwest::Error> {
    let mut body: String = include_str!("../notification_template.html").to_owned();
    body = body
        .replace("{{name}}", &ticket.name)
        .replace("{{email}}", &ticket.email)
        .replace("{{number}}", &ticket.number.to_string())
        .replace("{{status}}", &ticket.status)
        .replace(
            "{{link}}",
            format!(
                "https://ticket.matheo-galuba.com/?ticket={}",
                ticket.id.unwrap().to_hex()
            )
            .as_str(),
        );
    send_email(
        &User {
            name: ticket.name.clone(),
            email: ticket.email.clone(),
        },
        "Your ticket has been updated",
        body.as_ref(),
    )
    .await
}
