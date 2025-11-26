use sqlx::PgPool;
use std::net::TcpListener;
use std::time::Duration;
use zero2prod::configuration::get_configuration;
use zero2prod::email_client::EmailClient;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Set up telemetry first
    let subscriber = get_subscriber(
        "zero2prod".to_string(),
        "info".to_string(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    // Load configuration
    let configuration = get_configuration().expect("Failed to read configuration.");

    // Construct EmailClient
    let sender_email = configuration
        .email_client
        .sender()
        .expect("Invalid sender email in configuration");
    let timeout = Duration::from_secs(10);
    let email_client = EmailClient::new(
        configuration.email_client.base_url.clone(),
        sender_email,
        timeout,
        configuration.email_client.authorization_token,
    );

    let pool = PgPool::connect_lazy_with(configuration.database.connection_options());
    let address = format!("{}:{}", configuration.application.host, configuration.application.port);
    let listener = TcpListener::bind(address)?;
    run(listener, pool, email_client)?.await
}

