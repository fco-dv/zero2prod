use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::get_subscriber;
use zero2prod::telemetry::init_subscriber;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let settings: zero2prod::configuration::Settings =
        get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&settings.database.connection_string().expose_secret())
        .await
        .expect("Failed to get PgConnection");
    let address: String = format!("127.0.0.1:{}", settings.application_port);
    let tcp_listener: TcpListener = TcpListener::bind(address).unwrap();
    run(tcp_listener, connection_pool)?.await
}
