//! tests/health_check.rs

use reqwest;
use std::net::TcpListener;
use zero2prod;

// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute. //
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file) #[tokio::test]
#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client: reqwest::Client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let tcp_listener = TcpListener::bind("127.0.0.1:0").expect("Fail to create listener");
    let port = tcp_listener.local_addr().unwrap().port();
    let server: actix_web::dev::Server =
        zero2prod::run(tcp_listener).expect("Failed to create server");
    tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
