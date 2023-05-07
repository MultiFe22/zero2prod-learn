use sqlx::postgres::PgPoolOptions;
use zero2prod::telemetry::{get_subscriber, init_subscriber};
use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Panic if we can't read the configuration.
    let configuration = get_configuration().expect("Failed to read configuration.");

    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());

    // Now the port is read from the configuration file.
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    // print the address to the console.
    println!("Listening on http://{}", address);
    // Set listener to run on port 0, which will assign a random port.
    let listener = std::net::TcpListener::bind(address)?;
    // Run the server.
    run(listener, connection_pool)?.await?;

    Ok(())
}
