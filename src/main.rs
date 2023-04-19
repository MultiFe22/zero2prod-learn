use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Panic if we can't read the configuration.
    let configuration = get_configuration().expect("Failed to read configuration.");
    // Now the port is read from the configuration file.
    let address = format!("127.0.0.1:{}", configuration.application_port);
    // Set listener to run on port 0, which will assign a random port.
    let listener = std::net::TcpListener::bind(address)?;
    // Run the server.
    run(listener)?.await
}
