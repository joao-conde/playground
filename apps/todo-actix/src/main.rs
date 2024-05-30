use actix_web::{middleware::Logger, App, HttpServer};
use log::info;
use sqlx::SqlitePool;
use std::env;
use todo_actix::app::configure_app;

const HOST: &str = "127.0.0.1";
const PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let db_url = env::var("DATABASE_URL")?;
    let db_pool = SqlitePool::connect(&db_url).await?;
    sqlx::migrate!("./migrations").run(&db_pool).await?;

    let app_builder = move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .configure(|config| configure_app(config, db_pool.clone()))
    };

    let server = HttpServer::new(app_builder).bind((HOST, PORT))?;

    info!("Listening on http://{HOST}:{PORT}");
    server.run().await?;

    Ok(())
}
