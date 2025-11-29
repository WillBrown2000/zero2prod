use sqlx::PgPool;
use std::net::TcpListener;
use std::time::Duration;
use zero2prod::configuration::get_configuration;
use zero2prod::email_client::EmailClient;
use zero2prod::startup::{build, run};
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

    let server = build(configuration);
    Ok(())

}

