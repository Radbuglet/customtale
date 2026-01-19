use customtale_protocol::serde::Uuid;

#[derive(Debug, Clone)]
pub struct IdentityTokenClaims {
    pub issuer: String,
    pub subject: Uuid,
    pub username: String,
    pub issued_at: u64,
    pub expires_at: u64,
    pub not_before: u64,
    pub scope: String,
}

impl IdentityTokenClaims {
    // com/hypixel/hytale/server/core/auth/JWTValidator.java
    pub fn validate(identity_token: &str) -> Self {
        todo!()
    }
}
