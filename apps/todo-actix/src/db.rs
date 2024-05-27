use actix_web::web;
use serde::{Deserialize, Serialize};

use crate::error::InternalError;

pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

#[derive(Deserialize, Serialize)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct CreateTodo {
    pub title: String,
    pub description: String,
}

async fn execute<T, F>(pool: &Pool, f: F) -> Result<T, InternalError>
where
    T: Send + 'static,
    F: Send + 'static,
    F: FnOnce(Connection) -> T,
{
    let pool = pool.clone();
    let conn = web::block(move || pool.get()).await??;
    let res = web::block(move || f(conn)).await?;
    Ok(res)
}

pub async fn list_todos(pool: &Pool) -> Result<Vec<Todo>, InternalError> {
    let todos = execute(pool, |conn| {
        conn.prepare("SELECT * FROM todos")?
            .query_map([], |row| {
                Ok(Todo {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    description: row.get(2)?,
                })
            })
            .and_then(Iterator::collect)
    })
    .await??;
    Ok(todos)
}

pub async fn get_todo(pool: &Pool, id: usize) -> Result<Todo, InternalError> {
    let todo = execute(pool, move |conn| {
        conn.prepare("SELECT * FROM todos WHERE id = ?1")?
            .query_row([id], |row| {
                Ok(Todo {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    description: row.get(2)?,
                })
            })
    })
    .await??;
    Ok(todo)
}

pub async fn create_todo(pool: &Pool, todo: CreateTodo) -> Result<Todo, InternalError> {
    let todo = execute(pool, move |conn| {
        conn.prepare("INSERT INTO todos (title, description) VALUES (?1, ?2) RETURNING *")?
            .query_row([todo.title, todo.description], |row| {
                Ok(Todo {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    description: row.get(2)?,
                })
            })
    })
    .await??;
    Ok(todo)
}

pub async fn update_todo(pool: &Pool, id: usize, todo: CreateTodo) -> Result<Todo, InternalError> {
    let todo = execute(pool, move |conn| {
        conn.prepare("UPDATE todos SET title = ?1, description = ?2 WHERE id = ?3 RETURNING *")?
            .query_row([todo.title, todo.description, id.to_string()], |row| {
                Ok(Todo {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    description: row.get(2)?,
                })
            })
    })
    .await??;
    Ok(todo)
}

pub async fn delete_todo(pool: &Pool, id: usize) -> Result<Todo, InternalError> {
    let todo = execute(pool, move |conn| {
        conn.prepare("DELETE FROM todos WHERE id = ?1 RETURNING *")?
            .query_row([id], |row| {
                Ok(Todo {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    description: row.get(2)?,
                })
            })
    })
    .await??;
    Ok(todo)
}
