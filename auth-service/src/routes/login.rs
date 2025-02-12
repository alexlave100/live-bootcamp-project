use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{app_state::AppState, domain::{AuthAPIError, Email, LoginAttemptId, Password, TwoFACode}, utils::generate_auth_cookie};

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

// New!
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
            
    /*
    That's excellent! Adding await and having the error disappear confirms that the issue was indeed related to asynchronous code and futures.  This means that the function you were working with was likely intended to be an async function, or it was calling another async function that needed to be awaited.

    Here's a recap of why adding await solved the problem and what it means:

    Why await Solved It:

    The await keyword is used in asynchronous functions (async fn) to pause the execution of the function until the future it's waiting on completes.  In your case, you were likely calling an asynchronous function (e.g., state.email_client.read().await.send_email(...)) that returned a future.  Without await, you were trying to use the future itself as a value, which is what caused the type mismatch error.

    By adding await, you're telling Rust to:

    Execute the asynchronous operation (e.g., state.email_client.read().await.send_email(...)).
    Pause the current function's execution until the asynchronous operation completes and produces its result (the Result).
    Once the asynchronous operation is done, get the Result value and continue with the rest of your code.
    What It Means:

    The fact that adding await fixed the error strongly suggests that the code you were working with was part of an asynchronous context.  This is very common when dealing with I/O operations (like network requests, file access, or interactions with databases), as these operations are often non-blocking and handled asynchronously.

    Example (Illustrative):

    Rust
    async fn my_async_function() -> Result<(), String> {
        let result: Result<i32, String> = some_async_function().await; // Await the future

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    async fn some_async_function() -> Result<i32, String> {
        // ... some asynchronous operation ...
        Ok(42) // Or Err(...) if it fails
    }
    In this example, some_async_function() returns a future.  Inside my_async_function(), we use .await to wait for that future to complete and get the Result<i32, String> value.

    Key Takeaway:

    Whenever you're working with asynchronous code in Rust, remember that asynchronous functions return futures, not the actual results.  You need to use await to wait for these futures to complete and get the values they produce.  This is a fundamental concept in Rust's asynchronous programming model.  If you encounter similar errors in the future, double-check that you're correctly using await when necessary.
    */
    match result.send_email(email, "2FA Code", two_fa_code.as_ref()).await {
        Ok(_) => {},
        Err(_) => { return (jar, Err(AuthAPIError::UnexpectedError)) }
    }

    match state.two_fa_code_store
            .write()
            .await
            .add_code(email.to_owned(), login_attempt_id.clone(), two_fa_code)
            .await {
        Ok(_) => return (
                    jar, 
                    Ok((StatusCode::PARTIAL_CONTENT, 
                        Json(LoginResponse::TwoFactorAuth(TwoFactorAuthResponse {
                                message: "2FA required".to_owned(), 
                                login_attempt_id: login_attempt_id.as_ref().to_owned() 
                                }
                        ))
                    ))
                ),
        Err(_) => return (jar, Err(AuthAPIError::UnexpectedError))
    };
}

// New!
async fn handle_no_2fa(
    email: &Email,
    jar: CookieJar,
) -> (CookieJar, Result<(StatusCode, Json<LoginResponse>), AuthAPIError>) {
    let auth_cookie= match generate_auth_cookie(email) {
        Ok(auth_cookie) => auth_cookie,
        Err(_) => return (jar, Err(AuthAPIError::UnexpectedError)),
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