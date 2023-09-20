
mod handlers;
mod ticket;
mod sendgrid;

use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use mongodb::Database;
use reqwest::Method;
use tower_http::cors::{CorsLayer, Any};

#[derive(Clone)]
pub struct AppState {
    pub secret_store: shuttle_secrets::SecretStore,
    pub database: Arc<mongodb::Database>,
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_secrets::Secrets] secret_store: shuttle_secrets::SecretStore,
    #[shuttle_shared_db::MongoDb(local_uri = "{secrets.MONGO_CONNECTION_STRING}")] db: Database,
) -> shuttle_axum::ShuttleAxum {

    let state = AppState {
        secret_store: secret_store.clone(),
        database: Arc::new(db),
    };

    let cors = CorsLayer::new()
        .allow_methods([Method::HEAD, Method::OPTIONS, Method::GET, Method::POST, Method::PUT])
        .allow_origin(Any)
        .allow_headers(Any);

    let router = Router::new()
        .route("/", get(handlers::home))
        .route("/status", get(handlers::status))
        .route("/ticket", post(handlers::post_ticket))
        .route("/ticket/:id", get(handlers::get_ticket).put(handlers::put_ticket))
        .layer(CorsLayer::permissive())
        .with_state(state);

    Ok(router.into())
}
