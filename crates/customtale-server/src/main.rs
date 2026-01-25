use std::{
    collections::HashMap,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    sync::Arc,
};

use customtale_auth::{
    fingerprint::compute_certificate_fingerprint,
    manager::{ServerAuthCredentials, ServerAuthManager},
    oauth::OAuthBrowserFlow,
    session::SessionService,
};
use customtale_protocol::packets::{
    AnyPacket, PacketCategory,
    assets::{
        UpdateAmbienceFX, UpdateAudioCategories, UpdateBlockBreakingDecals, UpdateBlockGroups,
        UpdateBlockHitboxes, UpdateBlockParticleSets, UpdateBlockTypes, UpdateCameraShake,
        UpdateEntityEffects, UpdateEntityStatTypes, UpdateEnvironments, UpdateEqualizerEffects,
        UpdateFieldcraftCategories, UpdateFluidFx, UpdateFluids, UpdateHitboxCollisionConfig,
        UpdateInteractions, UpdateType,
    },
    auth::{AuthGrant, ServerAuthToken},
    setup::{WorldLoadFinished, WorldLoadProgress, WorldSettings},
};
use futures::{SinkExt, StreamExt};
use miette::IntoDiagnostic;
use quinn::{
    crypto::rustls::QuicServerConfig,
    rustls::{
        self,
        pki_types::{CertificateDer, PrivatePkcs8KeyDer},
    },
};
use rustls::crypto::CryptoProvider;
use tokio_util::codec::Framed;
use tracing_subscriber::util::SubscriberInitExt;

use crate::framed::{HytaleDecoder, HytaleEncoder};

pub mod framed;

