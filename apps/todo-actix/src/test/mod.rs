use crate::app::configure_app;
use actix_web::{
    body::{to_bytes, BoxBody},
    dev::ServiceResponse,
    test, App,
};
use serde::de::DeserializeOwned;
use sqlx::SqlitePool;

pub trait BoxBodyTest {
    async fn deserialize<T: DeserializeOwned>(&mut self) -> T;
    async fn as_str(&mut self) -> String;
}

impl BoxBodyTest for BoxBody {
    async fn deserialize<T>(&mut self) -> T
    where
        T: DeserializeOwned,
    {
        let body = to_bytes(self).await.unwrap();
        let data = std::str::from_utf8(&body).unwrap().to_string();
        serde_json::from_str(&data).unwrap()
    }

    async fn as_str(&mut self) -> String {
        let body = to_bytes(self).await.unwrap();
        std::str::from_utf8(&body).unwrap().to_string()
    }
}

pub async fn make_request(pool: SqlitePool, request: test::TestRequest) -> ServiceResponse {
    let app = App::new().configure(|config| configure_app(config, pool));
    let app = test::init_service(app).await;
    let response = test::call_service(&app, request.to_request()).await;
    response
}
