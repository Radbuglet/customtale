// com/hypixel/hytale/server/core/auth/oauth/OAuthClient.java

use std::{convert::Infallible, net::Ipv4Addr};

use base64::Engine;
use miette::Diagnostic;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use thiserror::Error;
use tokio::sync::oneshot;
use warp::Filter;

use crate::session::{OAUTH_CLIENT_ID, SessionService};

#[derive(Debug, Error, Diagnostic)]
pub enum OauthFlowError {
    #[error("failed to generate random bytes for the operation")]
    RngFailed,
    #[error("failed to start oauth callback server")]
    StartServer(#[from] tokio::io::Error),
}

pub struct OauthFlow {
    auth_url: String,
    _shutdown_tx: oneshot::Sender<Infallible>,
    session_service: SessionService,
    got_code_rx: oneshot::Receiver<Result<String, OauthFlowError>>,
}

impl OauthFlow {
    pub async fn start(session_service: SessionService) -> Result<Self, OauthFlowError> {
        let csrf_state = generate_random_string(32)?;
        let code_verifier = generate_random_string(64)?;
        let code_challenge = generate_code_challenge(&code_verifier);

        let server = tokio::net::TcpListener::bind((Ipv4Addr::LOCALHOST, 0))
            .await
            .map_err(OauthFlowError::StartServer)?;

        let port = server.local_addr().unwrap().port();

        let encoded_state = encode_state_with_port(&csrf_state, port);
        let redirect_uri = "https://accounts.hytale.com/consent/client";

        let (got_code_tx, got_code_rx) = oneshot::channel();
        let (shutdown_tx, shutdown_rx) = oneshot::channel();

        tokio::spawn(run_server(server, got_code_tx, shutdown_rx));

        let auth_url = build_auth_url(&encoded_state, &code_challenge, redirect_uri);

        Ok(Self {
            auth_url,
            _shutdown_tx: shutdown_tx,
            session_service,
            got_code_rx,
        })
    }

    pub fn auth_url(&self) -> &str {
        &self.auth_url
    }

    pub async fn finished(self) -> Result<String, OauthFlowError> {
        let code = self.got_code_rx.await;

        todo!()
    }
}

async fn run_server(
    server: tokio::net::TcpListener,
    got_code_tx: oneshot::Sender<Result<String, OauthFlowError>>,
    shutdown_rx: oneshot::Receiver<Infallible>,
) {
    #[derive(Debug, Deserialize)]
    struct ServerQuery {
        code: Option<String>,
        state: Option<String>,
    }

    let filter = warp::path!("/")
        .and(warp::get())
        .and(warp::query())
        .map(|query: ServerQuery| format!("{query:?}"));

    let server = warp::serve(filter).incoming(server).run();

    tokio::select! {
        () = server => {}
        Err(_) = shutdown_rx => {}
    }
}

fn generate_random_string(len: usize) -> Result<String, OauthFlowError> {
    let mut dest = vec![0; len];
    aws_lc_rs::rand::fill(&mut dest).map_err(|_| OauthFlowError::RngFailed)?;
    Ok(base64::engine::general_purpose::STANDARD_NO_PAD.encode(&dest))
}

fn generate_code_challenge(code_verifier: &str) -> String {
    let digest = sha2::Sha256::digest(code_verifier.as_bytes());
    base64::engine::general_purpose::STANDARD_NO_PAD.encode(digest)
}

fn encode_state_with_port(csrf_state: &str, port: u16) -> String {
    #[derive(Serialize)]
    struct State<'a> {
        state: &'a str,
        port: u16,
    }

    serde_json::to_string(&State {
        state: csrf_state,
        port,
    })
    .unwrap()
}

fn build_auth_url(state: &str, code_challenge: &str, redirect_uri: &str) -> String {
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
        urlencoding::encode(redirect_uri),
        urlencoding::encode("openid offline auth:server"),
        urlencoding::encode(state),
        urlencoding::encode(code_challenge),
    )
}
