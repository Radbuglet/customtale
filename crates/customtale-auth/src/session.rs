use base64::Engine as _;
use miette::Diagnostic;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

// com/hypixel/hytale/server/core/auth/SessionServiceClient.java

pub const SESSION_SERVER_URL: &str = "https://sessions.hytale.com";
pub const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub const OAUTH_REDIRECT_URI: &str = "https://accounts.hytale.com/consent/client";
pub const OAUTH_CLIENT_ID: &str = "hytale-server";
pub const OAUTH_SCOPES: &str = "openid offline auth:server";

#[derive(Debug, Error, Diagnostic)]
pub enum SessionServiceError {
    #[error("failed to initialize session server service")]
    Init(#[source] reqwest::Error),
    #[error("failed to connect to session server")]
    Connect(#[source] reqwest::Error),
    #[error("failed to decode session server response")]
    Body(#[source] reqwest::Error),
    #[error("session server responded with non-200 status {status}: {body}")]
    Status {
        status: reqwest::StatusCode,
        body: String,
    },
}

#[derive(Debug, Clone, Deserialize)]
pub struct GameProfile {
    pub uuid: Uuid,
    pub username: String,
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

#[derive(Debug, Clone, Deserialize)]
pub struct OAuthTokenResponse {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub id_token: Option<String>,
    pub error: Option<String>,
    #[serde(default)]
    pub expires_in: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OAuthDeviceResponse {
    pub device_code: Option<String>,
    pub user_code: Option<String>,
    pub verification_uri: Option<String>,
    pub verification_uri_complete: Option<String>,
    #[serde(default)]
    pub expires_in: OAuthDeviceExpiresIn,
    #[serde(default)]
    pub interval: OAuthDeviceInterval,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OAuthDeviceExpiresIn(pub u32);

impl Default for OAuthDeviceExpiresIn {
    fn default() -> Self {
        Self(600)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct OAuthDeviceInterval(pub u32);

impl Default for OAuthDeviceInterval {
    fn default() -> Self {
        Self(5)
    }
}

// === SessionService === //

#[derive(Debug, Clone)]
pub struct SessionService {
    client: reqwest::Client,
}

impl SessionService {
    pub fn new() -> Result<Self, SessionServiceError> {
        Ok(Self {
            client: reqwest::ClientBuilder::new()
                .build()
                .map_err(SessionServiceError::Init)?,
        })
    }

    pub async fn request_authorization_grant(
        &self,
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

        let resp = self
            .client
            .post(format!("{SESSION_SERVER_URL}/server-join/auth-grant"))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("Authorization", format!("Bearer {bearer_token}"))
            .header("User-Agent", USER_AGENT)
            .json(&RequestBody {
                identity_token,
                server_audience,
            })
            .send()
            .await
            .map_err(SessionServiceError::Connect)?;

        let resp = filter_status(resp).await?;

        let body = resp
            .json::<ResponseBody>()
            .await
            .map_err(SessionServiceError::Body)?;

        Ok(body.authorization_grant)
    }

    pub async fn exchange_auth_grant_for_token(
        &self,
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

        let resp = self
            .client
            .post(format!("{SESSION_SERVER_URL}/server-join/auth-token"))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("Authorization", format!("Bearer {bearer_token}"))
            .header("User-Agent", USER_AGENT)
            .json(&RequestBody {
                authorization_grant,
                x509_fingerprint,
            })
            .send()
            .await
            .map_err(SessionServiceError::Connect)?;

        let resp = filter_status(resp).await?;

        let body = resp
            .json::<ResponseBody>()
            .await
            .map_err(SessionServiceError::Body)?;

        Ok(body.access_token)
    }

    pub async fn get_jwks(&self) -> Result<jsonwebtoken::jwk::Jwk, SessionServiceError> {
        let resp = self
            .client
            .get(format!("{SESSION_SERVER_URL}/.well-known/jwks.json"))
            .header("Accept", "application/json")
            .header("User-Agent", USER_AGENT)
            .send()
            .await
            .map_err(SessionServiceError::Connect)?;

        let resp = filter_status(resp).await?;

        let body = resp
            .json::<jsonwebtoken::jwk::Jwk>()
            .await
            .map_err(SessionServiceError::Body)?;

        Ok(body)
    }

    pub async fn get_game_profiles(
        &self,
        oauth_access_token: &str,
    ) -> Result<Vec<GameProfile>, SessionServiceError> {
        #[derive(Deserialize)]
        struct LauncherDataResponse {
            #[serde(rename = "owner")]
            _owner: Uuid,
            profiles: Vec<GameProfile>,
        }

        let resp = self
            .client
            .get("https://account-data.hytale.com/my-account/get-profiles")
            .header("Accept", "application/json")
            .header("Authorization", format!("Bearer {oauth_access_token}"))
            .header("User-Agent", USER_AGENT)
            .send()
            .await
            .map_err(SessionServiceError::Connect)?;

        let resp = filter_status(resp).await?;

        let body = resp
            .json::<LauncherDataResponse>()
            .await
            .map_err(SessionServiceError::Body)?;

        Ok(body.profiles)
    }

    pub async fn create_game_session(
        &self,
        oauth_access_token: &str,
        profile_uuid: Uuid,
    ) -> Result<GameSessionResponse, SessionServiceError> {
        #[derive(Serialize)]
        struct RequestBody {
            uuid: Uuid,
        }

        let resp = self
            .client
            .post(format!("{SESSION_SERVER_URL}/game-session/new"))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {oauth_access_token}"))
            .header("User-Agent", USER_AGENT)
            .json(&RequestBody { uuid: profile_uuid })
            .send()
            .await
            .map_err(SessionServiceError::Connect)?;

        let resp = filter_status(resp).await?;

        let body = resp
            .json::<GameSessionResponse>()
            .await
            .map_err(SessionServiceError::Body)?;

        Ok(body)
    }

    pub async fn refresh_session(
        &self,
        session_token: &str,
    ) -> Result<GameSessionResponse, SessionServiceError> {
        let resp = self
            .client
            .post(format!("{SESSION_SERVER_URL}/game-session/new"))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {session_token}"))
            .header("User-Agent", USER_AGENT)
            .send()
            .await
            .map_err(SessionServiceError::Connect)?;

        let resp = filter_status(resp).await?;

        let body = resp
            .json::<GameSessionResponse>()
            .await
            .map_err(SessionServiceError::Body)?;

        Ok(body)
    }

    pub async fn terminate_session(&self, session_token: &str) -> Result<(), SessionServiceError> {
        let resp = self
            .client
            .delete(format!("{SESSION_SERVER_URL}/game-session"))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {session_token}"))
            .header("User-Agent", USER_AGENT)
            .send()
            .await
            .map_err(SessionServiceError::Connect)?;

        let _resp = filter_status(resp).await?;

        Ok(())
    }

    // com/hypixel/hytale/server/core/auth/oauth/OAuthClient.java
    pub fn oauth_encode_state_with_port(csrf_state: &str, port: u16) -> String {
        #[derive(Serialize)]
        struct State<'a> {
            state: &'a str,
            port: String,
        }

        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(
            serde_json::to_string(&State {
                state: csrf_state,
                port: port.to_string(),
            })
            .unwrap(),
        )
    }

    pub fn oauth_build_auth_url(state: &str, code_challenge: &str) -> String {
        format!(
            "https://oauth.accounts.hytale.com/oauth2/auth\
             ?response_type=code\
             &client_id={}\
             &redirect_uri={}\
             &scope={}\
             &state={}\
             &code_challenge={}\
             &code_challenge_method=S256",
            urlencoding::encode(OAUTH_CLIENT_ID),
            urlencoding::encode(OAUTH_REDIRECT_URI),
            urlencoding::encode(OAUTH_SCOPES),
            urlencoding::encode(state),
            urlencoding::encode(code_challenge),
        )
    }

    pub async fn oauth_exchange_code_for_tokens(
        &self,
        code: &str,
        code_verifier: &str,
    ) -> Result<OAuthTokenResponse, SessionServiceError> {
        let resp = self
            .client
            .post("https://oauth.accounts.hytale.com/oauth2/token")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("User-Agent", USER_AGENT)
            .body(format!(
                "grant_type=authorization_code\
                 &client_id={}\
                 &code={}\
                 &redirect_uri={}\
                 &code_verifier={}",
                urlencoding::encode(OAUTH_CLIENT_ID),
                urlencoding::encode(code),
                urlencoding::encode(OAUTH_REDIRECT_URI),
                urlencoding::encode(code_verifier),
            ))
            .send()
            .await
            .map_err(SessionServiceError::Connect)?;

        let resp = filter_status(resp).await?;

        let body = resp
            .json::<OAuthTokenResponse>()
            .await
            .map_err(SessionServiceError::Body)?;

        Ok(body)
    }

    pub async fn oauth_request_device_authorization(
        &self,
    ) -> Result<OAuthDeviceResponse, SessionServiceError> {
        let resp = self
            .client
            .post("https://oauth.accounts.hytale.com/oauth2/device/auth")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("User-Agent", USER_AGENT)
            .body(format!(
                "client_id={}&scope={}",
                urlencoding::encode(OAUTH_CLIENT_ID),
                urlencoding::encode(OAUTH_SCOPES),
            ))
            .send()
            .await
            .map_err(SessionServiceError::Connect)?;

        let resp = filter_status(resp).await?;

        let body = resp
            .json::<OAuthDeviceResponse>()
            .await
            .map_err(SessionServiceError::Body)?;

        Ok(body)
    }

    pub async fn oauth_poll_device_token(
        &self,
        device_code: &str,
    ) -> Result<OAuthTokenResponse, SessionServiceError> {
        let resp = self
            .client
            .post("https://oauth.accounts.hytale.com/oauth2/token")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("User-Agent", USER_AGENT)
            .body(format!(
                "grant_type=urn:ietf:params:oauth:grant-type:device_code&client_id={}&device_code={}",
                urlencoding::encode(OAUTH_CLIENT_ID),
                urlencoding::encode(device_code),
            ))
            .send()
            .await
            .map_err(SessionServiceError::Connect)?;

        let resp = filter_status_advanced(
            resp,
            &[reqwest::StatusCode::OK, reqwest::StatusCode::BAD_REQUEST],
        )
        .await?;

        let body = resp
            .json::<OAuthTokenResponse>()
            .await
            .map_err(SessionServiceError::Body)?;

        Ok(body)
    }

    pub async fn oauth_refresh_tokens(
        &self,
        refresh_token: &str,
    ) -> Result<OAuthTokenResponse, SessionServiceError> {
        let resp = self
            .client
            .post("https://oauth.accounts.hytale.com/oauth2/token")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("User-Agent", USER_AGENT)
            .body(format!(
                "grant_type=refresh_token&client_id={}&refresh_token={}",
                urlencoding::encode(OAUTH_CLIENT_ID),
                urlencoding::encode(refresh_token),
            ))
            .send()
            .await
            .map_err(SessionServiceError::Connect)?;

        let resp = filter_status(resp).await?;

        let body = resp
            .json::<OAuthTokenResponse>()
            .await
            .map_err(SessionServiceError::Body)?;

        Ok(body)
    }
}

async fn filter_status(resp: reqwest::Response) -> Result<reqwest::Response, SessionServiceError> {
    filter_status_advanced(resp, &[reqwest::StatusCode::OK]).await
}

async fn filter_status_advanced(
    resp: reqwest::Response,
    ok_statuses: &[reqwest::StatusCode],
) -> Result<reqwest::Response, SessionServiceError> {
    let status = resp.status();

    if !ok_statuses.contains(&status) {
        let (Ok(body) | Err(body)) = resp.text().await.map_err(|v| v.to_string());

        return Err(SessionServiceError::Status { status, body });
    }

    Ok(resp)
}
