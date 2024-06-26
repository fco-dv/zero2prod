//! tests/health_check.rs

use reqwest;
use std::net::TcpListener;
use zero2prod::startup::run;

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

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let address = spawn_app();
    let client: reqwest::Client = reqwest::Client::new();
    let form_data = "name=francois&email=francois@gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(form_data)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
}

#[tokio::test]
async fn subscribe_returns_a_400_for_non_valid_form_data() {
    let address = spawn_app();
    let client: reqwest::Client = reqwest::Client::new();
    let test_parameters = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_parameters {
        let response = client
            .post(&format!("{}/subscriptions", &address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        )
    }
}

fn spawn_app() -> String {
    let tcp_listener = TcpListener::bind("127.0.0.1:0").expect("Fail to create listener");
    let port = tcp_listener.local_addr().unwrap().port();
    let server: actix_web::dev::Server = run(tcp_listener).expect("Failed to create server");
    tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
