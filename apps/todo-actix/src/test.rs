use actix_web::body::{to_bytes, BoxBody};

pub trait BoxBodyTest {
    async fn to_string(self) -> String;
}

impl BoxBodyTest for BoxBody {
    async fn to_string(self) -> String {
        let body = to_bytes(self).await.unwrap();
        std::str::from_utf8(&body).unwrap().to_string()
    }
}
