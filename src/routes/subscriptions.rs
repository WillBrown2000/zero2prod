use actix_web::{web, web::Form, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct Subscription {
    pub email: String,
    pub name: String,
}

// Minimal subscribe handler; extend later to persist to DB
pub async fn subscribe(
    form: Form<Subscription>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
sqlx::query!(
    r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
    "#,
    Uuid::new_v4(),
    form.email,
    form.name,
    Utc::now()
)
    .execute(pool.get_ref())
    .await.expect("can't connect to database");
    HttpResponse::Ok().finish()
}
