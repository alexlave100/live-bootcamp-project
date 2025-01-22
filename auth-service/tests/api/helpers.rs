use std::sync::Arc;

use auth_service::{
    app_state::AppState, services::hashmap_user_store::HashmapUserStore, Application,
};
use reqwest::Client;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
        let app_state = AppState::new(user_store);
        
        let app = Application::build(app_state, "127.0.0.1:0")
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address.clone());

        // Run the auth service in a separate async task
        // to avoid blocking the main test thread. 
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let http_client =  Client::new(); 
        
        // Create new `TestApp` instance and return it
        TestApp {
            address,
            http_client
        }
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_signup<Body>(&self, body: &Body) -> reqwest::Response where Body: serde::Serialize {
        self.http_client
            .post(&format!("{}/signup", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub fn get_random_email() -> String {
        format!("{}@example.com", Uuid::new_v4())
    }

    pub async fn post_login<Body>(&self, body: &Body) -> reqwest::Response where Body: serde::Serialize {
        self.http_client
            .post(&format!("{}/login", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_logout(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/logout", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_verify_2fa(&self, code: &str) -> reqwest::Response {
        let json = serde_json::json!({
            "code": code,
        });

        self.http_client
            .post(&format!("{}/verify-2fa", &self.address))
            .json(&json)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_verifytoken(&self, token: &str) -> reqwest::Response {
        let json = serde_json::json!({
            "token": token,
        });

        self.http_client
            .post(&format!("{}/verify-2fa", &self.address))
            .json(&json)
            .send()
            .await
            .expect("Failed to execute request.")
    }
}