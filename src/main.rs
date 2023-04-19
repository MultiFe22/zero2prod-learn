use sqlx::PgPool;
use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Panic if we can't read the configuration.
    let configuration = get_configuration().expect("Failed to read configuration.");

    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    // Now the port is read from the configuration file.
    let address = format!("127.0.0.1:{}", configuration.application_port);
    // print the address to the console.
    println!("Listening on http://{}", address);
    // Set listener to run on port 0, which will assign a random port.
    let listener = std::net::TcpListener::bind(address)?;
    // Run the server.
    run(listener, connection_pool)?.await
}
