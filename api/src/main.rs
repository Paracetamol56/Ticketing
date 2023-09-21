mod handlers;
mod sendgrid;
mod ticket;

use std::sync::Arc;

use axum::{
    body::{boxed, Body},
    handler::Handler,
    http::Request,
    middleware::{Next, self},
    response::Response,
    routing::get,
    Router,
};
use mongodb::Database;
use reqwest::StatusCode;
use tower_http::{
    cors::{Any, CorsLayer},
    timeout::TimeoutLayer,
};

async fn admin_auth<B>(
    req: Request<B>,
    next: Next<B>,
    admin_token: String,
) -> axum::response::Response {
    let request_token = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok());

    if request_token != Some(&admin_token) {
        eprintln!("Unauthorized request to admin endpoint");
        let response = Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(boxed(Body::empty()))
            .unwrap();
        return response;
    }

    next.run(req).await
}

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
        .allow_headers(Any)
        .allow_methods(Any)
        .allow_origin(Any)
        .expose_headers(Any);

    let admin_aut_middleware = middleware::from_fn(move |req, next| {
        let admin_token = secret_store.get("ADMIN_TOKEN").unwrap();
        admin_auth(req, next, admin_token)
    });
    
    let router = Router::new()
        .route("/", get(handlers::home))
        .route("/status", get(handlers::status))
        .route("/statistics", get(handlers::statistics.layer(admin_aut_middleware.clone())))
        .route(
            "/ticket",
            get(handlers::get_ticket_page.layer(admin_aut_middleware.clone()))
                .post(handlers::post_ticket),
        )
        .route(
            "/ticket/:id",
            get(handlers::get_ticket)
                .patch(handlers::put_ticket.layer(admin_aut_middleware.clone()))
        )
        .layer(cors)
        .layer(TimeoutLayer::new(std::time::Duration::from_secs(30)))
        .with_state(state);

    Ok(router.into())
}
