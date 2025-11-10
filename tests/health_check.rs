use reqwest;

#[tokio::test]
async fn test_health_check() {
    spawn_app();
    let client = reqwest::Client::new();

    let response = client.get("http://127.0.0.1:8080/health_check").send().await.unwrap();
    assert_eq!(response.status(), 200);
}

fn spawn_app()  {
    let server = zero2prod::run().expect("failed to start server");
    let _ = tokio::spawn(server);
}