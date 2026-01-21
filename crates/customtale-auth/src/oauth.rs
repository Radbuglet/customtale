// com/hypixel/hytale/server/core/auth/oauth/OAuthClient.java

use std::{
    convert::Infallible,
    net::Ipv4Addr,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use base64::Engine;
use miette::Diagnostic;
use serde::Deserialize;
use sha2::Digest;
use thiserror::Error;
use tokio::sync::oneshot;
use warp::{
    Filter,
    http::{Response, StatusCode},
};

use crate::session::{
    OAuthDeviceResponse, OAuthTokenResponse, SessionService, SessionServiceError,
};

// === Common === //

#[derive(Debug, Error, Diagnostic)]
pub enum OAuthFlowError {
    #[error("failed to generate random bytes for the operation")]
    RngFailed,
    #[error("failed to request device OAuth")]
    RequestDeviceOAuth(#[source] SessionServiceError),
    #[error("failed to start oauth callback server")]
    StartCallbackServer(#[source] tokio::io::Error),
    #[error("local oauth callback server crashed")]
    CallbackServerCrashed,
    #[error("callback received invalid CSRF state")]
    RespInvalidState,
    #[error("callback did not receive OAuth code")]
    RespMissingCode,
    #[error("failed to exchange OAuth code for token")]
    OAuthCodeExchange(#[source] SessionServiceError),
    #[error("failed to poll for OAuth token")]
    OAuthDevicePoll(#[source] SessionServiceError),
    #[error("failed to poll for OAuth token: {0}")]
    OAuthDevicePollCustom(String),
    #[error("timed out")]
    TimedOut,
}

fn generate_random_string(len: usize) -> Result<String, OAuthFlowError> {
    let mut dest = vec![0; len];
    aws_lc_rs::rand::fill(&mut dest).map_err(|_| OAuthFlowError::RngFailed)?;
    Ok(base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&dest))
}

fn generate_code_challenge(code_verifier: &str) -> String {
    let digest = sha2::Sha256::digest(code_verifier.as_bytes());
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(digest)
}

// === OAuthBrowserFlow === //

pub struct OAuthBrowserFlow {
    auth_url: String,
    code_verifier: String,
    _shutdown_tx: oneshot::Sender<Infallible>,
    session_service: SessionService,
    got_code_rx: oneshot::Receiver<Result<String, OAuthFlowError>>,
}

impl OAuthBrowserFlow {
    pub async fn start(session_service: SessionService) -> Result<Self, OAuthFlowError> {
        let csrf_state = generate_random_string(32)?;
        let code_verifier = generate_random_string(64)?;
        let code_challenge = generate_code_challenge(&code_verifier);

        let server = tokio::net::TcpListener::bind((Ipv4Addr::LOCALHOST, 0))
            .await
            .map_err(OAuthFlowError::StartCallbackServer)?;

        let port = server.local_addr().unwrap().port();

        let encoded_state = SessionService::oauth_encode_state_with_port(&csrf_state, port);

        let (got_code_tx, got_code_rx) = oneshot::channel();
        let (shutdown_tx, shutdown_rx) = oneshot::channel();

        tokio::spawn(Self::run_server(
            server,
            csrf_state,
            got_code_tx,
            shutdown_rx,
        ));

        let auth_url = SessionService::oauth_build_auth_url(&encoded_state, &code_challenge);

        Ok(Self {
            auth_url,
            code_verifier,
            _shutdown_tx: shutdown_tx,
            session_service,
            got_code_rx,
        })
    }

    pub fn auth_url(&self) -> &str {
        &self.auth_url
    }

    pub async fn finished(self) -> Result<OAuthTokenResponse, OAuthFlowError> {
        let code = self
            .got_code_rx
            .await
            .map_err(|_| OAuthFlowError::CallbackServerCrashed)??;

        let resp = self
            .session_service
            .oauth_exchange_code_for_tokens(&code, &self.code_verifier)
            .await
            .map_err(OAuthFlowError::OAuthCodeExchange)?;

        Ok(resp)
    }

    async fn run_server(
        server: tokio::net::TcpListener,
        csrf_state: String,
        got_code_tx: oneshot::Sender<Result<String, OAuthFlowError>>,
        shutdown_rx: oneshot::Receiver<Infallible>,
    ) {
        #[derive(Debug, Deserialize)]
        struct ServerQuery {
            code: Option<String>,
            state: Option<String>,
        }

        let got_code_tx = Arc::new(Mutex::new(Some(got_code_tx)));

        let filter = warp::any().and(warp::get()).and(warp::query()).map({
            let got_code_tx = got_code_tx.clone();

            move |query: ServerQuery| {
                let Some(got_code_tx) = got_code_tx.lock().unwrap().take() else {
                    return Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body("OAuth callback can only be called once".to_string());
                };

                if query.state.is_none_or(|v| v != csrf_state.as_str()) {
                    _ = got_code_tx.send(Err(OAuthFlowError::RespInvalidState));

                    return Response::builder().status(StatusCode::BAD_REQUEST).body(
                        "Authentication Failed\n\
                         Something went wrong during authentication. \
                         Please close this window and try again.\n\
                         Invalid state parameter"
                            .to_string(),
                    );
                }

                let Some(code) = query.code.filter(|v| !v.is_empty()) else {
                    _ = got_code_tx.send(Err(OAuthFlowError::RespMissingCode));

                    return Response::builder().status(StatusCode::BAD_REQUEST).body(
                        "Authentication Failed\n\
                         Something went wrong during authentication. \
                         Please close this window and try again.\n\
                         Code was not received or empty."
                            .to_string(),
                    );
                };

                _ = got_code_tx.send(Ok(code));

                Response::builder().status(StatusCode::BAD_REQUEST).body(
                    "Authentication Successful\n\
                     You have been logged in successfully. \
                     You can now close this window and return to the server."
                        .to_string(),
                )
            }
        });

        let server = warp::serve(filter).incoming(server).run();

        tokio::select! {
            () = server => {}
            () = tokio::time::sleep(Duration::from_mins(5)) => {
                if let Some(chan) = got_code_tx.lock().unwrap().take() {
                    _ = chan.send(Err(OAuthFlowError::TimedOut));
                }
            }
            Err(_) = shutdown_rx => {}
        }
    }
}

// === OAuthDeviceFlow === //

pub struct OAuthDeviceFlow {
    session_service: SessionService,
    deadline: Instant,
    device_auth: OAuthDeviceResponse,
}

impl OAuthDeviceFlow {
    pub async fn start(session_service: SessionService) -> Result<Self, OAuthFlowError> {
        let device_auth = session_service
            .oauth_request_device_authorization()
            .await
            .map_err(OAuthFlowError::RequestDeviceOAuth)?;

        let deadline = Instant::now() + Duration::from_secs(device_auth.expires_in.0 as u64);

        Ok(Self {
            session_service,
            deadline,
            device_auth,
        })
    }

    pub fn verification_uri(&self) -> &str {
        self.device_auth.verification_uri.as_ref().unwrap()
    }

    pub fn verification_code(&self) -> &str {
        self.device_auth.user_code.as_ref().unwrap()
    }

    pub fn verification_uri_complete(&self) -> &str {
        self.device_auth.verification_uri_complete.as_ref().unwrap()
    }

    pub async fn finished(self) -> Result<OAuthTokenResponse, OAuthFlowError> {
        let mut poll_interval =
            Duration::from_secs(self.device_auth.interval.0 as u64).max(Duration::from_secs(5));

        while Instant::now() < self.deadline {
            tokio::time::sleep(poll_interval).await;

            let tokens = self
                .session_service
                .oauth_poll_device_token(self.device_auth.device_code.as_ref().unwrap())
                .await
                .map_err(OAuthFlowError::OAuthDevicePoll)?;

            if let Some(err) = tokens.error {
                match err.as_str() {
                    "authorization_pending" => {
                        // (continue)
                    }
                    "slow_down" => {
                        poll_interval += Duration::from_secs(5);
                    }
                    _ => {
                        return Err(OAuthFlowError::OAuthDevicePollCustom(err));
                    }
                }
            } else {
                return Ok(tokens);
            }
        }

        Err(OAuthFlowError::TimedOut)
    }
}

// === OAuthRefreshTask === //

// TODO
