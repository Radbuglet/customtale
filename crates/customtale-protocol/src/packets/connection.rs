use crate::{
    packets::{Packet, PacketCategory, PacketDescriptor},
    serde::{
        Codec, EnumCodec, ErasedCodec, FixedSizeStringCodec, LeU16Codec, LeU32Codec,
        NulTerminatedStringCodec, Serde, StructCodec, VarByteArrayCodec, VarStringCodec, field,
    },
};
use bytes::Bytes;
use enum_ordinalize::Ordinalize;
use uuid::Uuid;

#[derive(Debug, Clone, Default)]
pub struct Connect {
    pub protocol_crc: u32,
    pub protocol_build_number: u32,
    pub client_version: String,
    pub client_type: ClientType,
    pub uuid: Uuid,
    pub username: String,
    pub identity_token: Option<String>,
    pub language: String,
    pub referral_data: Option<Bytes>,
    pub referral_source: Option<HostAddress>,
}

impl Packet for Connect {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "connect",
        id: 0,
        is_compressed: false,
        max_size: 38013,
        category: PacketCategory::CONNECTION,
    };
}

impl Serde for Connect {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            LeU32Codec
                .field(field![Connect, protocol_crc])
                .named("protocol_crc"),
            LeU32Codec
                .field(field![Connect, protocol_build_number])
                .named("protocol_build_number"),
            FixedSizeStringCodec::new(20)
                .field(field!(Connect, client_version))
                .named("client_version"),
            ClientType::codec()
                .field(field![Connect, client_type])
                .named("client_type"),
            Uuid::codec().field(field!(Connect, uuid)).named("uuid"),
            VarStringCodec::new(16)
                .field(field![Connect, username])
                .named("username"),
            VarStringCodec::new(8192)
                .nullable_variable()
                .field(field![Connect, identity_token])
                .named("identity_token"),
            VarStringCodec::new(16)
                .field(field![Connect, language])
                .named("language"),
            VarByteArrayCodec::new(4096)
                .nullable_variable()
                .field(field![Connect, referral_data])
                .named("referral_data"),
            HostAddress::codec()
                .nullable_variable()
                .field(field![Connect, referral_source])
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
                .field(field![HostAddress, host])
                .named("host"),
            LeU16Codec.field(field![HostAddress, port]).named("port"),
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
                .field(field![Disconnect, reason])
                .named("reason"),
            DisconnectType::codec()
                .field(field![Disconnect, type_])
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
