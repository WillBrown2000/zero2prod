use std::net::TcpListener;
use zero2prod::configurations::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Read configuration (port, db, etc.)
    let configuration = get_configuration().expect("Failed to read configuration.");

    // Bind to the configured application port
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    // Run the HTTP server
    run(listener)?.await
}