use sqlx::postgres::{PgConnectOptions, PgSslMode};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

use zero2prod::configuration::{get_configuration, DatabaseSettings};
use zero2prod::startup::{get_connection_pool, Application};

pub struct TestApp {
    pub address: String,
    pub pool: PgPool,
}


pub async fn spawn_app() -> TestApp {
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        c.database.database_name = Uuid::new_v4().to_string();
        // Ask OS for an available port
        c.application.port = 0;
        c
    };

    // Build the application first to learn which port was actually bound
    let application = Application::build(configuration.clone()).await.unwrap();
    // Run DB migrations before we start serving requests
    configure_database(&configuration.database).await;

    // Compose a valid absolute URL that reqwest can use
    let address = format!("http://{}:{}", configuration.application.host, application.port());

    // Launch the server in the background
    let _ = tokio::spawn(application.run_until_stopped());

    TestApp {
        address,
        pool: get_connection_pool(&configuration.database),
    }
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
