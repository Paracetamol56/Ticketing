use auth::admin_auth;
use axum::{handler::Handler, middleware, routing::get, Router};
use dotenv::dotenv;
use log;
use mongodb::Database;
use tower_http::{
    cors::{Any, CorsLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

mod auth;
mod handlers;
mod sendgrid;
mod ticket;

// Database initialization
async fn init_db() -> Result<Database, mongodb::error::Error> {
    let client = mongodb::Client::with_uri_str("mongodb://localhost:27017/").await;
    match client {
        Ok(client) => {
            log::info!("Connected to database");
            Ok(client.database("tickets"))
        }
        Err(e) => {
            log::error!("Failed to connect to database: {}", e);
            Err(e)
        }
    }
}

#[derive(Clone)]
struct AppState {
    db: Database,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let state = AppState {
        db: init_db().await.expect("Failed to connect to database"),
    };

    let cors = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods(Any)
        .allow_origin(Any)
        .expose_headers(Any);

    let admin_auth_middleware = middleware::from_fn(move |req, next| {
        let admin_token = dotenv::var("ADMIN_TOKEN").expect("ADMIN_TOKEN must be set");
        admin_auth(req, next, admin_token)
    });

    let router = Router::new()
        .route("/", get(handlers::home))
        .route("/status", get(handlers::status))
        .route(
            "/statistics",
            get(handlers::statistics.layer(admin_auth_middleware.clone())),
        )
        .route(
            "/ticket",
            get(handlers::get_ticket_page.layer(admin_auth_middleware.clone()))
                .post(handlers::post_ticket),
        )
        .route(
            "/ticket/{id}",
            get(handlers::get_ticket)
                .patch(handlers::put_ticket.layer(admin_auth_middleware.clone())),
        )
        .layer(cors)
        .layer(TimeoutLayer::new(std::time::Duration::from_secs(30)))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    log::info!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}
