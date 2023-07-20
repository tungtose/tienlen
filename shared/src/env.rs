use dotenv::dotenv;
use log::info;

use std::env::var;

// 60 lines for only load env and without cache them? What Im I doing?
#[derive(Debug)]
pub struct Env {
    pub auth_user_name: String,
    pub auth_pass: String,
    pub webrtc_address: String,
    pub signaling_address: String,
    pub server_public_address: String,
    pub server_init_address: String,
}

impl Default for Env {
    fn default() -> Self {
        Self {
            auth_user_name: "charlie".to_string(),
            auth_pass: "12345".to_string(),
            webrtc_address: "127.0.0.1:14192".to_string(),
            signaling_address: "127.0.0.1:14191".to_string(),
            server_public_address: "http://127.0.0.1:14192".to_string(),
            server_init_address: "http://127.0.0.1:14191".to_string(),
        }
    }
}

enum Environment {
    Dev,
    Prod,
}

impl From<&str> for Environment {
    fn from(value: &str) -> Self {
        if value == "PROD" {
            return Self::Prod;
        }

        Self::Dev
    }
}

impl Env {
    pub fn new() -> Self {
        dotenv().ok();

        let environment = env!("ENVIRONMENT");

        // Seem like JAVA? holy shit why I write this?
        if let Environment::Dev = Environment::from(environment) {
            Self::default()
        } else {
            Self {
                auth_user_name: var("AUTH_USER_NAME").expect("AUTH_USER_NAME should be setted"),
                auth_pass: var("AUTH_USER_PASS").expect("AUTH_USER_PASS should be setted"),
                server_public_address: var("SERVER_ADDRESS")
                    .expect("SERVER_ADDRESS should be setted"),
                webrtc_address: var("SERVER_WEBRTC_ADDRESS")
                    .expect("SERVER_WEBRTC_ADDRESS should be setted"),
                signaling_address: var("SERVER_SIGNALING_ADDRESS")
                    .expect("SERVER_SIGNALING_ADDRESS should be setted"),
                server_init_address: var("SERVER_INIT_ADDRESS")
                    .expect("SERVER_SIGNALING_ADDRESS should be setted"),
            }
        }
    }
}
