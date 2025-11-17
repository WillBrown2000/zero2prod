use actix_web::{web, web::Form, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
use tracing::Instrument;

#[derive(serde::Deserialize)]
pub struct Subscription {
    pub email: String,
    pub name: String,
}

pub async fn subscribe(form: Form<Subscription>, pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
            "Adding a new subscriber.",
            %request_id,
            subscriber_email = form.email,
            subscriber_name = form.name
        );
    let _enter = request_span.enter();
    let query_span = tracing::info_span!("Received subscription started...");
    match sqlx::query!(
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
        .instrument(query_span)
        .await
    {
        Ok(_) => {
            tracing::info!( "request_id {} - Subscription saved successfully.", request_id);
            HttpResponse::Ok().finish()
        },

        Err(e) => {
            tracing::error!("request_id {} Failed to save subscription: {:?}", request_id, e);
            HttpResponse::InternalServerError().body(e.to_string())
        },
    }

}
