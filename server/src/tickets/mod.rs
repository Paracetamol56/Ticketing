use actix_web::HttpResponse;

pub mod handlers;
pub mod models;
mod repository;
pub mod routes;
mod service;

#[derive(Debug)]
pub enum ServiceError {
    Database(rusqlite::Error),
    NotFound,
    #[allow(dead_code)]
    Internal(String),
}

impl From<rusqlite::Error> for ServiceError {
    fn from(err: rusqlite::Error) -> Self {
        match err {
            rusqlite::Error::QueryReturnedNoRows => ServiceError::NotFound,
            e => ServiceError::Database(e),
        }
    }
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceError::NotFound => write!(f, "Resource not found"),
            ServiceError::Database(e) => write!(f, "Database error: {}", e),
            ServiceError::Internal(msg) => write!(f, "Internal server error: {}", msg),
        }
    }
}

impl actix_web::ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::NotFound => HttpResponse::NotFound().finish(),
            ServiceError::Database(e) => {
                HttpResponse::InternalServerError().body(format!("Database error: {}", e))
            }
            ServiceError::Internal(msg) => HttpResponse::InternalServerError().body(msg.clone()),
        }
    }
}
