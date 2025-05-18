mod database;
mod model;
mod routes;
mod util;
mod websocket;

use util::logger;

use std::sync::Arc;
use std::{env, io};

use actix_web::{App, HttpServer};
use sqlx::SqlitePool;

const DEFAULT_PORT: u16 = 2137;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let _ = dotenvy::dotenv();
    logger::init();

    log::info!(
        "Logger initialized successfully with level: {}",
        log::max_level()
    );

    if cfg!(feature = "dev") {
        log::warn!("Running with development mode enabled");
    }

    let port = env::var("PORT")
        .unwrap_or_else(|_| DEFAULT_PORT.to_string())
        .parse::<u16>();

    let port = match port {
        Ok(port) => port,
        Err(err) => {
            log::error!(
                "Invalid port number: {}; using default port {}",
                err,
                DEFAULT_PORT
            );
            DEFAULT_PORT
        }
    };

    let database = util::get_path_to("database.sqlite3");
    let database_url = format!("sqlite:{}", database.display());

    let pool = match SqlitePool::connect(&database_url).await {
        Ok(pool) => Arc::new(pool),
        Err(err) => {
            log::error!("Failed to connect to database: {}", err);
            return Ok(());
        }
    };

    let addr = format!("0.0.0.0:{port}");
    log::info!("Starting server on {addr}");

    HttpServer::new(move || App::new().wrap(util::Cors))
        .bind(&addr)?
        .run()
        .await
}
