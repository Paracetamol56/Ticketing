use chrono::{DateTime, Utc};
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticket {
    #[serde(rename = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub number: u32,
    pub name: String,
    pub email: String,
    pub message: String,
    pub note: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub closed_at: Option<DateTime<Utc>>,
}

impl Ticket {
    pub fn new(number: u32, name: String, email: String, message: String) -> Ticket {
        Ticket {
            id: None,
            number: number,
            name: name,
            email,
            message,
            note: None,
            status: String::from("open"),
            created_at: Utc::now(),
            updated_at: None,
            closed_at: None,
        }
    }
}
