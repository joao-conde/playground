use crate::error::InternalError;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, FromRow)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub description: String,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct CreateTodo {
    pub title: String,
    pub description: String,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct UpdateTodo {
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
    todo: UpdateTodo,
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

#[cfg(test)]
mod test {
    use super::{CreateTodo, Todo, UpdateTodo};
    use crate::error::InternalError;
    use assert_matches::assert_matches;
    use sqlx::SqlitePool;

    #[sqlx::test(fixtures("test/fixtures/todos.sql"))]
    async fn list_todos(pool: SqlitePool) {
        let todos = super::list_todos(&pool).await.unwrap();
        assert_eq!(
            todos,
            vec![
                Todo {
                    id: 1,
                    title: "TODO API".to_string(),
                    description: "Build a TODO API with Actix Web and SQLX".to_string()
                },
                Todo {
                    id: 2,
                    title: "Fix home printer".to_string(),
                    description:
                        "Fix the home printer ASAP because my college degree ain't paying itself"
                            .to_string()
                },
                Todo {
                    id: 3,
                    title: "Update CV".to_string(),
                    description: "Update CV ASAP to send to that dream Rust job".to_string()
                }
            ]
        );
    }

    #[sqlx::test]
    async fn list_todos_empty(pool: SqlitePool) {
        let todos = super::list_todos(&pool).await.unwrap();
        assert_eq!(todos, vec![]);
    }

    #[sqlx::test(fixtures("test/fixtures/todos.sql"))]
    async fn get_todo(pool: SqlitePool) {
        let todo = super::get_todo(&pool, 2).await.unwrap();
        assert_eq!(
            todo,
            Todo {
                id: 2,
                title: "Fix home printer".to_string(),
                description:
                    "Fix the home printer ASAP because my college degree ain't paying itself"
                        .to_string()
            },
        );
    }

    #[sqlx::test]
    async fn get_todo_not_found(pool: SqlitePool) {
        let err = super::get_todo(&pool, -1).await;
        assert_matches!(err, Err(InternalError::Sql(sqlx::Error::RowNotFound)));
    }

    #[sqlx::test]
    async fn create_todo(pool: SqlitePool) {
        let todo = super::create_todo(
            &pool,
            CreateTodo {
                title: "title".to_string(),
                description: "description".to_string(),
            },
        )
        .await
        .unwrap();
        assert_eq!(
            todo,
            Todo {
                id: 1,
                title: "title".to_string(),
                description: "description".to_string(),
            }
        );

        let todo = super::get_todo(&pool, 1).await.unwrap();
        assert_eq!(
            todo,
            Todo {
                id: 1,
                title: "title".to_string(),
                description: "description".to_string(),
            }
        );
    }

    #[sqlx::test(fixtures("test/fixtures/todos.sql"))]
    async fn update_todo(pool: SqlitePool) {
        let todo = super::get_todo(&pool, 2).await.unwrap();
        assert_eq!(
            todo,
            Todo {
                id: 2,
                title: "Fix home printer".to_string(),
                description:
                    "Fix the home printer ASAP because my college degree ain't paying itself"
                        .to_string()
            },
        );

        let todo = super::update_todo(
            &pool,
            2,
            UpdateTodo {
                title: "title".to_string(),
                description: "description".to_string(),
            },
        )
        .await
        .unwrap();
        assert_eq!(
            todo,
            Todo {
                id: 2,
                title: "title".to_string(),
                description: "description".to_string(),
            }
        );

        let todo = super::get_todo(&pool, 2).await.unwrap();
        assert_eq!(
            todo,
            Todo {
                id: 2,
                title: "title".to_string(),
                description: "description".to_string(),
            },
        );
    }

    #[sqlx::test]
    async fn update_todo_not_found(pool: SqlitePool) {
        let err = super::update_todo(
            &pool,
            -1,
            UpdateTodo {
                title: "title".to_string(),
                description: "description".to_string(),
            },
        )
        .await;
        assert_matches!(err, Err(InternalError::Sql(sqlx::Error::RowNotFound)));
    }

    #[sqlx::test(fixtures("test/fixtures/todos.sql"))]
    async fn delete_todo(pool: SqlitePool) {
        let todo = super::get_todo(&pool, 2).await.unwrap();
        assert_eq!(
            todo,
            Todo {
                id: 2,
                title: "Fix home printer".to_string(),
                description:
                    "Fix the home printer ASAP because my college degree ain't paying itself"
                        .to_string()
            },
        );

        let todo = super::delete_todo(&pool, 2).await.unwrap();
        assert_eq!(
            todo,
            Todo {
                id: 2,
                title: "Fix home printer".to_string(),
                description:
                    "Fix the home printer ASAP because my college degree ain't paying itself"
                        .to_string()
            },
        );

        let err = super::get_todo(&pool, 2).await;
        assert_matches!(err, Err(InternalError::Sql(sqlx::Error::RowNotFound)));
    }

    #[sqlx::test]
    async fn delete_todo_not_found(pool: SqlitePool) {
        let err = super::delete_todo(&pool, -1).await;
        assert_matches!(err, Err(InternalError::Sql(sqlx::Error::RowNotFound)));
    }
}
