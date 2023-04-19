use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Set listener to run on port 0, which will assign a random port.
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // Run the server.
    run(listener)?.await
}