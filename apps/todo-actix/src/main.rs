mod db;

use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use r2d2_sqlite::SqliteConnectionManager;

const HOST: &str = "127.0.0.1";
const PORT: u16 = 8080;

#[get("/todos")]
async fn list_todos(pool: web::Data<db::Pool>) -> impl Responder {
    let todos = db::list_todos(&pool).await.unwrap();
    HttpResponse::Ok().json(todos)
}

#[post("/todos")]
async fn create_todo(pool: web::Data<db::Pool>, todo: web::Json<db::CreateTodo>) -> impl Responder {
    let created = db::create_todo(&pool, todo.into_inner()).await.unwrap();
    HttpResponse::Ok().json(created)
}

#[get("/todos/{id}")]
async fn get_todo(pool: web::Data<db::Pool>, id: web::Path<usize>) -> impl Responder {
    let todo = db::get_todo(&pool, *id).await.unwrap();
    HttpResponse::Ok().json(todo)
}

#[put("/todos")]
async fn update_todo(todo: web::Json<db::Todo>) -> impl Responder {
    HttpResponse::Ok().json(todo)
}

#[delete("/todos/{id}")]
async fn delete_todo(pool: web::Data<db::Pool>, id: web::Path<usize>) -> impl Responder {
    let deleted = db::delete_todo(&pool, *id).await.unwrap();
    HttpResponse::Ok().json(deleted)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_pool = r2d2::Pool::builder()
        .build(SqliteConnectionManager::file("db/todos.db"))
        .unwrap();

    println!("Server listening on {HOST}:{PORT}");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .service(list_todos)
            .service(create_todo)
            .service(get_todo)
            .service(update_todo)
            .service(delete_todo)
    })
    .bind((HOST, PORT))?
    .run()
    .await
}
