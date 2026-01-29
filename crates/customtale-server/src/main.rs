use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    sync::Arc,
};

use customtale_auth::{
    fingerprint::compute_certificate_fingerprint,
    manager::{ServerAuthCredentials, ServerAuthManager},
    oauth::OAuthBrowserFlow,
    session::SessionService,
};
use customtale_protocol::{
    packets::{
        AnyPacket, AuthGrant, ItemCategory, ItemGridInfoDisplayMode, PacketCategory,
        ServerAuthToken, UpdateAmbienceFX, UpdateAudioCategories, UpdateBlockBreakingDecals,
        UpdateBlockGroups, UpdateBlockHitboxes, UpdateBlockParticleSets, UpdateBlockSets,
        UpdateBlockSoundSets, UpdateBlockTypes, UpdateCameraShake, UpdateEntityEffects,
        UpdateEntityStatTypes, UpdateEntityUIComponents, UpdateEnvironments,
        UpdateEqualizerEffects, UpdateFieldcraftCategories, UpdateFluidFX, UpdateFluids,
        UpdateHitboxCollisionConfig, UpdateInteractions, UpdateItemCategories,
        UpdateItemPlayerAnimations, UpdateItemQualities, UpdateItemReticles, UpdateItemSoundSets,
        UpdateModelvfxs, UpdateParticleSpawners, UpdateParticleSystems, UpdateRecipes,
        UpdateRepulsionConfig, UpdateResourceTypes, UpdateReverbEffects, UpdateRootInteractions,
        UpdateSoundEvents, UpdateSoundSets, UpdateTagPatterns, UpdateTrails, UpdateTranslations,
        UpdateType, UpdateUnarmedInteractions, UpdateWeathers, WorldLoadFinished,
        WorldLoadProgress, WorldSettings,
    },
    serde::Dictionary,
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
                    packet1.identityToken.as_ref().unwrap(),
                    auth_manager.audience(),
                    &server_credentials.session_token,
                )
                .await
                .unwrap();

            tx.send(
                AuthGrant {
                    authorizationGrant: Some(grant.clone()),
                    serverIdentityToken: Some(server_credentials.identity_token.clone()),
                }
                .into(),
            )
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
                    packet2.serverAuthorizationGrant.as_ref().unwrap(),
                    &cert_fingerprint,
                    &server_credentials.session_token,
                )
                .await
                .unwrap();

            tx.send(
                ServerAuthToken {
                    serverAccessToken: Some(server_access_token),
                    passwordChallenge: None,
                }
                .into(),
            )
            .await
            .unwrap();

            // We've authenticated!
            // com/hypixel/hytale/server/core/io/handlers/SetupPacketHandler.java
            tracing::info!("Authenticated!");
            rx.codec_mut().allowed_categories |= PacketCategory::SETUP;

            tx.send(
                WorldSettings {
                    worldHeight: 320,
                    requiredAssets: Some(Vec::new()),
                }
                .into(),
            )
            .await
            .unwrap();

            loop {
                let Some(packet3) = rx.next().await else {
                    return;
                };

                match packet3.unwrap() {
                    AnyPacket::RequestAssets(_) => {
                        tx.send(
                            UpdateAmbienceFX {
                                r#type: UpdateType::Init,
                                maxId: 0,
                                ambienceFX: Some(Dictionary::default()),
                            }
                            .into(),
                        )
                        .await
                        .unwrap();

                        tx.send(
                            UpdateAudioCategories {
                                r#type: UpdateType::Init,
                                maxId: 0,
                                categories: Some(Dictionary::default()),
                            }
                            .into(),
                        )
                        .await
                        .unwrap();

                        tx.send(
                            UpdateBlockBreakingDecals {
                                r#type: UpdateType::Init,
                                blockBreakingDecals: Some(Dictionary::default()),
                            }
                            .into(),
                        )
                        .await
                        .unwrap();

                        tx.send(
                            UpdateBlockGroups {
                                r#type: UpdateType::Init,
                                groups: Some(Dictionary::default()),
                            }
                            .into(),
                        )
                        .await
                        .unwrap();

                        tx.send(
                            UpdateBlockHitboxes {
                                r#type: UpdateType::Init,
                                maxId: 0,
                                blockBaseHitboxes: Some(Dictionary::default()),
                            }
                            .into(),
                        )
                        .await
                        .unwrap();

                        tx.send(
                            UpdateBlockParticleSets {
                                r#type: UpdateType::Init,
                                blockParticleSets: Some(Dictionary::default()),
                            }
                            .into(),
                        )
                        .await
                        .unwrap();

                        tx.send(
                            UpdateBlockTypes {
                                r#type: UpdateType::Init,
                                maxId: 0,
                                blockTypes: Some(Dictionary::default()),
                                updateBlockTextures: true,
                                updateModelTextures: true,
                                updateModels: true,
                                updateMapGeometry: true,
                            }
                            .into(),
                        )
                        .await
                        .unwrap();

                        tx.send(
                            UpdateCameraShake {
                                r#type: UpdateType::Init,
                                profiles: Some(Dictionary::default()),
                            }
                            .into(),
                        )
                        .await
                        .unwrap();

                        tx.send(
                            UpdateEntityEffects {
                                r#type: UpdateType::Init,
                                maxId: 0,
                                entityEffects: Some(Dictionary::default()),
                            }
                            .into(),
                        )
                        .await
                        .unwrap();

                        tx.send(
                            UpdateEntityStatTypes {
                                r#type: UpdateType::Init,
                                maxId: 0,
                                types: Some(Dictionary::default()),
                            }
                            .into(),
                        )
                        .await
                        .unwrap();

                        tx.send(
                            UpdateEnvironments {
                                r#type: UpdateType::Init,
                                maxId: 0,
                                environments: Some(Dictionary::default()),
                                rebuildMapGeometry: true,
                            }
                            .into(),
                        )
                        .await
                        .unwrap();

                        tx.send(
                            UpdateEqualizerEffects {
                                r#type: UpdateType::Init,
                                maxId: 0,
                                effects: Some(Dictionary::default()),
                            }
                            .into(),
                        )
                        .await
                        .unwrap();

                        tx.send(
                            UpdateFieldcraftCategories {
                                r#type: UpdateType::Init,
                                itemCategories: Some(Vec::new()),
                            }
                            .into(),
                        )
                        .await
                        .unwrap();

                        tx.send(
                            UpdateFluidFX {
                                r#type: UpdateType::Init,
                                maxId: 0,
                                fluidFX: Some(Dictionary::default()),
                            }
                            .into(),
                        )
                        .await
                        .unwrap();

                        tx.send(
                            UpdateFluids {
                                r#type: UpdateType::Init,
                                maxId: 0,
                                fluids: Some(Dictionary::default()),
                            }
                            .into(),
                        )
                        .await
                        .unwrap();

                        tx.send(
                            UpdateHitboxCollisionConfig {
                                r#type: UpdateType::Init,
                                maxId: 0,
                                hitboxCollisionConfigs: Some(Dictionary::default()),
                            }
                            .into(),
                        )
                        .await
                        .unwrap();

                        tx.send(
                            UpdateItemCategories {
                                r#type: UpdateType::Init,
                                itemCategories: Some(vec![ItemCategory {
                                    id: Some("Blocks".to_string()),
                                    name: Some("Blocks".to_string()),
                                    icon: Some("Icons/ItemCategories/Natural.png".to_string()),
                                    order: 0,
                                    infoDisplayMode: ItemGridInfoDisplayMode::None,
                                    children: Some(vec![ItemCategory {
                                        id: Some("Rocks".to_string()),
                                        name: Some("server.ui.itemcategory.rocks".to_string()),
                                        icon: Some("Icons/ItemCategories/Blocks.png".to_string()),
                                        order: 0,
                                        infoDisplayMode: ItemGridInfoDisplayMode::None,
                                        children: None,
                                    }]),
                                }]),
                            }
                            .into(),
                        )
                        .await
                        .unwrap();

                        tx.send(
                            UpdateItemPlayerAnimations {
                                r#type: UpdateType::Init,
                                itemPlayerAnimations: Some(Dictionary::default()),
                            }
                            .into(),
                        )
                        .await
                        .unwrap();

                        tx.send(
                            UpdateItemQualities {
                                r#type: UpdateType::Init,
                                maxId: 0,
                                itemQualities: Some(Dictionary::default()),
                            }
                            .into(),
                        )
                        .await
                        .unwrap();

                        tx.send(
                            UpdateItemReticles {
                                r#type: UpdateType::Init,
                                maxId: 0,
                                itemReticleConfigs: Some(Dictionary::default()),
                            }
                            .into(),
                        )
                        .await
                        .unwrap();

                        tx.send(
                            UpdateParticleSpawners {
                                r#type: UpdateType::Init,
                                particleSpawners: Some(Dictionary::default()),
                                removedParticleSpawners: Some(Vec::new()),
                            }
                            .into(),
                        )
                        .await
                        .unwrap();

                        tx.send(
                            UpdateParticleSystems {
                                r#type: UpdateType::Init,
                                particleSystems: Some(Dictionary::default()),
                                removedParticleSystems: Some(Vec::new()),
                            }
                            .into(),
                        )
                        .await
                        .unwrap();

                        tx.send(
                            UpdateResourceTypes {
                                r#type: UpdateType::Init,
                                resourceTypes: Some(Dictionary::default()),
                            }
                            .into(),
                        )
                        .await
                        .unwrap();

                        tx.send(
                            UpdateWeathers {
                                r#type: UpdateType::Init,
                                maxId: 0,
                                weathers: Some(Dictionary::default()),
                            }
                            .into(),
                        )
                        .await
                        .unwrap();

                        tx.send(
                            UpdateTranslations {
                                r#type: UpdateType::Init,
                                translations: Some(Dictionary::default()),
                            }
                            .into(),
                        )
                        .await
                        .unwrap();

                        tx.send(
                            UpdateTrails {
                                r#type: UpdateType::Init,
                                trails: Some(Dictionary::default()),
                            }
                            .into(),
                        )
                        .await
                        .unwrap();

                        tx.send(
                            UpdateSoundEvents {
                                r#type: UpdateType::Init,
                                maxId: 0,
                                soundEvents: Some(Dictionary::default()),
                            }
                            .into(),
                        )
                        .await
                        .unwrap();

                        tx.send(
                            UpdateRootInteractions {
                                r#type: UpdateType::Init,
                                maxId: 0,
                                interactions: Some(Dictionary::default()),
                            }
                            .into(),
                        )
                        .await
                        .unwrap();

                        tx.send(
                            UpdateUnarmedInteractions {
                                r#type: UpdateType::Init,
                                interactions: Some(Dictionary::default()),
                            }
                            .into(),
                        )
                        .await
                        .unwrap();

                        tx.send(
                            UpdateBlockSoundSets {
                                r#type: UpdateType::Init,
                                maxId: 0,
                                blockSoundSets: Some(Dictionary::default()),
                            }
                            .into(),
                        )
                        .await
                        .unwrap();

                        tx.send(
                            UpdateRepulsionConfig {
                                r#type: UpdateType::Init,
                                maxId: 0,
                                repulsionConfigs: Some(Dictionary::default()),
                            }
                            .into(),
                        )
                        .await
                        .unwrap();

                        tx.send(
                            UpdateModelvfxs {
                                r#type: UpdateType::Init,
                                maxId: 0,
                                modelVFXs: Some(Dictionary::default()),
                            }
                            .into(),
                        )
                        .await
                        .unwrap();

                        tx.send(
                            UpdateEntityUIComponents {
                                r#type: UpdateType::Init,
                                maxId: 0,
                                components: Some(Dictionary::default()),
                            }
                            .into(),
                        )
                        .await
                        .unwrap();

                        tx.send(
                            UpdateSoundSets {
                                r#type: UpdateType::Init,
                                maxId: 0,
                                soundSets: Some(Dictionary::default()),
                            }
                            .into(),
                        )
                        .await
                        .unwrap();

                        tx.send(
                            UpdateBlockSets {
                                r#type: UpdateType::Init,
                                blockSets: Some(Dictionary::default()),
                            }
                            .into(),
                        )
                        .await
                        .unwrap();

                        tx.send(
                            UpdateRecipes {
                                r#type: UpdateType::Init,
                                recipes: Some(Dictionary::default()),
                                removedRecipes: Some(Vec::new()),
                            }
                            .into(),
                        )
                        .await
                        .unwrap();

                        tx.send(
                            UpdateTagPatterns {
                                r#type: UpdateType::Init,
                                maxId: 0,
                                patterns: Some(Dictionary::default()),
                            }
                            .into(),
                        )
                        .await
                        .unwrap();

                        tx.send(
                            UpdateItemSoundSets {
                                r#type: UpdateType::Init,
                                maxId: 0,
                                itemSoundSets: Some(Dictionary::default()),
                            }
                            .into(),
                        )
                        .await
                        .unwrap();

                        tx.send(
                            UpdateReverbEffects {
                                r#type: UpdateType::Init,
                                maxId: 0,
                                effects: Some(Dictionary::default()),
                            }
                            .into(),
                        )
                        .await
                        .unwrap();

                        tx.send(
                            UpdateInteractions {
                                r#type: UpdateType::Init,
                                maxId: 0,
                                interactions: Some(Dictionary::default()),
                            }
                            .into(),
                        )
                        .await
                        .unwrap();

                        tx.send(
                            WorldLoadProgress {
                                status: Some("Meowing".to_string()),
                                percentComplete: 50,
                                percentCompleteSubitem: 0,
                            }
                            .into(),
                        )
                        .await
                        .unwrap();

                        tx.send(WorldLoadFinished {}.into()).await.unwrap();
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
