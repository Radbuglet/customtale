use bytes::Bytes;

use crate::{
    field,
    packets::{Packet, PacketCategory, PacketDescriptor},
    serde::{Codec as _, ErasedCodec, Serde, StructCodec, VarByteArrayCodec, VarStringCodec},
};

#[derive(Debug, Clone, Default)]
pub struct AuthGrant {
    pub authorization_grant: Option<String>,
    pub server_identity_token: Option<String>,
}

impl Packet for AuthGrant {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "AuthGrant",
        id: 11,
        is_compressed: false,
        max_size: 49171,
        category: PacketCategory::AUTH,
    };
}

impl Serde for AuthGrant {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            VarStringCodec::new(4096)
                .nullable_variable()
                .map(field![AuthGrant, authorization_grant])
                .named("authorization_grant"),
            VarStringCodec::new(8192)
                .nullable_variable()
                .map(field![AuthGrant, server_identity_token])
                .named("server_identity_token"),
        ])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct AuthToken {
    pub access_token: Option<String>,
    pub server_authorization_grant: Option<String>,
}

impl Packet for AuthToken {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "AuthToken",
        id: 12,
        is_compressed: false,
        max_size: 49171,
        category: PacketCategory::AUTH,
    };
}

impl Serde for AuthToken {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            VarStringCodec::new(8192)
                .nullable_variable()
                .map(field![AuthToken, access_token])
                .named("auth_token"),
            VarStringCodec::new(4096)
                .nullable_variable()
                .map(field![AuthToken, server_authorization_grant])
                .named("server_authorization_grant"),
        ])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct ConnectAccept {
    pub password_challenge: Option<Bytes>,
}

impl Packet for ConnectAccept {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "ConnectAuth",
        id: 14,
        is_compressed: false,
        max_size: 70,
        category: PacketCategory::AUTH,
    };
}

impl Serde for ConnectAccept {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([VarByteArrayCodec::new(64)
            .nullable_variable()
            .map(field![ConnectAccept, password_challenge])
            .named("password_challenge")])
        .erase()
    }
}
