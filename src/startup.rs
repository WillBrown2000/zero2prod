use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;
use crate::email_client::EmailClient;
use crate::configuration::Settings;
use sqlx::postgres::{ PgPoolOptions};

pub fn run(
    tcp_listener: TcpListener,
    db_pool: PgPool,
    email_client: EmailClient
    ) -> Result<Server, std::io::Error> {
    let pool = web::Data::new(db_pool);
    let email_client = web::Data::new(email_client);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/subscriptions", web::post().to(subscribe))
            .route("/health_check", web::get().to(health_check))
            // Provide the existing Data<PgPool> to the app; avoid wrapping Data inside Data
            .app_data(pool.clone())
            .app_data(email_client.clone())
    })
    .listen(tcp_listener)?
    .run();
    Ok(server)
}

pub fn build(configuration: Settings) -> Result<Server, std::io::Error> {
    let connection_pool = PgPoolOptions::new().connect_lazy_with(configuration.database.connection_options());
    
    let sender_email = configuration.email_client.sender().expect("Invalid sender email in configuration");
    let timeout = configuration.email_client.timeout;
    let email_client = EmailClient::new(configuration.email_client.base_url, sender_email, timeout, configuration.email_client.authorization_token);
    let address = format!("{}:{}", configuration.application.host, configuration.application.port);
    let listener = TcpListener::bind(address)?;
    run(std::net::TcpListener::bind(format!("{}:{}", configuration.application.host, configuration.application.port))?, connection_pool, email_client)
}


