use axum::{response::IntoResponse, http::StatusCode};
use axum_extra::extract::{CookieJar};

use crate::{domain::AuthAPIError, utils::{validate_token, JWT_COOKIE_NAME}};

pub async fn logout(jar: CookieJar) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    // Retrieve JWT cookie from the `CookieJar`
    // Return AuthAPIError::MissingToken is the cookie is not found
    let cookie = match jar.get(JWT_COOKIE_NAME) {
        Some(value) => value,
        None => return (jar, Err(AuthAPIError::MissingToken))
    };

    let token = cookie.value().to_owned();

    // TODO: Validate JWT token by calling `validate_token` from the auth service.
    // If the token is valid you can ignore the returned claims for now.
    // Return AuthAPIError::InvalidToken is validation fails.
    match validate_token(&token).await {
        Ok(claims) => claims,
        Err(_) => return (jar, Err(AuthAPIError::InvalidToken))
    };

    let jar = jar.remove(JWT_COOKIE_NAME);
    
    (jar, Ok(StatusCode::OK))
}