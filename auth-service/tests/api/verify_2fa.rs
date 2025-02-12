use auth_service::{domain::{Email, ErrorResponse}, routes::TwoFactorAuthResponse, utils::JWT_COOKIE_NAME};

use crate::helpers::TestApp;

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let email = "testuser@example.com".to_owned();
    
    let verify_two_fa_body_test_cases =[ 
        serde_json::json!({
        "email": email,
        "loginAttemptId": "asd",
        }),
        serde_json::json!({
            "email": email
        }),
        serde_json::json!({
        }),
        serde_json::json!({
            "email": "testemail@email.com",
            "loginAttemptId": "0",
        }),
        serde_json::json!({
            "loginAttemptId": 0,
        }),
        serde_json::json!({
            "loginAttemptId": 0,
            "2FACode": "123",
        }),
    ];

    for test_case in verify_two_fa_body_test_cases {
        let two_fa_response =  app.post_verify_2fa(&test_case).await;
    
        assert_eq!(two_fa_response.status(), 422);
    }
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    let invalid_input_test_cases = [
        serde_json::json!({
            "email": "email@email.com",
            "loginAttemptId": "invalid_attempt_id",
            "2FACode": "two_fa_code",
        }),
        serde_json::json!({
            "email": "emailemail.com",
            "loginAttemptId": "1234567890",
            "2FACode": "123456",
        })
    ];

    for test_input in invalid_input_test_cases {
        let response = app.post_verify_2fa(&test_input).await;
        assert_eq!(response.status(), 400);

        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "Invalid credentials".to_owned()
        );
    }
}

#[tokio::test]
async fn should_return_200_if_correct_code() {
    let app = TestApp::new().await;

    let random_email = "testuser@example.com".to_owned();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123"
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 206);

    let response_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");

    assert_eq!(response_body.message, "2FA required".to_owned());
    assert!(!response_body.login_attempt_id.is_empty());

    let login_attempt_id = response_body.login_attempt_id;

    let code_tuple = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&Email::parse(random_email.clone()).unwrap())
        .await
        .unwrap();

    let code = code_tuple.1.as_ref();

    let request_body = serde_json::json!({
        "email": random_email,
        "loginAttemptId": login_attempt_id,
        "2FACode": code
    });

    let response = app.post_verify_2fa(&request_body).await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
}

#[tokio::test]
async fn should_return_401_if_same_code_twice() {
    let app = TestApp::new().await;

    let random_email = "testuser@example.com".to_owned();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123"
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 206);

    let response_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");

    assert_eq!(response_body.message, "2FA required".to_owned());
    assert!(!response_body.login_attempt_id.is_empty());

    let login_attempt_id = response_body.login_attempt_id;

    let code_tuple = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&Email::parse(random_email.clone()).unwrap())
        .await
        .unwrap();

    let code = code_tuple.1.as_ref();

    let request_body = serde_json::json!({
        "email": random_email,
        "loginAttemptId": login_attempt_id,
        "2FACode": code
    });

    let response = app.post_verify_2fa(&request_body).await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());

    let response = app.post_verify_2fa(&request_body).await;

    assert_eq!(response.status().as_u16(), 401);
}