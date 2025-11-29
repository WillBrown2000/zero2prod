use std::net::TcpListener;
use std::time::Duration;

use sqlx::{Connection, Executor, PgConnection, PgPool};
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use uuid::Uuid;

use zero2prod::configuration::{get_configuration, DatabaseSettings};
use zero2prod::email_client::EmailClient;
use zero2prod::startup::run;

pub struct TestApp {
    pub address: String,
    pub pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    // Load configuration and randomize the database name to ensure test isolation
    let mut configuration = get_configuration().expect("Failed to read configuration");
    configuration.database.database_name = Uuid::new_v4().to_string();

    // Configure and migrate the database
    let pool = configure_database(&configuration.database).await;

    // Bind the application to a random available port
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    // Build the EmailClient from configuration
    let email_settings = configuration.email_client;
    let sender = email_settings.sender().expect("Invalid sender email in configuration");
    let base_url = if email_settings.base_url.starts_with("http") {
        email_settings.base_url
    } else {
        format!("http://{}", email_settings.base_url)
    };
    let email_client = EmailClient::new(
        base_url,
        sender,
        Duration::from_secs(10),
        email_settings.authorization_token,
    );

    // Launch the application as a background task
    let server = run(listener, pool.clone(), email_client).expect("Failed to bind address");
    tokio::spawn(server);

    TestApp { address, pool }
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database without selecting a specific DB first
    let mut options = PgConnectOptions::new()
        .host(&config.host)
        .username(&config.username)
        .password(&config.password)
        .port(config.port)
        .ssl_mode(if config.require_ssl { PgSslMode::Require } else { PgSslMode::Prefer });

    // Connect to the Postgres instance (no database selected)
    let mut connection = PgConnection::connect_with(&options)
        .await
        .expect("Failed to connect to Postgres");

    // Create the database
    let db_name = &config.database_name;
    connection
        .execute(format!(r#"CREATE DATABASE "{}""#, db_name).as_str())
        .await
        .expect("Failed to create database.");

    // Now connect to the newly created database
    options = options.database(db_name);
    let pool = PgPool::connect_with(options)
        .await
        .expect("Failed to connect to Postgres with created database");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate the database");

    pool
}
