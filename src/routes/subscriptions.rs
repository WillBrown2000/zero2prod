use actix_web::{web, web::Form, HttpResponse};
use sqlx::PgPool;
use tracing;
use uuid::Uuid;
use crate::domain::{NewSubscriber, SubscriberName};
use crate::domain::subscriber_email::SubscriberEmail;

#[derive(serde::Deserialize)]
pub struct Subscription {
    pub email: String,
    pub name: String,
}

impl TryFrom<Subscription> for NewSubscriber {
    type Error = String;

    fn try_from(form: Subscription) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(form.name)?;
        let email = SubscriberEmail::parse(form.email)?;
        Ok(NewSubscriber { name, email })
    }
}
#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool, email_client),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(
    form: Form<Subscription>,
    pool: web::Data<PgPool>,
    email_client: web::Data<crate::email_client::EmailClient>
) -> HttpResponse {

    let new_subscriber = match form.0.try_into()
    {
        Ok(subscriber) => subscriber,
        Err(_) => return HttpResponse::BadRequest().finish()
    };

    if insert_subscriber(&new_subscriber, &pool.get_ref()).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    // Include a confirmation link in both the HTML and text bodies
    let confirmation_link = "http://localhost/subscriptions/confirm";
    let html_body = format!(
        "Welcome to our newsletter!<br/>Click <a href=\"{0}\">here</a> to confirm your subscription.",
        confirmation_link
    );
    let text_body = format!(
        "Welcome to our newsletter!\nVisit {0} to confirm your subscription.",
        confirmation_link
    );

    if email_client
        .send_email(
            new_subscriber.email,
            "Welcome!",
            &html_body,
            &text_body,
        )
        .await
        .is_err()
    {
        return HttpResponse::InternalServerError().finish();
    }
    HttpResponse::Ok().finish()

}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(subscriber, pool)
)]
pub async fn insert_subscriber(subscriber: &NewSubscriber, pool: &PgPool) -> Result<(), sqlx::Error> {
    // Move out owned values so query bindings don't need references
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at, status)
        VALUES ($1, $2, $3, $4, 'confirmed')
        "#,
        Uuid::new_v4(),
        subscriber.email.as_ref(),
        subscriber.name.as_ref(),
        chrono::Utc::now(),
        )
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to save subscriber: {:?}", e);
            e
        })?;

        Ok(())
}
