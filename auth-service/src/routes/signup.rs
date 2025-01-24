use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, domain::User};

use crate::domain::{AuthAPIError, Email, Password};

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> impl IntoResponse {
    // early return AuthAPIError::InvalidCredentials if:
    // - email is empty or does not contain '@'
    // - password is less than 8 characters
    let email =
        Email::parse(request.email.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password =
        Password::parse(request.password.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;

    // Create a new `User` instance using data in the `request`
    let user = User {
        email,
        password,
        requires_2fa: request.requires_2fa
    };

    let mut user_store = state.user_store.write().await;

    // early return AuthAPIError::UserAlreadyExists if email exists in user_store.
    if user_store.get_user(&user.email).await.is_ok() {
        return Err(AuthAPIError::UserAlreadyExists);
    }

    // instead of using unwrap, early return AuthAPIError::UnexpectedError if add_user() fails.
    if user_store.add_user(user).await.is_err() {
        return Err(AuthAPIError::UnexpectedError);
    }

    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    Ok((StatusCode::CREATED, response))
}

#[derive(Serialize, PartialEq, Debug, Deserialize)]
pub struct SignupResponse {
    pub message: String,
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}