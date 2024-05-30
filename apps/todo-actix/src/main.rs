use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use log::info;
use sqlx::SqlitePool;
use std::env;
use todo_actix::{
    routes::{create_todo, delete_todo, get_todo, list_todos, update_todo},
    AppData,
};

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
        let app_data = Data::new(AppData {
            db_pool: db_pool.clone(),
        });
        App::new()
            .wrap(logger)
            .app_data(app_data)
            .service(list_todos)
            .service(get_todo)
            .service(create_todo)
            .service(update_todo)
            .service(delete_todo)
    };

    let server = HttpServer::new(app_builder).bind((HOST, PORT))?;

    info!("Listening on http://{HOST}:{PORT}");
    server.run().await?;

    Ok(())
}
