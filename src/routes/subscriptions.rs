use actix_web::{web, web::Form, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
use tracing;
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
    let name = match SubscriberName::parse(form.0.name) {
        Ok(name) => name,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    
    let new_subscriber = NewSubscriber {
        name,
        email: form.0.email,
    };
    match insert_subscriber(&new_subscriber, &pool.get_ref()).await
    {
        Ok(_) => {
            HttpResponse::Ok().finish()
        },

        Err(e) => {
            tracing::error!(" Failed to save subscription: {:?}", e);
            HttpResponse::InternalServerError().finish()
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
