use reqwest;

#[tokio::test]
async fn test_health_check() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
}

fn spawn_app() -> String {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("failed to start server");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}