use crate::{
    error::InternalError,
    todo::{CreateTodo, Todo, UpdateTodo},
};
use sqlx::SqlitePool;

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
    use crate::{
        db,
        error::InternalError,
        todo::{CreateTodo, Todo, UpdateTodo},
    };
    use assert_matches::assert_matches;
    use sqlx::SqlitePool;

    #[sqlx::test(fixtures("test/fixtures/todos.sql"))]
    async fn list_todos(pool: SqlitePool) {
        let todos = db::list_todos(&pool).await.unwrap();
        assert_eq!(
            todos,
            vec![
                Todo {
                    id: 1,
                    title: "todo1".to_string(),
                    description: "description1".to_string()
                },
                Todo {
                    id: 2,
                    title: "todo2".to_string(),
                    description: "description2".to_string()
                },
                Todo {
                    id: 3,
                    title: "todo3".to_string(),
                    description: "description3".to_string()
                }
            ]
        );
    }

    #[sqlx::test]
    async fn list_todos_empty(pool: SqlitePool) {
        let todos = db::list_todos(&pool).await.unwrap();
        assert_eq!(todos, vec![]);
    }

    #[sqlx::test(fixtures("test/fixtures/todos.sql"))]
    async fn get_todo(pool: SqlitePool) {
        let todo = db::get_todo(&pool, 2).await.unwrap();
        assert_eq!(
            todo,
            Todo {
                id: 2,
                title: "todo2".to_string(),
                description: "description2".to_string()
            },
        );
    }

    #[sqlx::test]
    async fn get_todo_not_found(pool: SqlitePool) {
        let err = db::get_todo(&pool, -1).await;
        assert_matches!(err, Err(InternalError::Sql(sqlx::Error::RowNotFound)));
    }

    #[sqlx::test]
    async fn create_todo(pool: SqlitePool) {
        let todo = db::create_todo(
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

        let todo = db::get_todo(&pool, 1).await.unwrap();
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
        let todo = db::get_todo(&pool, 2).await.unwrap();
        assert_eq!(
            todo,
            Todo {
                id: 2,
                title: "todo2".to_string(),
                description: "description2".to_string()
            },
        );

        let todo = db::update_todo(
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

        let todo = db::get_todo(&pool, 2).await.unwrap();
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
        let err = db::update_todo(
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
        let todo = db::get_todo(&pool, 2).await.unwrap();
        assert_eq!(
            todo,
            Todo {
                id: 2,
                title: "todo2".to_string(),
                description: "description2".to_string()
            },
        );

        let todo = db::delete_todo(&pool, 2).await.unwrap();
        assert_eq!(
            todo,
            Todo {
                id: 2,
                title: "todo2".to_string(),
                description: "description2".to_string()
            },
        );

        let err = db::get_todo(&pool, 2).await;
        assert_matches!(err, Err(InternalError::Sql(sqlx::Error::RowNotFound)));
    }

    #[sqlx::test]
    async fn delete_todo_not_found(pool: SqlitePool) {
        let err = db::delete_todo(&pool, -1).await;
        assert_matches!(err, Err(InternalError::Sql(sqlx::Error::RowNotFound)));
    }
}
