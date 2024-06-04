use actix_web::{middleware::Logger, App, HttpServer};
use log::info;
use sqlx::SqlitePool;
use todo_actix::{configure_app, Config};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env()?;
    env_logger::builder().filter_level(config.log_level).init();

    let db_pool = SqlitePool::connect(&config.db_url).await?;
    sqlx::migrate!("./migrations").run(&db_pool).await?;

    let app_builder = move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .configure(|c| configure_app(c, db_pool.clone()))
    };
    let server = HttpServer::new(app_builder).bind((config.host.clone(), config.port))?;

    info!("Listening on http://{}:{}", config.host, config.port);
    server.run().await?;

    Ok(())
}
