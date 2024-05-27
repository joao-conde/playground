use crate::{db, error::ApiError};
use actix_web::{delete, get, post, put, web, HttpResponse};

#[get("/todos")]
async fn list_todos(pool: web::Data<db::Pool>) -> Result<HttpResponse, ApiError> {
    let todos = db::list_todos(&pool).await?;
    Ok(HttpResponse::Ok().json(todos))
}

#[get("/todos/{id}")]
async fn get_todo(
    pool: web::Data<db::Pool>,
    id: web::Path<usize>,
) -> Result<HttpResponse, ApiError> {
    let todo = db::get_todo(&pool, *id).await?;
    Ok(HttpResponse::Ok().json(todo))
}

#[post("/todos")]
async fn create_todo(
    pool: web::Data<db::Pool>,
    todo: web::Json<db::CreateTodo>,
) -> Result<HttpResponse, ApiError> {
    let created = db::create_todo(&pool, todo.into_inner()).await?;
    Ok(HttpResponse::Ok().json(created))
}

#[put("/todos/{id}")]
async fn update_todo(
    pool: web::Data<db::Pool>,
    id: web::Path<usize>,
    todo: web::Json<db::CreateTodo>,
) -> Result<HttpResponse, ApiError> {
    let updated = db::update_todo(&pool, *id, todo.into_inner()).await?;
    Ok(HttpResponse::Ok().json(updated))
}

#[delete("/todos/{id}")]
async fn delete_todo(
    pool: web::Data<db::Pool>,
    id: web::Path<usize>,
) -> Result<HttpResponse, ApiError> {
    let deleted = db::delete_todo(&pool, *id).await?;
    Ok(HttpResponse::Ok().json(deleted))
}
