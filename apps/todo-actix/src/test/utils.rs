use actix_web::body::{to_bytes, BoxBody};
use serde::de::DeserializeOwned;

pub trait BoxBodyTest {
    async fn deserialize<T: DeserializeOwned>(self) -> T;
}

impl BoxBodyTest for BoxBody {
    async fn deserialize<T>(self) -> T
    where
        T: DeserializeOwned,
    {
        let body = to_bytes(self).await.unwrap();
        let data = std::str::from_utf8(&body).unwrap().to_string();
        serde_json::from_str(&data).unwrap()
    }
}
