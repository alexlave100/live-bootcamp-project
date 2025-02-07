use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use crate::{app_state::AppState, domain::AuthAPIError, utils::validate_token};

pub async fn verify_token(
        State(state): State<AppState>,
        Json(auth_token): Json<TokenToBeVerified>
    ) -> Result<impl IntoResponse, AuthAPIError> {
    match validate_token(&auth_token.token, state.banned_token_store.clone()).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => return Err(AuthAPIError::InvalidToken),
    }
}

#[derive(Deserialize)]
pub struct TokenToBeVerified {
    pub token: String
}
