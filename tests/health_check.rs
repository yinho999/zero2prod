use std::net::TcpListener;

#[actix_web::test]
async fn health_check() {
    // No .awit no .expect
    let port = spawn_app();

    // Bring reqwest to test our HTTP request
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/health_check", port))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch our application in the background ~somehow~
// No .await call, therefore no need for `spawn_app` to be async now.
// We are also running tests, so it is not worth it to propagate errors:
// if we fail to perform the required setup we can just panic and crash
// all the things.
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Fail to bind address");

    let port = listener.local_addr().unwrap().port();
    println!("Listening on port {}", port);

    let server = zero2prod::run(listener).expect("Fail to bind address");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = actix_rt::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
