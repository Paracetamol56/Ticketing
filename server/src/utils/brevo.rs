use reqwest::{header, Client};
use serde::Serialize;
use serde_json::json;

use crate::tickets::models::Ticket;

#[derive(Debug, Serialize)]
pub struct User {
    pub name: String,
    pub email: String,
}

pub async fn send_email(recipient: &User, subject: &str, body: &str) -> Result<(), reqwest::Error> {
    let api_key: String = std::env::var("BREVO_API_KEY").expect("BREVO_API_KEY must be set");
    let sender_email: String = std::env::var("SENDER_EMAIL").expect("SENDER_EMAIL must be set");
    let sender_name: String = std::env::var("SENDER_NAME").expect("SENDER_NAME must be set");

    log::debug!("Sending email to {}", recipient.email);
    log::debug!("From: {} <{}>", sender_name, sender_email);
    log::debug!("Subject: {}", subject);

    // Payload according to Brevo's API
    let request_body = json!({
        "sender": {
            "name": sender_name,
            "email": sender_email
        },
        "to": [
            {
                "email": recipient.email,
                "name": recipient.name
            }
        ],
        "subject": subject,
        "htmlContent": body
    });

    let client = Client::new();
    let response = client
        .post("https://api.brevo.com/v3/smtp/email")
        .header("api-key", api_key)
        .header(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        )
        .json(&request_body)
        .send();

    match response.await {
        Ok(resp) => {
            if !resp.status().is_success() {
                let status = resp.status();
                let text = resp
                    .text()
                    .await
                    .unwrap_or_else(|_| "<failed to read body>".into());
                log::error!("Error response: {} - {}", status, text);
            } else {
                log::info!("Email sent successfully!");
            }
            Ok(())
        }
        Err(err) => {
            log::error!("[brevo] Request failed: {}", err);
            Err(err)
        }
    }
}

pub async fn send_ticket(ticket: &Ticket) -> Result<(), reqwest::Error> {
    let mut body: String = include_str!("../../ticket_template.html").to_owned();
    body = body
        .replace("{{name}}", &ticket.name)
        .replace("{{email}}", &ticket.email)
        .replace("{{number}}", &ticket.number.to_string())
        .replace(
            "{{link}}",
            format!("https://ticket.matheo-galuba.com/?ticket={}", ticket.uuid).as_str(),
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
    let mut body: String = include_str!("../../notification_template.html").to_owned();
    body = body
        .replace("{{name}}", &ticket.name)
        .replace("{{email}}", &ticket.email)
        .replace("{{number}}", &ticket.number.to_string())
        .replace("{{status}}", &ticket.status)
        .replace(
            "{{link}}",
            format!("https://ticket.matheo-galuba.com/?ticket={}", ticket.uuid).as_str(),
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
