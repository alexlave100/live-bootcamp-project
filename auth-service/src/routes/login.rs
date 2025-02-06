use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};
use crate::{app_state::AppState, domain::{AuthAPIError, Email, Password}, utils::generate_auth_cookie};

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    
    let user_store = &state.user_store.read().await;
    
    let password = match Password::parse(request.password) {
        Ok(password) => password,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };
    
    let email = match Email::parse(request.email) {
        Ok(email) => email,
        _ => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };
    
    let user_validated = user_store.validate_user(&email, &password).await.is_ok();
    if !user_validated {
        return (jar, Err(AuthAPIError::IncorrectCredentials));
    }
    
    let auth_cookie= match generate_auth_cookie(&email) {
        Ok(auth_cookie) => auth_cookie,
        Err(_) => return (jar, Err(AuthAPIError::UnexpectedError)),
    };

    let update_jar = jar.add(auth_cookie);
    
    (update_jar, Ok(StatusCode::OK.into_response()))
}

#[derive(Serialize, PartialEq, Debug, Deserialize)]
pub struct LoginResponse {
    message: String
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String
}