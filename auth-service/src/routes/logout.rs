use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::{CookieJar};

use color_eyre::eyre::Error;

use crate::{app_state::AppState, domain::AuthAPIError, utils::{validate_token, JWT_COOKIE_NAME}};

#[tracing::instrument(name = "Logout", skip_all)]
pub async fn logout(State(state): State<AppState>, jar: CookieJar) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
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
    match validate_token(&token, state.banned_token_store.clone()).await {
        Ok(claims) => claims,
        Err(_) => return (jar, Err(AuthAPIError::InvalidToken))
    };

        // Add token to banned list
        if let Err(e) = state
        .banned_token_store
        .write()
        .await
        .store_token(token.to_owned())
        .await
    {
       return (jar, Err(AuthAPIError::UnexpectedError(e.into())));
    }

    let jar = jar.remove(JWT_COOKIE_NAME);

    (jar, Ok(StatusCode::OK))
}