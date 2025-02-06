use auth_service::utils::JWT_COOKIE_NAME;

use crate::helpers::TestApp;

// #[tokio::test]
// async fn login_returns_200_ok() {
//     let app = TestApp::new().await;

//     let json = serde_json::json!({
//         "email": "email@example.com",
//         "password": "password",
//     });

//     let response = app.post_login(&json).await;

//     assert_eq!(response.status().as_u16(), 201);
// }

#[tokio::test]
async fn should_return_200_if_valid_credentials_and_2fa_disabled() {
    let app = TestApp::new().await;

    let random_email = "testuser@email.com"; //get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let app = TestApp::new().await;

    let signup_body = serde_json::json!({
        "email": "test@example.com",
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let json_body = serde_json::json!({
        "email": "test@example.com",
        "password": "password234"
    });

    let response = app.post_login(&json_body).await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    let signup_body = serde_json::json!({
        "email": "test@example.com",
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let json = serde_json::json!({
        "email": "testexample.com",
        "password": "password",
    });

    let response = app.post_login(&json).await;

    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;

    let signup_body = serde_json::json!({
        "email": "test@example.com",
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let json = serde_json::json!({
        "email": "test@example.co",
        // "password": "password",
    });

    let response = app.post_login(&json).await;

    assert_eq!(response.status().as_u16(), 422);
}