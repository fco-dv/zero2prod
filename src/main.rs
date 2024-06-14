use env_logger::Env;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let settings: zero2prod::configuration::Settings =
        get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&settings.database.connection_string())
        .await
        .expect("Failed to get PgConnection");
    let address: String = format!("127.0.0.1:{}", settings.application_port);
    let tcp_listener: TcpListener = TcpListener::bind(address).unwrap();
    run(tcp_listener, connection_pool)?.await
}
