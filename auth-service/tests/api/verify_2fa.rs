use crate::helpers::TestApp;

#[tokio::test]
async fn verify2fa_returns_200_ok() {
    let app = TestApp::new().await;

    let response = app.post_verify_2fa("123123123").await;

    assert_eq!(response.status().as_u16(), 200);
}