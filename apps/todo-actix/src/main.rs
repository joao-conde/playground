use actix_web::{middleware::Logger, web, App, HttpServer};
use log::info;
use r2d2_sqlite::SqliteConnectionManager;
use todo_actix::routes::{create_todo, delete_todo, get_todo, list_todos, update_todo};

const HOST: &str = "127.0.0.1";
const PORT: u16 = 8080;
const DB_URL: &str = "db/todos.db";

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let db_pool = r2d2::Pool::builder().build(SqliteConnectionManager::file(DB_URL))?;

    let app_builder = move || {
        let db_pool = web::Data::new(db_pool.clone());
        let logger = Logger::default();
        App::new()
            .app_data(db_pool)
            .wrap(logger)
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
