use miette::Diagnostic;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::auth::{SESSION_SERVER_URL, ServerAuthManager, USER_AGENT};

#[derive(Serialize)]
struct RequestBody<'a> {
    #[serde(rename = "identityToken")]
    identity_token: &'a str,
    #[serde(rename = "aud")]
    server_audience: &'a str,
}

#[derive(Deserialize)]
struct ResponseBody {
    #[serde(rename = "authorizationGrant")]
    authorization_grant: String,
}

#[derive(Debug, Error, Diagnostic)]
pub enum RequestAuthorizationError {
    #[error("failed to connect to authorization server")]
    Connect(#[source] Box<dyn std::error::Error + 'static>),
    #[error("failed to decode authorization server response")]
    Body(#[source] Box<dyn std::error::Error + 'static>),
    #[error("server responded with non-200 status {status}: {body}")]
    Status {
        status: surf::StatusCode,
        body: String,
    },
    #[error("server has not yet loaded authentication token")]
    NoAuthToken,
}

// com/hypixel/hytale/server/core/auth/SessionServiceClient.java
impl ServerAuthManager {
    pub async fn request_authorization_grant(
        &self,
        identity_token: &str,
    ) -> Result<String, RequestAuthorizationError> {
        let server_audience = self.server_audience();
        let bearer_token = self
            .bearer_token()
            .ok_or(RequestAuthorizationError::NoAuthToken)?;

        let mut resp = surf::post(format!("{SESSION_SERVER_URL}/server-join/auth-grant"))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("Authorization", format!("Bearer {bearer_token}"))
            .header("User-Agent", USER_AGENT)
            .body(
                surf::Body::from_json(&RequestBody {
                    identity_token,
                    server_audience,
                })
                .unwrap(),
            )
            .send()
            .await
            .map_err(|err| RequestAuthorizationError::Connect(err.into()))?;

        if resp.status() != surf::StatusCode::Ok {
            let (Ok(body) | Err(body)) = resp.body_string().await.map_err(|v| v.to_string());

            return Err(RequestAuthorizationError::Status {
                status: resp.status(),
                body,
            });
        }

        let body = resp
            .body_json::<ResponseBody>()
            .await
            .map_err(|err| RequestAuthorizationError::Body(err.into()))?;

        Ok(body.authorization_grant)
    }
}
