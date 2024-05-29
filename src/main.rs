use std::net::TcpListener;
use zero2prod::startup;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let tcp_listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    startup::run(tcp_listener)?.await
}
