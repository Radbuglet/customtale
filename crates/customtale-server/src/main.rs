use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    sync::Arc,
};

use bytes::{Buf as _, BufMut, Bytes, BytesMut};
use customtale_auth::{oauth::OauthFlow, session::SessionService};
use customtale_protocol::{packets, serde::Serde as _};
use miette::IntoDiagnostic;
use quinn::{
    crypto::rustls::QuicServerConfig,
    rustls::{
        self,
        pki_types::{CertificateDer, PrivatePkcs8KeyDer},
    },
};
use rustls::crypto::CryptoProvider;

#[tokio::main]
async fn main() -> miette::Result<()> {
    let session_service = SessionService::new()?;
    let flow = OauthFlow::start(session_service.clone()).await?;

    dbg!(flow.auth_url());

    let code = flow.finished().await?;

    dbg!(&code);

    Ok(())
}

async fn main_old() -> miette::Result<()> {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .unwrap();

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

            let (mut tx, mut rx) = conn.accept_bi().await.unwrap();

            // com/hypixel/hytale/server/core/io/netty/HytaleChannelInitializer.java
            // com/hypixel/hytale/protocol/io/netty/PacketDecoder.java
            // com/hypixel/hytale/protocol/io/netty/PacketEncoder.java

            let mut buffer = BytesMut::new();
            let mut buffer_len = 0;
            buffer.resize(4096, 0);

            loop {
                buffer_len += rx.read(&mut buffer[buffer_len..]).await.unwrap().unwrap();
                let mut packet = &buffer[..buffer_len];

                if packet.len() < 8 {
                    continue;
                }

                let packet_len = packet.get_u32_le() as usize;
                let packet_id = packet.get_u32_le();

                if packet.len() < packet_len {
                    continue;
                }

                dbg!(packet_id, packet_len);

                if packet_id == 0 {
                    // com/hypixel/hytale/server/core/io/handlers/InitialPacketHandler.java
                    match packets::connection::Connect::decode(Bytes::copy_from_slice(packet)) {
                        Ok(packet) => {
                            dbg!(&packet);

                            let mut output = BytesMut::new();
                            output.put_u32_le(0); // size
                            output.put_u32_le(1); // packetId

                            packets::connection::Disconnect {
                                reason: Some(format!(
                                    "Hello from Customtale, {}!",
                                    packet.username,
                                )),
                                type_: packets::connection::DisconnectType::Disconnect,
                            }
                            .encode(&mut output)
                            .unwrap();

                            let output_len = (output.len() - 8) as u32;
                            output[0..4].copy_from_slice(&output_len.to_le_bytes());

                            dbg!(&output);

                            tx.write_all(&output).await.unwrap();
                            tx.finish().unwrap();
                            tx.stopped().await.unwrap();
                            return;
                        }
                        Err(err) => panic!("{err:#}"),
                    }
                }

                buffer.advance(packet_len);
            }
        });
    }

    Ok(())
}
