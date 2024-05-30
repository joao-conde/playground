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
    use crate::{app::configure_app, test::BoxBodyTest};
    use actix_web::{http::StatusCode, test, App};
    use sqlx::SqlitePool;

    #[sqlx::test(fixtures("todos"))]
    async fn list_todos(pool: SqlitePool) {
        let app = App::new().configure(|config| configure_app(config, pool));
        let app = test::init_service(app).await;
        let request = test::TestRequest::default().uri("/todos").to_request();
        let response = test::call_service(&app, request).await;
        let status_code = response.status();
        let body = response.into_body().to_string().await;
        assert_eq!(
            body,
            r#"[
                {
                    "id": 1,
                    "title": "TODO API",
                    "description": "Build a TODO API with Actix Web and SQLX"
                },
                {
                    "id": 2,
                    "title": "Fix home printer",
                    "description": "Fix the home printer ASAP because my college degree ain't paying itself"
                },
                {
                    "id": 3,
                    "title": "Update CV",
                    "description": "Update CV ASAP to send to that dream Rust job"
                }
            ]"#
        );
        assert_eq!(status_code, StatusCode::OK);
    }

    #[sqlx::test]
    async fn list_todos_empty(pool: SqlitePool) {
        let app = App::new().configure(|config| configure_app(config, pool));
        let app = test::init_service(app).await;
        let request = test::TestRequest::default().uri("/todos").to_request();
        let response = test::call_service(&app, request).await;
        let status_code = response.status();
        let body = response.into_body().to_string().await;
        assert_eq!(body, "[]");
        assert_eq!(status_code, StatusCode::OK);
    }
}
