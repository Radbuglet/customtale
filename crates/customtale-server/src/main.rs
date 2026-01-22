use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    sync::Arc,
};

use customtale_auth::session::SessionService;
use customtale_protocol::packets::{
    AnyPacket, PacketCategory,
    connection::{Disconnect, DisconnectType},
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

use crate::framed::{HytaleDecoder, HytaleEncoder};

pub mod framed;

#[tokio::main]
async fn main() -> miette::Result<()> {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .unwrap();

    let session_service = SessionService::new()?;

    // TODO: com/hypixel/hytale/server/core/io/transport/QUICTransport.java
    let ssc =
        rcgen::generate_simple_self_signed(vec!["localhost".to_string()]).into_diagnostic()?;

    let cert_der = CertificateDer::from(ssc.cert);
    let key = PrivatePkcs8KeyDer::from(ssc.signing_key.serialize_der());

    let mut tls_server_config =
        rustls::ServerConfig::builder_with_provider(CryptoProvider::get_default().unwrap().clone())
            .with_protocol_versions(&[&rustls::version::TLS13])
            .unwrap()
            .with_no_client_auth()
            .with_single_cert(vec![cert_der], key.into())
            .unwrap();

    tls_server_config.alpn_protocols = vec![b"hytale/1".to_vec()];

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

            let Some(packet) = rx.next().await else {
                return;
            };
            let AnyPacket::Connect(packet) = packet.unwrap() else {
                panic!("what?");
            };

            tx.send(AnyPacket::Disconnect(Disconnect {
                reason: Some(format!("Welcome to Customtale, {}", packet.username)),
                type_: DisconnectType::Disconnect,
            }))
            .await
            .unwrap();

            tx.get_mut().finish().unwrap();
            tx.get_mut().stopped().await.unwrap();
        });
    }

    Ok(())
}
