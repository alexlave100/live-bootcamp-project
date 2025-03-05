// use core::panic;

// use auth_service::{domain::Email, routes::TwoFactorAuthResponse, utils::JWT_COOKIE_NAME};
// use serde_json::error;

// use crate::helpers::TestApp;

// // #[tokio::test]
// // async fn login_returns_200_ok() {
// //     let app = TestApp::new().await;

// //     let json = serde_json::json!({
// //         "email": "email@example.com",
// //         "password": "password",
// //     });

// //     let response = app.post_login(&json).await;

// //     assert_eq!(response.status().as_u16(), 201);
// // }

// #[tokio::test]
// async fn should_return_200_if_valid_credentials_and_2fa_disabled() {
//     let app = TestApp::new().await;

//     let random_email = "testuser@email.com"; //get_random_email();

//     let signup_body = serde_json::json!({
//         "email": random_email,
//         "password": "password123",
//         "requires2FA": false
//     });

//     let response = app.post_signup(&signup_body).await;

//     assert_eq!(response.status().as_u16(), 201);

//     let login_body = serde_json::json!({
//         "email": random_email,
//         "password": "password123",
//     });

//     let response = app.post_login(&login_body).await;

//     assert_eq!(response.status().as_u16(), 200);

//     let auth_cookie = response
//         .cookies()
//         .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
//         .expect("No auth cookie found");

//     assert!(!auth_cookie.value().is_empty());

//     app.cleanup_test().await;
// }

// #[tokio::test]
// async fn should_return_401_if_incorrect_credentials() {
//     let app = TestApp::new().await;

//     let signup_body = serde_json::json!({
//         "email": "test@example.com",
//         "password": "password123",
//         "requires2FA": false
//     });

//     let response = app.post_signup(&signup_body).await;

//     assert_eq!(response.status().as_u16(), 201);

//     let json_body = serde_json::json!({
//         "email": "test@example.com",
//         "password": "password234"
//     });

//     let response = app.post_login(&json_body).await;

//     assert_eq!(response.status().as_u16(), 401);

//     app.cleanup_test().await;
// }

// #[tokio::test]
// async fn should_return_400_if_invalid_input() {
//     let app = TestApp::new().await;

//     let signup_body = serde_json::json!({
//         "email": "test@example.com",
//         "password": "password123",
//         "requires2FA": false
//     });

//     let response = app.post_signup(&signup_body).await;

//     assert_eq!(response.status().as_u16(), 201);

//     let json = serde_json::json!({
//         "email": "testexample.com",
//         "password": "password",
//     });

//     let response = app.post_login(&json).await;

//     assert_eq!(response.status().as_u16(), 400);

//     app.cleanup_test().await;
// }

// #[tokio::test]
// async fn should_return_422_if_malformed_credentials() {
//     let app = TestApp::new().await;

//     let signup_body = serde_json::json!({
//         "email": "test@example.com",
//         "password": "password123",
//         "requires2FA": false
//     });

//     let response = app.post_signup(&signup_body).await;

//     assert_eq!(response.status().as_u16(), 201);

//     let json = serde_json::json!({
//         "email": "test@example.co",
//         // "password": "password",
//     });

//     let response = app.post_login(&json).await;

//     assert_eq!(response.status().as_u16(), 422);

//     app.cleanup_test().await;
// }

// #[tokio::test]
// async fn should_return_206_if_valid_credentials_and_2fa_enabled() {
//     let app = TestApp::new().await;

//     let random_email = "testuser@example.com".to_owned();

//     let signup_body = serde_json::json!({
//         "email": random_email,
//         "password": "password123",
//         "requires2FA": true
//     });

//     let response = app.post_signup(&signup_body).await;

//     assert_eq!(response.status().as_u16(), 201);

//     let login_body = serde_json::json!({
//         "email": random_email,
//         "password": "password123"
//     });

//     let response = app.post_login(&login_body).await;

//     assert_eq!(response.status().as_u16(), 206);

//     let json_body = response
//         .json::<TwoFactorAuthResponse>()
//         .await
//         .expect("Could not deserialize response body to TwoFactorAuthResponse");

//     assert_eq!(json_body.message, "2FA required".to_owned());

//     let email_result = &Email::parse(random_email).unwrap();
//     let code = app.two_fa_code_store.read().await.get_code(email_result).await.expect("Failed to get 2FA code");

//     assert_eq!(json_body.login_attempt_id, code.0.as_ref());

//     app.cleanup_test().await;
// }