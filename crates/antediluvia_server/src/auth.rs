//! Simple in-memory auth/token service (placeholder).

use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use rand::Rng;
use axum::{Router, routing::post, Json, extract::State};
use serde::{Serialize, Deserialize};
use anyhow::Result;

#[derive(Clone, Default)]
pub struct AuthService {
    tokens: Arc<Mutex<HashSet<String>>>,
}

impl AuthService {
    pub fn new() -> Self {
        Self::default()
    }

    /// Issue a new token for a player_id (placeholder, no password validation).
    pub fn issue_token(&self, _player_id: u64) -> String {
        let mut bytes = [0u8; 16];
        rand::rng().fill(&mut bytes);
        let token = hex::encode(bytes);
        if let Ok(mut set) = self.tokens.lock() {
            set.insert(token.clone());
        }
        token
    }

    /// Validate a token.
    #[allow(dead_code)]
    pub fn validate(&self, token: &str) -> bool {
        self.tokens
            .lock()
            .map(|set| set.contains(token))
            .unwrap_or(false)
    }
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    #[allow(dead_code)]
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
    player_id: u64,
}

/// Start a minimal auth HTTP server on the given port.
pub async fn start_auth_server(auth: AuthService, port: u16) -> Result<()> {
    async fn login_handler(
        State(auth): State<AuthService>,
        Json(payload): Json<LoginRequest>,
    ) -> Json<LoginResponse> {
        let player_id = hash_username(&payload.username);
        let token = auth.issue_token(player_id);
        Json(LoginResponse { token, player_id })
    }

    let app = Router::new()
        .route("/auth/login", post(login_handler))
        .with_state(auth);

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port)).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

fn hash_username(name: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    name.hash(&mut h);
    h.finish()
}