// TODO: Implement actual authentication and socket handling.
#[tokio::main]
async fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive("INFO".parse().unwrap())
                .from_env_lossy(),
        )
        .finish()
        .try_init()
        .unwrap();

    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .unwrap();

    let session_service = SessionService::new()?;
    let auth_manager = ServerAuthManager::new(session_service.clone());

    let flow = OAuthBrowserFlow::start(session_service.clone()).await?;

    tracing::info!("OAuth path: {}", flow.auth_url());

    let oauth = flow.finished().await?;

    auth_manager
        .provide_credentials(ServerAuthCredentials {
            oauth: Some(oauth),
            session: None,
        })
        .await;

    // TODO: com/hypixel/hytale/server/core/io/transport/QUICTransport.java
    let ssc =
        rcgen::generate_simple_self_signed(vec!["localhost".to_string()]).into_diagnostic()?;

    let cert_der = CertificateDer::from(ssc.cert);
    let key = PrivatePkcs8KeyDer::from(ssc.signing_key.serialize_der());

    let cert_fingerprint = Arc::new(compute_certificate_fingerprint(&cert_der));

    let mut tls_server_config =
        rustls::ServerConfig::builder_with_provider(CryptoProvider::get_default().unwrap().clone())
            .with_protocol_versions(&[&rustls::version::TLS13])
            .unwrap()
            .with_no_client_auth()
            .with_single_cert(vec![cert_der], key.into())
            .unwrap();

    tls_server_config.alpn_protocols = vec![b"hytale/2".to_vec(), b"hytale/1".to_vec()];

    let suite = tls_server_config
        .crypto_provider()
        .cipher_suites
        .iter()
        .find_map(|cs| match (cs.suite(), cs.tls13()) {
            (rustls::CipherSuite::TLS13_AES_128_GCM_SHA256, Some(suite)) => {
                Some(suite.quic_suite())
            }
            _ => None,
        })
        .flatten();

    let crypto =
        QuicServerConfig::with_initial(Arc::new(tls_server_config), suite.unwrap()).unwrap();
    let crypto = Arc::new(crypto);

    let endpoint = quinn::Endpoint::server(
        quinn::ServerConfig::with_crypto(crypto),
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 5520)),
    )
    .into_diagnostic()?;

    while let Some(incoming) = endpoint.accept().await {
        let session_service = session_service.clone();
        let auth_manager = auth_manager.clone();
        let cert_fingerprint = cert_fingerprint.clone();

        tokio::spawn(async move {
            let conn = incoming.await.unwrap();

            let (tx, rx) = conn.accept_bi().await.unwrap();

            // com/hypixel/hytale/server/core/io/netty/HytaleChannelInitializer.java
            // com/hypixel/hytale/protocol/io/netty/PacketDecoder.java
            // com/hypixel/hytale/protocol/io/netty/PacketEncoder.java

            let mut tx = Framed::new(tx, HytaleEncoder);
            let mut rx = Framed::new(
                rx,
                HytaleDecoder {
                    allowed_categories: PacketCategory::CONNECTION,
                },
            );

            let Some(packet1) = rx.next().await else {
                return;
            };

            let AnyPacket::Connect(packet1) = packet1.unwrap() else {
                panic!("what?");
            };

            let server_credentials = auth_manager.credentials();
            let server_credentials = server_credentials.session.as_ref().unwrap();

            let grant = session_service
                .request_authorization_grant(
                    packet1.identity_token.as_ref().unwrap(),
                    auth_manager.audience(),
                    &server_credentials.session_token,
                )
                .await
                .unwrap();

            tx.send(AnyPacket::AuthGrant(AuthGrant {
                authorization_grant: Some(grant.clone()),
                server_identity_token: Some(server_credentials.identity_token.clone()),
            }))
            .await
            .unwrap();

            rx.codec_mut().allowed_categories |= PacketCategory::AUTH;

            let Some(packet2) = rx.next().await else {
                return;
            };

            let AnyPacket::AuthToken(packet2) = packet2.unwrap() else {
                panic!("what?");
            };

            dbg!(&packet2);

            let server_access_token = session_service
                .exchange_auth_grant_for_token(
                    packet2.server_authorization_grant.as_ref().unwrap(),
                    &cert_fingerprint,
                    &server_credentials.session_token,
                )
                .await
                .unwrap();

            tx.send(AnyPacket::ServerAuthToken(ServerAuthToken {
                server_access_token: Some(server_access_token),
                password_challenge: None,
            }))
            .await
            .unwrap();

            // We've authenticated!
            // com/hypixel/hytale/server/core/io/handlers/SetupPacketHandler.java
            tracing::info!("Authenticated!");
            rx.codec_mut().allowed_categories |= PacketCategory::SETUP;

            tx.send(AnyPacket::WorldSettings(WorldSettings {
                world_height: 320,
                required_assets: Some(Vec::new()),
            }))
            .await
            .unwrap();

            loop {
                let Some(packet3) = rx.next().await else {
                    return;
                };

                match packet3.unwrap() {
                    AnyPacket::RequestAssets(_) => {
                        tx.send(AnyPacket::UpdateAmbienceFX(UpdateAmbienceFX {
                            type_: UpdateType::Init,
                            max_id: 0,
                            ambience_fx: Some(HashMap::default()),
                        }))
                        .await
                        .unwrap();

                        tx.send(AnyPacket::UpdateAudioCategories(UpdateAudioCategories {
                            type_: UpdateType::Init,
                            max_id: 0,
                            categories: Some(HashMap::default()),
                        }))
                        .await
                        .unwrap();

                        tx.send(AnyPacket::UpdateBlockBreakingDecals(
                            UpdateBlockBreakingDecals {
                                type_: UpdateType::Init,
                                block_breaking_decals: Some(HashMap::default()),
                            },
                        ))
                        .await
                        .unwrap();

                        tx.send(AnyPacket::UpdateBlockGroups(UpdateBlockGroups {
                            type_: UpdateType::Init,
                            groups: Some(HashMap::default()),
                        }))
                        .await
                        .unwrap();

                        tx.send(AnyPacket::UpdateBlockHitboxes(UpdateBlockHitboxes {
                            type_: UpdateType::Init,
                            max_id: 0,
                            block_base_hitboxes: Some(HashMap::default()),
                        }))
                        .await
                        .unwrap();

                        tx.send(AnyPacket::UpdateBlockParticleSets(
                            UpdateBlockParticleSets {
                                type_: UpdateType::Init,
                                block_particle_sets: Some(HashMap::default()),
                            },
                        ))
                        .await
                        .unwrap();

                        tx.send(AnyPacket::UpdateBlockTypes(UpdateBlockTypes {
                            type_: UpdateType::Init,
                            max_id: 0,
                            block_types: Some(HashMap::default()),
                            update_block_textures: true,
                            update_model_textures: true,
                            update_models: true,
                            update_map_geometry: true,
                        }))
                        .await
                        .unwrap();

                        tx.send(AnyPacket::UpdateCameraShake(UpdateCameraShake {
                            type_: UpdateType::Init,
                            profiles: Some(HashMap::default()),
                        }))
                        .await
                        .unwrap();

                        tx.send(AnyPacket::UpdateEntityEffects(UpdateEntityEffects {
                            type_: UpdateType::Init,
                            max_id: 0,
                            entity_effects: Some(HashMap::default()),
                        }))
                        .await
                        .unwrap();

                        tx.send(AnyPacket::UpdateEntityStatTypes(UpdateEntityStatTypes {
                            type_: UpdateType::Init,
                            max_id: 0,
                            types: Some(HashMap::default()),
                        }))
                        .await
                        .unwrap();

                        tx.send(AnyPacket::UpdateEnvironments(UpdateEnvironments {
                            type_: UpdateType::Init,
                            max_id: 0,
                            environments: Some(HashMap::default()),
                            rebuild_map_geometry: true,
                        }))
                        .await
                        .unwrap();

                        tx.send(AnyPacket::UpdateEqualizerEffects(UpdateEqualizerEffects {
                            type_: UpdateType::Init,
                            max_id: 0,
                            effects: Some(HashMap::default()),
                        }))
                        .await
                        .unwrap();

                        tx.send(AnyPacket::UpdateFieldcraftCategories(
                            UpdateFieldcraftCategories {
                                type_: UpdateType::Init,
                                item_categories: Some(Vec::new()),
                            },
                        ))
                        .await
                        .unwrap();

                        tx.send(AnyPacket::UpdateFluidFx(UpdateFluidFx {
                            type_: UpdateType::Init,
                            max_id: 0,
                            fluid_fx: Some(HashMap::default()),
                        }))
                        .await
                        .unwrap();

                        tx.send(AnyPacket::UpdateFluids(UpdateFluids {
                            type_: UpdateType::Init,
                            max_id: 0,
                            fluids: Some(HashMap::default()),
                        }))
                        .await
                        .unwrap();

                        tx.send(AnyPacket::UpdateHitboxCollisionConfig(
                            UpdateHitboxCollisionConfig {
                                type_: UpdateType::Init,
                                max_id: 0,
                                hitbox_collision_configs: Some(HashMap::default()),
                            },
                        ))
                        .await
                        .unwrap();

                        tx.send(AnyPacket::UpdateInteractions(UpdateInteractions {
                            type_: UpdateType::Init,
                            max_id: 0,
                            interactions: Some(HashMap::default()),
                        }))
                        .await
                        .unwrap();

                        tx.send(AnyPacket::WorldLoadProgress(WorldLoadProgress {
                            status: Some("Meowing".to_string()),
                            percent_complete: 50,
                            percent_complete_subitem: 0,
                        }))
                        .await
                        .unwrap();

                        tx.send(AnyPacket::WorldLoadFinished(WorldLoadFinished))
                            .await
                            .unwrap();
                    }
                    AnyPacket::ViewRadius(_) => {}
                    AnyPacket::PlayerOptions(_) => {}
                    AnyPacket::Disconnect(_) => {}
                    _ => unreachable!(),
                }
            }

            // tx.get_mut().finish().unwrap();
            // tx.get_mut().stopped().await.unwrap();
        });
    }

    Ok(())
}
