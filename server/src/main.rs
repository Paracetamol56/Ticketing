use actix_cors::Cors;
use actix_files::Files;
use actix_web::{
    http,
    middleware::{Logger, NormalizePath},
    web, App, HttpServer,
};
use dotenv::dotenv;
use env_logger::Env;
use r2d2_sqlite::SqliteConnectionManager;

use crate::utils::{
    db::{init_db, Pool},
    get_listen_address,
};

mod middlewares;
mod status;
mod tickets;
mod utils;

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

    // Get the static directory path
    let mut static_dir = std::env::current_exe()?;
    static_dir.pop();
    static_dir.push("static");

    let address = get_listen_address();

    log::info!("Starting HTTP server at http://{}:8080", address);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("https://ticket.matheo-galuba.com")
            // .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                http::header::CONTENT_TYPE,
                http::header::AUTHORIZATION,
            ])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .wrap(NormalizePath::trim())
            .wrap(Logger::new(
                "%a %t \"%r\" %s %b \"%{referer}i\" \"%{user-agent}i\" %t",
            ))
            .configure(status::routes::configure)
            .configure(tickets::routes::configure)
            .service(
                Files::new("/", &static_dir)
                    .use_last_modified(true)
                    .index_file("index.html"),
            )
    })
    .bind((address, 8080))?
    .run()
    .await
}
