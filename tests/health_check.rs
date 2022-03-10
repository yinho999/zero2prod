// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file

use std::net::TcpListener;

// When we receive a GET request for /health_check we return a 200 OK response with no body.
#[actix_web::test]
async fn health_check() {
    // No .await, no .expect
    let address = spawn_app();

    // We need to bring in `reqwest`
    // to perform HTTP requests against our application
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch our application in the background ~somehow~
fn spawn_app() -> String {
    // How do we find a random available port for our tests?
    // The operating system comes to the rescue: we will be using port 0.
    // Port 0 is special-cased at the OS level: trying to bind port 0 will trigger an OS scan for an available
    // port which will then be bound to the application.
    let listener = TcpListener::bind("127.0.0.1:0").expect("Fail to find a random port");

    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = actix_rt::spawn(server);
    // We return the application address to the caller!
    format!("http://127.0.0.1:{}", port)
}
