use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use crate::{domain::AuthAPIError, utils::validate_token};

pub async fn verify_token(Json(auth_token): Json<TokenToBeVerified>) -> Result<impl IntoResponse, AuthAPIError> {
    // auth_token.token
    let token = match validate_token(&auth_token.token).await {
        Ok(result) => result,
        Err(_) => return Err(AuthAPIError::InvalidToken),
    };

    Ok(StatusCode::OK)
}

#[derive(Deserialize)]
pub struct TokenToBeVerified {
    pub token: String
}
