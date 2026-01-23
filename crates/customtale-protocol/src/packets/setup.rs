use crate::{
    field,
    packets::{Packet, PacketCategory, PacketDescriptor},
    serde::{
        Codec, ErasedCodec, FixedSizeStringCodec, LeU32, Serde, StructCodec, VarArrayCodec,
        VarStringCodec,
    },
};

#[derive(Debug, Clone, Default)]
pub struct WorldSettings {
    pub world_height: u32,
    pub required_assets: Option<Vec<Asset>>,
}

impl Packet for WorldSettings {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "WorldSettings",
        id: 20,
        is_compressed: true,
        max_size: 1677721600,
        category: PacketCategory::SETUP,
    };
}

impl Serde for WorldSettings {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            LeU32
                .map(field![WorldSettings, world_height])
                .named("world_height"),
            VarArrayCodec::new(Asset::codec(), 4096000)
                .nullable_variable()
                .map(field![WorldSettings, required_assets])
                .named("required_assets"),
        ])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Asset {
    pub hash: String,
    pub name: String,
}

impl Serde for Asset {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            FixedSizeStringCodec::new(64)
                .map(field![Asset, hash])
                .named("hash"),
            VarStringCodec::new(512)
                .map(field![Asset, name])
                .named("name"),
        ])
        .erase()
    }
}
