use uuid::Uuid;

pub const SESSION_SERVER_URL: &str = "https://sessions.hytale.com";
pub const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

// com/hypixel/hytale/server/core/auth/ServerAuthManager.java
#[derive(Debug)]
pub struct ServerAuthManager {
    server_audience: String,
    bearer_token: Option<String>,
}

impl Default for ServerAuthManager {
    fn default() -> Self {
        let server_session_id = Uuid::new_v4();

        Self {
            server_audience: server_session_id.to_string(),
            bearer_token: None,
        }
    }
}

impl ServerAuthManager {
    pub fn server_audience(&self) -> &str {
        &self.server_audience
    }

    pub fn bearer_token(&self) -> Option<&str> {
        self.bearer_token.as_deref()
    }
}
