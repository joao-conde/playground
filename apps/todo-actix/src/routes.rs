use crate::{
    app::AppData,
    db::{self, CreateTodo, UpdateTodo},
    error::ApiError,
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

#[cfg(test)]
mod test {
    use crate::{
        db::Todo,
        test::actix::{make_request, BoxBodyTest},
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
        let body: Vec<Todo> = response.into_body().deserialize().await;
        assert_eq!(status_code, StatusCode::OK);
        assert_eq!(body, vec![]);
    }
}
