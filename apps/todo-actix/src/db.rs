use actix_web::web;
use serde::{Deserialize, Serialize};

use crate::error::ApiError;

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

async fn execute<T, F>(pool: &Pool, f: F) -> Result<T, ApiError>
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

pub async fn list_todos(pool: &Pool) -> Result<Vec<Todo>, ApiError> {
    let todos = execute(&pool, |conn| {
        conn.prepare("select * from todos")?
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

pub async fn get_todo(pool: &Pool, id: usize) -> Result<Todo, ApiError> {
    let todo = execute(&pool, move |conn| {
        conn.prepare("select * from todos where id = ?1")?
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

pub async fn create_todo(pool: &Pool, todo: CreateTodo) -> Result<Todo, ApiError> {
    let todo = execute(&pool, move |conn| {
        conn.prepare("insert into todos (title, description) values (?1, ?2) returning *")?
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

pub async fn update_todo(pool: &Pool, id: usize, todo: CreateTodo) -> Result<Todo, ApiError> {
    let todo = execute(&pool, move |conn| {
        conn.prepare("update todos set title = ?1, description = ?2 where id = ?3 returning *")?
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

pub async fn delete_todo(pool: &Pool, id: usize) -> Result<Todo, ApiError> {
    let todo = execute(&pool, move |conn| {
        conn.prepare("delete from todos where id = ?1 returning *")?
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
