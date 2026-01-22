use crate::{
    packets::{Packet, PacketCategory, PacketDescriptor},
    serde::{
        Codec, EnumCodec, ErasedCodec, FixedByteArray, LeU16, NulTerminatedStringCodec, Serde,
        StructCodec, VarByteArrayCodec, VarStringCodec, field,
    },
};
use bytes::Bytes;
use enum_ordinalize::Ordinalize;
use uuid::Uuid;

#[derive(Debug, Clone, Default)]
pub struct Connect {
    pub protocol_hash: FixedByteArray<64>,
    pub client_type: ClientType,
    pub language: Option<String>,
    pub identity_token: Option<String>,
    pub uuid: Uuid,
    pub username: String,
    pub referral_data: Option<Bytes>,
    pub referral_source: Option<HostAddress>,
}

impl Packet for Connect {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "connect",
        id: 0,
        is_compressed: false,
        max_size: 38161,
        category: PacketCategory::CONNECTION,
    };
}

impl Serde for Connect {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            FixedByteArray::codec()
                .map(field!(Connect, protocol_hash))
                .named("protocol_hash"),
            ClientType::codec()
                .map(field![Connect, client_type])
                .named("client_type"),
            VarStringCodec::new(128)
                .nullable_variable()
                .map(field![Connect, language])
                .named("language"),
            VarStringCodec::new(8192)
                .nullable_variable()
                .map(field![Connect, identity_token])
                .named("identity_token"),
            Uuid::codec().map(field!(Connect, uuid)).named("uuid"),
            VarStringCodec::new(4096)
                .map(field![Connect, username])
                .named("username"),
            VarByteArrayCodec::new(4096)
                .nullable_variable()
                .map(field![Connect, referral_data])
                .named("referral_data"),
            HostAddress::codec()
                .nullable_variable()
                .map(field![Connect, referral_source])
                .named("referral_source"),
        ])
        .erase()
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ordinalize, Default)]
#[repr(u8)]
pub enum ClientType {
    #[default]
    Game,
    Editor,
}

impl Serde for ClientType {
    fn build_codec() -> ErasedCodec<Self> {
        EnumCodec::default().erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct HostAddress {
    pub host: String,
    pub port: u16,
}

impl Serde for HostAddress {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            NulTerminatedStringCodec::new(256)
                .map(field![HostAddress, host])
                .named("host"),
            LeU16.map(field![HostAddress, port]).named("port"),
        ])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Disconnect {
    pub reason: Option<String>,
    pub type_: DisconnectType,
}

impl Packet for Disconnect {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "disconnect",
        id: 1,
        is_compressed: false,
        max_size: 16384007,
        category: PacketCategory::CONNECTION,
    };
}

impl Serde for Disconnect {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            VarStringCodec::new(4096000)
                .nullable_variable()
                .map(field![Disconnect, reason])
                .named("reason"),
            DisconnectType::codec()
                .map(field![Disconnect, type_])
                .named("type"),
        ])
        .erase()
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ordinalize, Default)]
#[repr(u8)]
pub enum DisconnectType {
    #[default]
    Disconnect,
    Crash,
}

impl Serde for DisconnectType {
    fn build_codec() -> ErasedCodec<Self> {
        EnumCodec::default().erase()
    }
}
