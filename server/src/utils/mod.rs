pub mod brevo;
pub mod db;
pub mod pagination;

pub fn get_listen_address() -> &'static str {
    if std::env::var("DOCKER_ENV").is_ok()
        || std::env::var("DOCKER").is_ok()
        || std::fs::metadata("/.dockerenv").is_ok()
    {
        return "0.0.0.0"; // Listen on all interfaces in Docker
    }
    "127.0.0.1" // Listen only on localhost otherwise
}
