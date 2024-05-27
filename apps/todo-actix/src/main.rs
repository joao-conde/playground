mod db;
mod error;
mod routes;

use actix_web::{web, App, HttpServer};
use r2d2_sqlite::SqliteConnectionManager;
use routes::{create_todo, delete_todo, get_todo, list_todos, update_todo};

const HOST: &str = "127.0.0.1";
const PORT: u16 = 8080;
const DB_URL: &str = "db/todos.db";

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_pool = r2d2::Pool::builder().build(SqliteConnectionManager::file(DB_URL))?;

    let app_builder = move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .service(list_todos)
            .service(create_todo)
            .service(get_todo)
            .service(update_todo)
            .service(delete_todo)
    };

    let server = HttpServer::new(app_builder).bind((HOST, PORT))?;

    println!("Server listening on http://{HOST}:{PORT}");
    server.run().await?;

    Ok(())
}
