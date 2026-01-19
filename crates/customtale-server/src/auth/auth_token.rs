// com/hypixel/hytale/server/core/auth/ServerAuthManager.java

#[derive(Debug, Clone)]
pub struct AuthConfig {
    pub server_audience: String,
    pub server_session_token: String,
}
