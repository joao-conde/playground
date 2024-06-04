use crate::{
    app::AppData,
    db,
    error::ApiError,
    todo::{CreateTodo, UpdateTodo},
};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ApiResponse<T> {
    pub result: T,
    pub errors: Vec<ApiError>,
}

#[get("/todos")]
async fn list_todos(app_data: Data<AppData>) -> Result<HttpResponse, ApiError> {
    let todos = db::list_todos(&app_data.db_pool).await?;
    let response = ApiResponse {
        result: todos,
        errors: vec![],
    };
    Ok(HttpResponse::Ok().json(response))
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

#[cfg(test)]
mod test {
    use crate::{
        routes::ApiResponse,
        test::{make_request, BoxBodyTest},
        todo::Todo,
    };
    use actix_web::{http::StatusCode, test};
    use sqlx::SqlitePool;

    #[sqlx::test(fixtures("test/fixtures/todos.sql"))]
    async fn list_todos(pool: SqlitePool) {
        let request = test::TestRequest::default().uri("/todos");
        let response = make_request(pool, request).await;

        let status_code = response.status();
        let body: Vec<Todo> = response.into_body().deserialize().await;
        assert_eq!(status_code, StatusCode::OK);
        assert_eq!(
            body,
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
        let request = test::TestRequest::default().uri("/todos");
        let response = make_request(pool, request).await;

        let status_code = response.status();
        let body: ApiResponse<Vec<Todo>> = response.into_body().deserialize().await;
        assert_eq!(status_code, StatusCode::OK);
        assert_eq!(body.result, vec![]);
    }

    #[sqlx::test(fixtures("test/fixtures/todos.sql"))]
    async fn get_todo(pool: SqlitePool) {
        let request = test::TestRequest::default().uri("/todos/2");
        let response = make_request(pool, request).await;

        let status_code = response.status();
        let body: ApiResponse<Todo> = response.into_body().deserialize().await;
        assert_eq!(status_code, StatusCode::OK);
        assert_eq!(
            body.result,
            Todo {
                id: 2,
                title: "todo2".to_string(),
                description: "description2".to_string()
            }
        );
    }

    #[sqlx::test]
    async fn get_todo_not_found(pool: SqlitePool) {
        let request = test::TestRequest::default().uri("/todos/2");
        let response = make_request(pool, request).await;

        let status_code = response.status();
        let body = response.into_body().as_str().await;
        assert_eq!(status_code, StatusCode::NOT_FOUND);
        assert_eq!(body, "Not Found");
    }
}
