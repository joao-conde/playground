use crate::{
    db::{self, CreateTodo, UpdateTodo},
    error::ApiError,
    AppData,
};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};

#[get("/todos")]
async fn list_todos(app_data: Data<AppData>) -> Result<HttpResponse, ApiError> {
    let todos = db::list_todos(&app_data.db_pool).await?;
    Ok(HttpResponse::Ok().json(todos))
}

#[get("/todos/{id}")]
async fn get_todo(app_data: Data<AppData>, id: Path<i64>) -> Result<HttpResponse, ApiError> {
    let todo = db::get_todo(&app_data.db_pool, *id).await?;
    Ok(HttpResponse::Ok().json(todo))
}

#[post("/todos")]
async fn create_todo(
    app_data: Data<AppData>,
    todo: Json<CreateTodo>,
) -> Result<HttpResponse, ApiError> {
    let created = db::create_todo(&app_data.db_pool, todo.into_inner()).await?;
    Ok(HttpResponse::Ok().json(created))
}

#[put("/todos/{id}")]
async fn update_todo(
    app_data: Data<AppData>,
    id: Path<i64>,
    todo: Json<UpdateTodo>,
) -> Result<HttpResponse, ApiError> {
    let updated = db::update_todo(&app_data.db_pool, *id, todo.into_inner()).await?;
    Ok(HttpResponse::Ok().json(updated))
}

#[delete("/todos/{id}")]
async fn delete_todo(app_data: Data<AppData>, id: Path<i64>) -> Result<HttpResponse, ApiError> {
    let deleted = db::delete_todo(&app_data.db_pool, *id).await?;
    Ok(HttpResponse::Ok().json(deleted))
}
