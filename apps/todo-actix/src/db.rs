use actix_web::{body::BoxBody, http::StatusCode, web, HttpResponse};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

#[derive(Debug)]
pub enum ApiError {
    NotFound,
    Internal,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::NotFound => write!(f, "Not Found Error"),
            ApiError::Internal => write!(f, "Internal Server Error"),
        }
    }
}

impl actix_web::ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::new(self.status_code())
    }
}

impl From<rusqlite::Error> for ApiError {
    fn from(value: rusqlite::Error) -> Self {
        match value {
            rusqlite::Error::QueryReturnedNoRows => Self::NotFound,
            _ => Self::Internal,
        }
    }
}

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

async fn execute<T, F>(pool: &Pool, f: F) -> T
where
    T: Send + 'static,
    F: Send + 'static,
    F: FnOnce(Connection) -> T,
{
    let pool = pool.clone();
    let conn = web::block(move || pool.get()).await.unwrap().unwrap();
    web::block(move || f(conn)).await.unwrap()
}

pub async fn list_todos(pool: &Pool) -> Result<Vec<Todo>, rusqlite::Error> {
    let todos = execute(&pool, |conn| {
        conn.prepare("select * from todos")
            .unwrap()
            .query_map([], |row| {
                Ok(Todo {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    description: row.get(2)?,
                })
            })
            .and_then(Iterator::collect)
    })
    .await;
    todos
}

pub async fn create_todo(pool: &Pool, todo: CreateTodo) -> Result<Todo, rusqlite::Error> {
    let todo = execute(&pool, move |conn| {
        conn.prepare("insert into todos (title, description) values (?1, ?2) returning *")
            .unwrap()
            .query_row([todo.title, todo.description], |row| {
                Ok(Todo {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    description: row.get(2)?,
                })
            })
    })
    .await;
    todo
}

pub async fn get_todo(pool: &Pool, id: usize) -> Result<Todo, rusqlite::Error> {
    let todos = execute(&pool, move |conn| {
        conn.prepare("select * from todos where id = ?1")
            .unwrap()
            .query_row([id], |row| {
                Ok(Todo {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    description: row.get(2)?,
                })
            })
    })
    .await;
    todos
}

pub async fn delete_todo(pool: &Pool, id: usize) -> Result<Todo, rusqlite::Error> {
    let todos = execute(&pool, move |conn| {
        conn.prepare("delete from todos where id = ?1 returning *")
            .unwrap()
            .query_row([id], |row| {
                Ok(Todo {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    description: row.get(2)?,
                })
            })
    })
    .await;
    todos
}
