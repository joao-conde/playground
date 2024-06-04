use actix_web::{middleware::Logger, App, HttpServer};
use log::info;
use sqlx::SqlitePool;
use std::env;
use todo_actix::app::configure_app;

const HOST: &str = "127.0.0.1";
const PORT: u16 = 8080;
const DATABASE_URL: &str = "sqlite://todos.db";

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let host = env::var("HOST").unwrap_or(HOST.to_string());
    let port = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(PORT);
    let db_url = env::var("DATABASE_URL").unwrap_or(DATABASE_URL.to_string());

    let db_pool = SqlitePool::connect(&db_url).await?;
    sqlx::migrate!("./migrations").run(&db_pool).await?;

    let app_builder = move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .configure(|config| configure_app(config, db_pool.clone()))
    };
    let server = HttpServer::new(app_builder).bind((host.clone(), port))?;

    info!("Listening on http://{host}:{port}");
    server.run().await?;

    Ok(())
}
