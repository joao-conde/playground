use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, FromRow)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub description: String,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct CreateTodo {
    pub title: String,
    pub description: String,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct UpdateTodo {
    pub title: String,
    pub description: String,
}
