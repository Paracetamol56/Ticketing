use serde::{Serialize, Deserialize};
use mongodb::bson::doc;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticket {
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

impl std::fmt::Display for Ticket {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Ticket: \n\tnumber: {},\n\temail: {},\n\tmessage: {},\n\tstatus: {},\n\tcreated_at: {},\n\tupdated_at: {},\n\tclosed_at: {}\n",
        self.number, self.email, self.message, self.status, self.created_at, self.updated_at.unwrap_or_default(), self.closed_at.unwrap_or_default())
    }
}
