use std::net::TcpListener;
use sqlx::{Connection, PgConnection};
use zero2prod::configurations::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Read configuration (port, db, etc.)
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection = PgConnection::connect(&configuration.database.get_connection_string()
    ).await
    .expect("Failed to connect to database.");
    // Bind to the configured application port
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    // Run the HTTP server
    run(listener, connection)?.await
}
