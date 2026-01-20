use miette::Diagnostic;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

// com/hypixel/hytale/server/core/auth/SessionServiceClient.java

const SESSION_SERVER_URL: &str = "https://sessions.hytale.com";
const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[derive(Debug, Error, Diagnostic)]
pub enum SessionServiceError {
    #[error("failed to connect to session server")]
    Connect(#[source] Box<dyn std::error::Error + 'static>),
    #[error("failed to decode session server response")]
    Body(#[source] Box<dyn std::error::Error + 'static>),
    #[error("session server responded with non-200 status {status}: {body}")]
    Status {
        status: surf::StatusCode,
        body: String,
    },
}

pub async fn request_authorization_grant(
    identity_token: &str,
    server_audience: &str,
    bearer_token: &str,
) -> Result<String, SessionServiceError> {
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
        .map_err(|err| SessionServiceError::Connect(err.into()))?;

    if resp.status() != surf::StatusCode::Ok {
        let (Ok(body) | Err(body)) = resp.body_string().await.map_err(|v| v.to_string());

        return Err(SessionServiceError::Status {
            status: resp.status(),
            body,
        });
    }

    let body = resp
        .body_json::<ResponseBody>()
        .await
        .map_err(|err| SessionServiceError::Body(err.into()))?;

    Ok(body.authorization_grant)
}

pub async fn exchange_auth_grant_for_token(
    authorization_grant: &str,
    x509_fingerprint: &str,
    bearer_token: &str,
) -> Result<String, SessionServiceError> {
    #[derive(Serialize)]
    struct RequestBody<'a> {
        #[serde(rename = "authorizationGrant")]
        authorization_grant: &'a str,
        #[serde(rename = "x509Fingerprint")]
        x509_fingerprint: &'a str,
    }

    #[derive(Deserialize)]
    struct ResponseBody {
        #[serde(rename = "accessToken")]
        access_token: String,
    }

    let mut resp = surf::post(format!("{SESSION_SERVER_URL}/server-join/auth-token"))
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {bearer_token}"))
        .header("User-Agent", USER_AGENT)
        .body(
            surf::Body::from_json(&RequestBody {
                authorization_grant,
                x509_fingerprint,
            })
            .unwrap(),
        )
        .send()
        .await
        .map_err(|err| SessionServiceError::Connect(err.into()))?;

    if resp.status() != surf::StatusCode::Ok {
        let (Ok(body) | Err(body)) = resp.body_string().await.map_err(|v| v.to_string());

        return Err(SessionServiceError::Status {
            status: resp.status(),
            body,
        });
    }

    let body = resp
        .body_json::<ResponseBody>()
        .await
        .map_err(|err| SessionServiceError::Body(err.into()))?;

    Ok(body.access_token)
}

pub async fn get_jwks() -> Result<jsonwebtoken::jwk::Jwk, SessionServiceError> {
    let mut resp = surf::get(format!("{SESSION_SERVER_URL}/.well-known/jwks.json"))
        .header("Accept", "application/json")
        .header("User-Agent", USER_AGENT)
        .send()
        .await
        .map_err(|err| SessionServiceError::Connect(err.into()))?;

    if resp.status() != surf::StatusCode::Ok {
        let (Ok(body) | Err(body)) = resp.body_string().await.map_err(|v| v.to_string());

        return Err(SessionServiceError::Status {
            status: resp.status(),
            body,
        });
    }

    let body = resp
        .body_json::<jsonwebtoken::jwk::Jwk>()
        .await
        .map_err(|err| SessionServiceError::Body(err.into()))?;

    Ok(body)
}

#[derive(Debug, Clone, Deserialize)]
pub struct GameProfile {
    pub uuid: Uuid,
    pub username: String,
}

pub async fn get_game_profiles(
    oauth_access_token: &str,
) -> Result<Vec<GameProfile>, SessionServiceError> {
    #[derive(Deserialize)]
    struct LauncherDataResponse {
        #[serde(rename = "owner")]
        _owner: Uuid,
        profiles: Vec<GameProfile>,
    }

    let mut resp = surf::get("https://account-data.hytale.com/my-account/get-profiles")
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {oauth_access_token}"))
        .header("User-Agent", USER_AGENT)
        .send()
        .await
        .map_err(|err| SessionServiceError::Connect(err.into()))?;

    if resp.status() != surf::StatusCode::Ok {
        let (Ok(body) | Err(body)) = resp.body_string().await.map_err(|v| v.to_string());

        return Err(SessionServiceError::Status {
            status: resp.status(),
            body,
        });
    }

    let body = resp
        .body_json::<LauncherDataResponse>()
        .await
        .map_err(|err| SessionServiceError::Body(err.into()))?;

    Ok(body.profiles)
}

#[derive(Debug, Clone, Deserialize)]
pub struct GameSessionResponse {
    #[serde(rename = "sessionToken")]
    pub session_token: String,
    #[serde(rename = "identityToken")]
    pub identity_token: String,
    #[serde(rename = "expiresAt")]
    pub expires_at: String,
}

pub async fn create_game_session(
    oauth_access_token: &str,
    profile_uuid: Uuid,
) -> Result<GameSessionResponse, SessionServiceError> {
    #[derive(Serialize)]
    struct RequestBody {
        uuid: Uuid,
    }

    let mut resp = surf::post(format!("{SESSION_SERVER_URL}/game-session/new"))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {oauth_access_token}"))
        .header("User-Agent", USER_AGENT)
        .body(surf::Body::from_json(&RequestBody { uuid: profile_uuid }).unwrap())
        .send()
        .await
        .map_err(|err| SessionServiceError::Connect(err.into()))?;

    if resp.status() != surf::StatusCode::Ok {
        let (Ok(body) | Err(body)) = resp.body_string().await.map_err(|v| v.to_string());

        return Err(SessionServiceError::Status {
            status: resp.status(),
            body,
        });
    }

    let body = resp
        .body_json::<GameSessionResponse>()
        .await
        .map_err(|err| SessionServiceError::Body(err.into()))?;

    Ok(body)
}

pub async fn refresh_session(
    session_token: &str,
) -> Result<GameSessionResponse, SessionServiceError> {
    let mut resp = surf::post(format!("{SESSION_SERVER_URL}/game-session/new"))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {session_token}"))
        .header("User-Agent", USER_AGENT)
        .send()
        .await
        .map_err(|err| SessionServiceError::Connect(err.into()))?;

    if resp.status() != surf::StatusCode::Ok {
        let (Ok(body) | Err(body)) = resp.body_string().await.map_err(|v| v.to_string());

        return Err(SessionServiceError::Status {
            status: resp.status(),
            body,
        });
    }

    let body = resp
        .body_json::<GameSessionResponse>()
        .await
        .map_err(|err| SessionServiceError::Body(err.into()))?;

    Ok(body)
}

pub async fn terminate_session(session_token: &str) -> Result<(), SessionServiceError> {
    let mut resp = surf::delete(format!("{SESSION_SERVER_URL}/game-session"))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {session_token}"))
        .header("User-Agent", USER_AGENT)
        .send()
        .await
        .map_err(|err| SessionServiceError::Connect(err.into()))?;

    if resp.status() != surf::StatusCode::Ok {
        let (Ok(body) | Err(body)) = resp.body_string().await.map_err(|v| v.to_string());

        return Err(SessionServiceError::Status {
            status: resp.status(),
            body,
        });
    }

    Ok(())
}
