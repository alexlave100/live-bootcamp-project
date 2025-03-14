use auth_service::{domain::ErrorResponse, utils::JWT_COOKIE_NAME};
use reqwest::Url;

use crate::helpers::TestApp;

#[tokio::test]
async fn should_return_200_if_valid_jwt_cookie() {
    let app = TestApp::new().await;

    let random_email = "email@example.com";

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

    let token = auth_cookie.value();

    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(auth_cookie.value().is_empty());

    let banned_token_store = app.banned_token_store.read().await;
    let contains_token = banned_token_store
        .token_exists(token)
        .await
        .expect("Failed to check if token is banned");

    assert!(contains_token);

    app.cleanup_test().await;
}

// #[tokio::test]
// async fn should_return_400_if_logout_called_twice_in_a_row() {
//     let app = TestApp::new().await;

//     let random_email = "email@example.com";

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

//     let response = app.post_logout().await;
//     assert_eq!(response.status().as_u16(), 200);

//     let auth_cookie = response
//         .cookies()
//         .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
//         .expect("No auth cookie found");

//     assert!(auth_cookie.value().is_empty());

//     let response = app.post_logout().await;
//     assert_eq!(response.status().as_u16(), 400);

//     assert_eq!(
//         response
//             .json::<ErrorResponse>()
//             .await
//             .expect("Could not deserialize response body to ErrorResponse")
//             .error,
//         "Missing auth token".to_owned()
//     );

//     app.cleanup_test().await;
// }

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let app = TestApp::new().await;

    let logout_response = app.post_logout().await;

    assert_eq!(logout_response.status().as_u16(), 400);
    
    app.cleanup_test().await;
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    // add invalid cookie
    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/", 
            JWT_COOKIE_NAME
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let logout_response = app.post_logout().await;

    assert_eq!(logout_response.status().as_u16(), 401);
    
    app.cleanup_test().await;
}