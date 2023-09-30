use crate::helpers::spawn_app;

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let url = &format!("{}/health_check", app.address);
    let path = url.as_str();
    println!("address {url}");
    let response = client
        .get(path)
        .send()
        .await
        .expect("Failed to execute request!!!");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
