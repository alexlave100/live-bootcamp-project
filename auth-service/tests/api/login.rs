use crate::helpers::TestApp;

#[tokio::test]
async fn login_returns_200_ok() {
    let app = TestApp::new().await;

    let json = serde_json::json!({
        "email": "email@example.com",
        "password": "password",
    });

    let response = app.post_login(&json).await;

    assert_eq!(response.status().as_u16(), 200);
}