use env_logger::Env;
use dotenv::dotenv;
use actix_cors::Cors;
use actix_web::{middleware::{Logger, NormalizePath}, http, web, App, HttpServer, HttpResponse, Responder};
use r2d2_sqlite::SqliteConnectionManager;

mod middlewares;
mod tickets;
mod utils;

pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

async fn get_status() -> impl Responder {
    HttpResponse::Ok()
        .json(serde_json::json!({
            "status": "ok",
            "version": env!("CARGO_PKG_VERSION"),
        }))
}

fn init_db(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tickets (
            uuid TEXT PRIMARY KEY,
            number INTEGER NOT NULL,
            name TEXT NOT NULL,
            email TEXT NOT NULL,
            message TEXT NOT NULL,
            note TEXT,
            status TEXT NOT NULL CHECK (status IN ('open', 'closed', 'pending')),
            created_at TEXT NOT NULL,
            updated_at TEXT,
            closed_at TEXT
        );",
        [],
    )?;
    Ok(())
}
    
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let manager = SqliteConnectionManager::file("tickets.sqlite3");
    let pool = Pool::new(manager).unwrap();
    match init_db(&pool.get().expect("Failed to get DB connection")) {
        Ok(_) => log::info!("Database initialized successfully"),
        Err(e) => log::error!("Failed to initialize database: {}", e),
    }

    let address = "0.0.0.0";

    log::info!("starting HTTP server at http://{}:8080", address);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("https://ticket.matheo-galuba.com")
            // .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE", "OPTIONS"])
            .allowed_headers(vec![http::header::CONTENT_TYPE, http::header::AUTHORIZATION])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors
            )
            .wrap(NormalizePath::trim())
            .wrap(Logger::new("%a %t \"%r\" %s %b \"%{referer}i\" \"%{user-agent}i\" %t"))
            .route("/status", web::get().to(get_status))
            .configure(tickets::routes::configure)
    })
    .bind((address, 8080))?
    .run()
    .await
}
