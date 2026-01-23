use crate::{
    field,
    packets::{Packet, PacketCategory, PacketDescriptor},
    serde::{Codec as _, ErasedCodec, Serde, StructCodec, VarStringCodec},
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
pub struct ServerAuthToken {
    pub server_access_token: Option<String>,
    pub password_challenge: Option<String>,
}

impl Packet for ServerAuthToken {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "ServerAuthToken",
        id: 13,
        is_compressed: false,
        max_size: 32851,
        category: PacketCategory::AUTH,
    };
}

impl Serde for ServerAuthToken {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            VarStringCodec::new(8192)
                .nullable_variable()
                .map(field![ServerAuthToken, server_access_token])
                .named("server_access_token"),
            VarStringCodec::new(64)
                .nullable_variable()
                .map(field![ServerAuthToken, password_challenge])
                .named("password_challenge"),
        ])
        .erase()
    }
}
