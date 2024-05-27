mod db;

use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer};
use db::ApiError;
use r2d2_sqlite::SqliteConnectionManager;

const HOST: &str = "127.0.0.1";
const PORT: u16 = 8080;

#[get("/todos")]
async fn list_todos(pool: web::Data<db::Pool>) -> Result<HttpResponse, ApiError> {
    let todos = db::list_todos(&pool).await?;
    Ok(HttpResponse::Ok().json(todos))
}

#[post("/todos")]
async fn create_todo(
    pool: web::Data<db::Pool>,
    todo: web::Json<db::CreateTodo>,
) -> Result<HttpResponse, ApiError> {
    let created = db::create_todo(&pool, todo.into_inner()).await?;
    Ok(HttpResponse::Ok().json(created))
}

#[get("/todos/{id}")]
async fn get_todo(
    pool: web::Data<db::Pool>,
    id: web::Path<usize>,
) -> Result<HttpResponse, ApiError> {
    let todo = db::get_todo(&pool, *id).await?;
    Ok(HttpResponse::Ok().json(todo))
}

#[put("/todos")]
async fn update_todo(todo: web::Json<db::Todo>) -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::Ok().json(todo))
}

#[delete("/todos/{id}")]
async fn delete_todo(
    pool: web::Data<db::Pool>,
    id: web::Path<usize>,
) -> Result<HttpResponse, ApiError> {
    let deleted = db::delete_todo(&pool, *id).await?;
    Ok(HttpResponse::Ok().json(deleted))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_pool = r2d2::Pool::builder()
        .build(SqliteConnectionManager::file("db/todos.db"))
        .unwrap();

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
    server.run().await
}
