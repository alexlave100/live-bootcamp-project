// use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
// use axum_extra::extract::CookieJar;
// use serde::Deserialize;

// use crate::{app_state::AppState, domain::{AuthAPIError, Email, LoginAttemptId, TwoFACode}, utils::generate_auth_cookie};

// pub async fn verify_2fa(
//     State(state): State<AppState>,
//     jar: CookieJar,
//     Json(request): Json<Verify2FARequest>,
// ) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
//      let email = match Email::parse(request.email) {
//         Ok(email) => email,
//         Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
//     };

//     let login_attempt_id = match LoginAttemptId::parse(request.login_attempt_id) {
//         Ok(login_attempt_id) => login_attempt_id,
//         Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
//     }; // Validate the login attempt ID in `request`

//     let two_fa_code = match TwoFACode::parse(request.two_fa_code) {
//         Ok(two_fa_code) => two_fa_code,
//         Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
//     };

//     let mut two_fa_code_store = state.two_fa_code_store.write().await;

//     // Call `two_fa_code_store.get_code`. If the call fails
//     // return a `AuthAPIError::IncorrectCredentials`.
//     let code_tuple = match two_fa_code_store.get_code(&email).await {
//         Ok((login_attempt_id, two_fa_code)) => (login_attempt_id, two_fa_code),
//         Err(_) => return (jar, Err(AuthAPIError::IncorrectCredentials)),
//     };

//     // TODO: Validate that the `login_attempt_id` and `two_fa_code`
//     // in the request body matches values in the `code_tuple`. 
//     // If not, return a `AuthAPIError::IncorrectCredentials`.
//     if code_tuple != (login_attempt_id, two_fa_code) {
//         return (jar, Err(AuthAPIError::IncorrectCredentials));
//     }

//     if two_fa_code_store.remove_code(&email).await.is_err() {
//         return (jar, Err(AuthAPIError::UnexpectedError));
//     }

//     let cookie = match generate_auth_cookie(&email) {
//         Ok(cookie) => cookie,
//         Err(_) => return (jar, Err(AuthAPIError::UnexpectedError)),
//     };

//     let updated_jar = jar.add(cookie);

//      // Validate the 2FA code in `request`
//     (updated_jar, Ok(StatusCode::OK.into_response()))
// }

// // TODO: implement the Verify2FARequest struct. See the verify-2fa route contract in step 1 for the expected JSON body.
// #[derive(Debug, Deserialize)]
// pub struct Verify2FARequest {
//     email: String,
//     #[serde(rename = "loginAttemptId")]
//     login_attempt_id: String,
//     #[serde(rename = "2FACode")]
//     two_fa_code: String,
// }