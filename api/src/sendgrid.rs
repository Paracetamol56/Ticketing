use reqwest::{Client, header, StatusCode};
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Serialize)]
pub struct User {
    pub name: String,
    pub email: String,
}

async fn send_email(secret_store: &shuttle_secrets::SecretStore, recipient: &User, subject: &str, body: &str) -> Result<(), reqwest::Error> {
    let api_key: String = secret_store.get("SENDGRID_API_KEY").unwrap();

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
            "email": secret_store.get("SENDER_EMAIL").unwrap(),
            "name": secret_store.get("SENDER_NAME").unwrap()
        },
        "content": [
            {
                "type": "text/plain",
                "value": body
            }
        ]
    });

    let client = Client::new()
        .post("https://api.sendgrid.com/v3/mail/send")
        .json(&request_body)
        .bearer_auth(api_key)
        .header(header::CONTENT_TYPE, header::HeaderValue::from_static("application/json"));

    let response = client.send().await?;
    let message_id = response.headers().get("x-message-id").unwrap().to_str().unwrap();

    match response.status() {
        StatusCode::OK | StatusCode::CREATED | StatusCode::ACCEPTED => println!(
            "Email sent to {} (SendGrid id = {})",
            recipient.email,
            message_id
        ),
        _ => eprintln!(
            "Unable to send your email. Status code was: {}.",
            response.status()
        ),
    }

    Ok(())

}

pub async fn send_ticket(secret_store: &shuttle_secrets::SecretStore, recipient: &User) -> Result<(), reqwest::Error> {
    send_email(
        secret_store,
        recipient,
        "Your ticket has been issued",
        "Thank you for using our service. Your ticket has been issued."
    ).await
}
