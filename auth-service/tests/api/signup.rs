use crate::helpers::TestApp;

#[tokio::test]
async fn signup_returns_200_ok() {
    let app = TestApp::new().await;

    let json = serde_json::json!({
        "email": "email@example.com",
        "password": "password",
        "requires2FA": false
    });

    let response = app.post_signup(&json).await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let random_email = TestApp::get_random_email();

    let test_cases = [
        serde_json::json!({
            "password": "password123",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": random_email,
            "pwd": "test321",
            "requires2FA": true
        })
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(&test_case).await; // call `post_signup`
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}