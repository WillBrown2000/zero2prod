use actix_web::{web, web::Form, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
use tracing::Instrument;
use crate::domain::{NewSubscriber, SubscriberName};

#[derive(serde::Deserialize)]
pub struct Subscription {
    pub email: String,
    pub name: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(
    form: Form<Subscription>,
    pool: web::Data<PgPool>
) -> HttpResponse {
    let new_subscriber = NewSubscriber {
        name: SubscriberName::parse(form.name.clone()),
        email: form.email.clone(),
    };
    match insert_subscriber(&new_subscriber, &pool.get_ref()).await
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
pub async fn insert_subscriber(subscriber: &NewSubscriber, pool: &PgPool) -> Result<(), sqlx::Error> {
    // Move out owned values so query bindings don't need references
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        subscriber.email,
        subscriber.name.as_ref(),
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
