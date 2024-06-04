use crate::routes;
use actix_web::web::{Data, ServiceConfig};
use sqlx::SqlitePool;

pub struct AppData {
    pub db_pool: SqlitePool,
}

pub fn configure_app(config: &mut ServiceConfig, db_pool: SqlitePool) {
    let app_data = Data::new(AppData { db_pool });
    config
        .app_data(app_data)
        .service(routes::list_todos)
        .service(routes::get_todo)
        .service(routes::create_todo)
        .service(routes::update_todo)
        .service(routes::delete_todo);
}
