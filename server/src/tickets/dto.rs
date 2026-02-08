use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct TicketStatsResponse {
    pub open: i64,
    pub pending: i64,
    pub closed: i64,
    pub total: i64,
    pub last_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct PostTicketRequest {
    pub name: String,
    pub email: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct PatchTicketRequest {
    pub note: Option<String>,
    pub status: Option<String>,
    pub notify: bool,
}
