use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, domain::{AuthAPIError, Email, LoginAttemptId, Password, TwoFACode}, utils::generate_auth_cookie};

#[tracing::instrument(name = "Login", skip_all)]
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
    
    // let auth_cookie= match generate_auth_cookie(&email) {
    //     Ok(auth_cookie) => auth_cookie,
    //     Err(_) => return (jar, Err(AuthAPIError::UnexpectedError)),
    // };

    let user = match user_store.get_user(&email).await {
        Ok(user) => user,
        Err(_) => return (jar, Err(AuthAPIError::IncorrectCredentials)),
    };

    // let update_jar = jar.add(auth_cookie);

    // Handle request based on user's 2FA configuration
    match user.requires_2fa {
        true => handle_2fa(&user.email, &state, jar).await,
        false => handle_no_2fa(&user.email, jar).await,
    }
}

#[tracing::instrument(name = "Handle 2FA", skip_all)]
async fn handle_2fa(
    email: &Email,
    state: &AppState,
    jar: CookieJar,
) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    let login_attempt_id = LoginAttemptId::default();
    let two_fa_code = TwoFACode::default();

    let result = state.email_client
            .read()
            .await;
            // .send_email(&email, "subject", "content");

        if let Err(e) = state
        .two_fa_code_store
        .write()
        .await
        .add_code(email.clone(), login_attempt_id.clone(), two_fa_code.clone())
        .await
        {
            return (jar, Err(AuthAPIError::UnexpectedError(e.into())));
        }
        
    // match result.send_email(email, "2FA Code", two_fa_code.as_ref()).await {
    //     Ok(_) => {},
    //     Err(err) => { return (jar, Err(AuthAPIError::UnexpectedError)) }
    // }
    if let Err(e) = result
    .send_email(email, "2FA Code", two_fa_code.as_ref())
    .await
    {
        return (jar, Err(AuthAPIError::UnexpectedError(e)));
    }

    let response = Json(LoginResponse::TwoFactorAuth(TwoFactorAuthResponse {
        message: "2FA required".to_owned(),
        login_attempt_id: login_attempt_id.as_ref().to_owned(),
    }));

    (jar, Ok((StatusCode::PARTIAL_CONTENT, response)))
    // match state.two_fa_code_store
    //         .write()
    //         .await
    //         .add_code(email.to_owned(), login_attempt_id.clone(), two_fa_code)
    //         .await {
    //     Ok(_) => return (
    //                 jar, 
    //                 Ok((StatusCode::PARTIAL_CONTENT, 
    //                     Json(LoginResponse::TwoFactorAuth(TwoFactorAuthResponse {
    //                             message: "2FA required".to_owned(), 
    //                             login_attempt_id: login_attempt_id.as_ref().to_owned() 
    //                             }
    //                     ))
    //                 ))
    //             ),
    //     Err(_) => return (jar, Err(AuthAPIError::UnexpectedError))
    // };
}

#[tracing::instrument(name = "Handle no 2FA", skip_all)]
async fn handle_no_2fa(
    email: &Email,
    jar: CookieJar,
) -> (CookieJar, Result<(StatusCode, Json<LoginResponse>), AuthAPIError>) {
    let auth_cookie= match generate_auth_cookie(email) {
        Ok(auth_cookie) => auth_cookie,
        Err(e) => return (jar, Err(AuthAPIError::UnexpectedError(e))),
    };

    let update_jar = jar.add(auth_cookie);
    
    (
        update_jar,
        Ok((StatusCode::OK, Json(LoginResponse::RegularAuth)))
    )
}

// The login route can return 2 possible success responses.
// This enum models each response!
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum LoginResponse {
    RegularAuth,
    TwoFactorAuth(TwoFactorAuthResponse),
}

// If a user requires 2FA, this JSON body should be returned!
#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorAuthResponse {
    pub message: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
}

// #[derive(Serialize, PartialEq, Debug, Deserialize)]
// pub struct LoginResponse {
//     message: String
// }

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String
}