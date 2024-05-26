//! tests/health_check.rs

use reqwest;
use zero2prod;

// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute. //
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file) #[tokio::test]
#[tokio::test]
async fn health_check_works() {
    spawn_app();
    let client: reqwest::Client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}


fn spawn_app() -> () {
    let server: actix_web::dev::Server = zero2prod::run().expect("erreur serveur");
    let _ = tokio::spawn(server);
}
