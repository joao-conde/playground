use crate::error::InternalError;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

#[derive(Deserialize, Serialize, FromRow)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct CreateTodo {
    pub title: String,
    pub description: String,
}

pub async fn list_todos(pool: &SqlitePool) -> Result<Vec<Todo>, InternalError> {
    let todos: Vec<Todo> = sqlx::query_as!(Todo, "SELECT * FROM todos")
        .fetch_all(pool)
        .await?;
    Ok(todos)
}

pub async fn get_todo(pool: &SqlitePool, id: i64) -> Result<Todo, InternalError> {
    let todo = sqlx::query_as!(Todo, "SELECT * FROM todos WHERE id = ?", id)
        .fetch_one(pool)
        .await?;
    Ok(todo)
}

pub async fn create_todo(pool: &SqlitePool, todo: CreateTodo) -> Result<Todo, InternalError> {
    let todo = sqlx::query_as!(
        Todo,
        "INSERT INTO todos (title, description) VALUES (?, ?) RETURNING *",
        todo.title,
        todo.description
    )
    .fetch_one(pool)
    .await?;
    Ok(todo)
}

pub async fn update_todo(
    pool: &SqlitePool,
    id: i64,
    todo: CreateTodo,
) -> Result<Todo, InternalError> {
    sqlx::query_as!(
        Todo,
        "UPDATE todos SET title = ?, description = ? WHERE id = ?",
        todo.title,
        todo.description,
        id,
    )
    .execute(pool)
    .await?;
    let todo = get_todo(pool, id).await?;
    Ok(todo)
}

pub async fn delete_todo(pool: &SqlitePool, id: i64) -> Result<Todo, InternalError> {
    let todo = sqlx::query_as!(Todo, "DELETE FROM todos WHERE id = ? RETURNING *", id)
        .fetch_one(pool)
        .await?;
    Ok(todo)
}
