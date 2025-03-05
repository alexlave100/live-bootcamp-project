use std::error::Error;

use app_state::AppState;
use axum::{http::{Method, StatusCode}, response::{IntoResponse, Response}, routing::post, serve::Serve, Json, Router};
use redis::{Client, RedisResult};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};
use utils::{make_span_with_request_id, on_request, on_response};

pub mod routes;
pub mod domain;
pub mod services;
pub mod app_state;
pub mod utils;

// This struct encapsulates our application-related logic.
pub struct Application {
    server: Serve<Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        // Allow the app service(running on our local machine and in production) to call the auth service
        let allowed_origins = [
            "http://localhost:8000".parse()?,
            "http://147.182.214.12:8000".parse()?,
        ];

        let cors = CorsLayer::new()
            // Allow GET and POST requests
            .allow_methods([Method::GET, Method::POST])
            // Allow cookies to be included in requests
            .allow_credentials(true)
            .allow_origin(allowed_origins);

        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(routes::signup))
            // .route("/login", post(routes::login))
            // .route("/logout", post(routes::logout))
            // .route("/verify-2fa", post(routes::verify_2fa))
            .route("/verify-token", post(routes::verify_token))
            .with_state(app_state)
            .layer(cors)
            .layer(
                // Add a TraceLayer for HTTP requests to enable detailed tracing
                // This layer will create spans for each request using the make_span_with_request_id function,
                // and log events at the start and end of each request using on_request and on_response functions.
                TraceLayer::new_for_http()
                    .make_span_with(make_span_with_request_id)
                    .on_request(on_request)
                    .on_response(on_response),
            ); // Add CORS config to our Axum router

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        Ok(Application {
            address,
            server
        })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        tracing::info!("listening on {}", &self.address);
        self.server.await
    }
}

pub async fn get_postgres_pool(url: &str) -> Result<PgPool, sqlx::Error> {
    // Create a new PostgreSQL connection pool
    PgPoolOptions::new().max_connections(5).connect(url).await
}

pub fn get_redis_client(redis_hostname: String) -> RedisResult<Client> {
    let redis_url = format!("redis://{}/", redis_hostname);
    redis::Client::open(redis_url)
}

// #[derive(Serialize, Deserialize)]
// pub struct ErrorResponse {
//     pub error: String,
// }

// impl IntoResponse for AuthAPIError {
//     fn into_response(self) -> Response {
//         let (status, error_message) = match self {
//             AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
//             AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
//             AuthAPIError::IncorrectCredentials => (StatusCode::UNAUTHORIZED, "Incorrect credentials"),
//             AuthAPIError::UnexpectedError => {
//                 (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
//             },
//             AuthAPIError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
//             AuthAPIError::MissingToken => (StatusCode::UNAUTHORIZED, "Missing token")
//         };
//         let body = Json(ErrorResponse {
//             error: error_message.to_string(),
//         });
//         (status, body).into_response()
//     }
// }
