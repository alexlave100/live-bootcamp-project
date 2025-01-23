use auth_service::{routes::SignupResponse, ErrorResponse};

use crate::helpers::TestApp;

#[tokio::test]
async fn should_return_201_if_valid_input() {
    let app = TestApp::new().await;

    let json = serde_json::json!({
        "email": "email@example.com",
        "password": "password",
        "requires2FA": false
    });

    let response = app.post_signup(&json).await;

    assert_eq!(response.status().as_u16(), 201);

    let expected_response = SignupResponse {
        message: "User created successfully!".to_owned(),
    };

    // Assert that we are getting the correct response body!
    assert_eq!(
        response
            .json::<SignupResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
        expected_response
    );
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

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    // The signup route should return a 400 HTTP status code if an invalid input is sent.
    // The input is considered invalid if:
    // - The email is empty or does not contain '@'
    // - The password is less than 8 characters

    // Create an array of invalid inputs. Then, iterate through the array and 
    // make HTTP calls to the signup route. Assert a 400 HTTP status code is returned.
    let app = TestApp::new().await;

    let input = [serde_json::json!({
        "email": "",
        "password": "password",
        "requires2FA": false
    })];

    for i in input.iter() {
        let response = app.post_signup(i).await;
        assert_eq!(response.status().as_u16(), 400, "Failed for input: {:?}", i);

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
async fn should_return_409_if_email_already_exists() {
    // Call the signup route twice. The second request should fail with a 409 HTTP status code    

    let app = TestApp::new().await;

    let json = serde_json::json!({
        "email": "email@example.com",
        "password": "password",
        "requires2FA": false
    });

    app.post_signup(&json).await;
    let response = app.post_signup(&json).await;

    assert_eq!(response.status().as_u16(), 409);

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "User already exists".to_owned()
    );
}