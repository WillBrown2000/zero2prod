use actix_web::{web::Form, HttpResponse};

#[derive(serde::Deserialize)]
pub struct Subscription {
    pub email: String,
    pub name: String,
}

// Minimal subscribe handler; extend later to persist to DB
pub async fn subscribe(_form: Form<Subscription>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
