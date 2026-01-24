use std::{
    str::FromStr,
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};

use futures::future::Either;
use tokio::sync::mpsc;
use tracing::Instrument;
use uuid::Uuid;

use crate::session::{GameSessionResponse, OAuthTokenResponse, SessionService};

type CurrentCredentialsArc = Arc<RwLock<Arc<ServerAuthCredentials>>>;

#[derive(Debug, Clone)]
pub struct ServerAuthManager {
    inner: Arc<ServerAuthManagerInner>,
}

#[derive(Debug)]
struct ServerAuthManagerInner {
    session_id: Uuid,
    session_id_str: String,
    current_credentials: CurrentCredentialsArc,
    credential_sender: mpsc::Sender<ServerAuthCredentials>,
}

#[derive(Debug, Clone, Default)]
pub struct ServerAuthCredentials {
    // TODO: Better error representation
    pub oauth: Option<OAuthTokenResponse>,
    pub session: Option<GameSessionResponse>,
}

impl ServerAuthManager {
    pub fn new(session_service: SessionService) -> Self {
        let server_session_id = Uuid::new_v4();

        let (credential_sender, credential_receiver) = mpsc::channel(1);
        let current_credentials = Arc::new(RwLock::new(Arc::new(ServerAuthCredentials::default())));

        tokio::spawn(
            Self::background_task(
                session_service,
                current_credentials.clone(),
                credential_receiver,
            )
            .instrument(tracing::info_span!("SessionAuthManager::background_task")),
        );

        Self {
            inner: Arc::new(ServerAuthManagerInner {
                session_id: server_session_id,
                session_id_str: server_session_id.to_string(),
                current_credentials,
                credential_sender,
            }),
        }
    }

    pub fn session_id(&self) -> Uuid {
        self.inner.session_id
    }

    pub fn audience(&self) -> &str {
        &self.inner.session_id_str
    }

    pub async fn provide_credentials(&self, credentials: ServerAuthCredentials) {
        if self
            .inner
            .credential_sender
            .send(credentials)
            .await
            .is_err()
        {
            tracing::error!("Failed to provide credentials to background task");
        }
    }

    pub fn credentials(&self) -> Arc<ServerAuthCredentials> {
        self.inner.current_credentials.read().unwrap().clone()
    }

    // FIXME: Seems to spam on expiry?
    async fn background_task(
        session_service: SessionService,
        current_credentials: CurrentCredentialsArc,
        mut credential_receiver: mpsc::Receiver<ServerAuthCredentials>,
    ) {
        let mut oauth = None::<OAuthTokenResponse>;
        let mut session = None::<GameSessionResponse>;

        let mut oauth_expire_instant = None::<Instant>;
        let mut session_expire_instant = None::<Instant>;

        loop {
            // Handle OAuth expiry
            if let Some(oauth) = &oauth
                && oauth_expire_instant.is_none()
            {
                let dur = Duration::from_secs(oauth.expires_in as u64).max(Duration::from_secs(60));

                tracing::info!("Refreshing OAuth token in {dur:?}");
                oauth_expire_instant = Some(Instant::now() + dur);
            }

            if oauth.is_none() {
                oauth_expire_instant = None;
            }

            let oauth_refresh_future = match oauth_expire_instant {
                Some(instant) => Either::Left(tokio::time::sleep_until(instant.into())),
                None => Either::Right(std::future::pending::<()>()),
            };

            // Handle session expiry
            if let Some(session) = &session
                && session_expire_instant.is_none()
            {
                let expires_at = jiff::Timestamp::from_str(&session.expires_at)
                    .unwrap()
                    .duration_since(jiff::Timestamp::now());

                let dur = Duration::from_secs(expires_at.as_secs().max(60) as u64);

                tracing::info!("Refreshing session token in {dur:?}");
                session_expire_instant = Some(Instant::now() + dur);
            }

            if session.is_none() {
                session_expire_instant = None;
            }

            let session_refresh_future = match session_expire_instant {
                Some(instant) => Either::Left(tokio::time::sleep_until(instant.into())),
                None => Either::Right(std::future::pending::<()>()),
            };

            // Process the next event.
            tokio::select! {
                new = credential_receiver.recv() => {
                    match new {
                        Some(new) => {
                            oauth = new.oauth;
                            session = new.session;
                        }
                        None => {
                            break;
                        }
                    }
                }
                () = oauth_refresh_future => {
                    tracing::info!("Refreshing OAuth token...");

                    let refresh_token = oauth.as_ref().unwrap().refresh_token.as_ref().unwrap();
                    let new_oauth = session_service
                        .oauth_refresh_tokens(refresh_token)
                        .await;

                    match new_oauth {
                        Ok(new_oauth @ OAuthTokenResponse { error: None, .. }) => {
                            oauth = Some(new_oauth);
                            tracing::info!("Refreshed OAuth token");
                        }
                        Ok(OAuthTokenResponse { error: Some(error), .. }) => {
                            oauth = None;
                            tracing::error!("failed to refresh OAuth token: {error}");
                        }
                        Err(error) => {
                            oauth = None;
                            tracing::error!("failed to refresh OAuth token: {error}");
                        }
                    }
                }
                () = session_refresh_future => {
                    tracing::info!("Refreshing session token...");

                    let new_session = session_service
                        .refresh_session(&session.as_ref().unwrap().session_token)
                        .await;

                    match new_session {
                        Ok(new_session) => {
                            session = Some(new_session);
                            tracing::info!("Refreshed session token");
                        }
                        Err(err) => {
                            session = None;
                            tracing::error!("failed to refresh session token: {err}");
                        }
                    }
                }
            }

            // If we have an OAuth token but no session token, attempt to create it.
            if let Some(oauth) = &oauth
                && session.is_none()
            {
                tracing::info!("Deriving session token from OAuth...");

                match Self::derive_session_from_oauth(&session_service, oauth).await {
                    Ok(new_session) => {
                        session = Some(new_session);
                        tracing::info!("Derived session token from OAuth!");
                    }
                    Err(error) => {
                        tracing::error!("failed to derive session token from OAuth: {error}");
                    }
                }
            }

            // Update the credential store to reflect our credentials.
            tracing::info!(
                "Updated ServerAuthManager credentials (oauth={}, session={})",
                oauth.is_some(),
                session.is_some()
            );

            let mut guard = current_credentials.write().unwrap();

            *guard = Arc::new(ServerAuthCredentials {
                oauth: oauth.clone(),
                session: session.clone(),
            });
        }
    }

    async fn derive_session_from_oauth(
        session_service: &SessionService,
        oauth: &OAuthTokenResponse,
    ) -> miette::Result<GameSessionResponse> {
        let oauth_access_token = oauth.access_token.as_ref().unwrap();

        let profiles = session_service
            .get_game_profiles(oauth_access_token)
            .await?;

        // TODO: Select profile by UUID
        let [profile] = &profiles[..] else {
            miette::bail!(
                "expected exactly one unique profile, got {}",
                profiles.len()
            );
        };

        let session = session_service
            .create_game_session(oauth_access_token, profile.uuid)
            .await?;

        Ok(session)
    }
}
