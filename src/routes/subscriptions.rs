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

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        request_id = %Uuid::new_v4(),
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(
    form: Form<Subscription>,
    pool: web::Data<PgPool>
) -> HttpResponse {
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
            HttpResponse::Ok().finish()
        },

        Err(e) => {
            tracing::error!(" Failed to save subscription: {:?}", e);
            HttpResponse::InternalServerError().body(e.to_string())
        },
    }

}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(subscriber, pool)
)]
pub async fn insert_subscriber(subscriber: &Subscription, pool: &PgPool) -> Result<(), sqlx::Error> {

    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        subscriber.email,
        subscriber.name,
        Utc::now()
        )
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to save subscriber: {:?}", e);
            e
        })?;
        Ok(())
}
