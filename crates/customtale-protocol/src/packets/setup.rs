use std::collections::HashMap;

use bytes::Bytes;
use enum_ordinalize::Ordinalize;

use crate::{
    field,
    packets::{Packet, PacketCategory, PacketDescriptor},
    serde::{
        ByteBoolCodec, Codec, EnumCodec, ErasedCodec, FixedSizeStringCodec, LeF64Codec, LeU32Codec,
        Serde, StructCodec, VarArrayCodec, VarByteArrayCodec, VarDictionaryCodec, VarStringCodec,
    },
};

// === Packets === //

#[derive(Debug, Clone, Default)]
pub struct AssetFinalize;

impl Packet for AssetFinalize {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "AssetFinalize",
        id: 26,
        is_compressed: false,
        max_size: 0,
        category: PacketCategory::SETUP,
    };
}

impl Serde for AssetFinalize {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([]).erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct AssetInitialize {
    pub asset: Asset,
    pub size: u32,
}

impl Packet for AssetInitialize {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "AssetInitialize",
        id: 24,
        is_compressed: false,
        max_size: 2121,
        category: PacketCategory::SETUP,
    };
}

impl Serde for AssetInitialize {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            Asset::codec()
                .field(field![AssetInitialize, asset])
                .named("asset"),
            LeU32Codec
                .field(field![AssetInitialize, size])
                .named("size"),
        ])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct PlayerOptions {
    pub skin: Option<Box<PlayerSkin>>,
}

impl Packet for PlayerOptions {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "PlayerOptions",
        id: 33,
        is_compressed: false,
        max_size: 327680184,
        category: PacketCategory::SETUP,
    };
}

impl Serde for PlayerOptions {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([PlayerSkin::codec()
            .boxed()
            .nullable_variable()
            .field(field![PlayerOptions, skin])
            .named("skin")])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct RemoveAssets {
    pub assets: Option<Vec<Asset>>,
}

impl Packet for RemoveAssets {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "RemoveAssets",
        id: 27,
        is_compressed: false,
        max_size: 1677721600,
        category: PacketCategory::SETUP,
    };
}

impl Serde for RemoveAssets {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([VarArrayCodec::new(Asset::codec(), 4096000)
            .nullable_variable()
            .field(field![RemoveAssets, assets])
            .named("assets")])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct RequestAssets {
    pub assets: Option<Vec<Asset>>,
}

impl Packet for RequestAssets {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "RequestAssets",
        id: 23,
        is_compressed: true,
        max_size: 1677721600,
        category: PacketCategory::SETUP,
    };
}

impl Serde for RequestAssets {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([VarArrayCodec::new(Asset::codec(), 4096000)
            .nullable_variable()
            .field(field![RequestAssets, assets])
            .named("assets")])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct RequestCommonAssetsRebuild;

impl Packet for RequestCommonAssetsRebuild {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "RequestCommonAssetsRebuild",
        id: 28,
        is_compressed: false,
        max_size: 0,
        category: PacketCategory::SETUP,
    };
}

impl Serde for RequestCommonAssetsRebuild {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([]).erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct ServerTags {
    pub tags: Option<HashMap<String, u32>>,
}

impl Packet for ServerTags {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "ServerTags",
        id: 34,
        is_compressed: false,
        max_size: 1677721600,
        category: PacketCategory::SETUP,
    };
}

impl Serde for ServerTags {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([VarDictionaryCodec::new(
            VarStringCodec::new(4096000).erase(),
            LeU32Codec.erase(),
            4096000,
        )
        .nullable_variable()
        .field(field![ServerTags, tags])
        .named("tags")])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct SetTimeDilation {
    pub time_dilation: f64,
}

impl Packet for SetTimeDilation {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "SetTimeDilation",
        id: 30,
        is_compressed: false,
        max_size: 4,
        category: PacketCategory::SETUP,
    };
}

impl Serde for SetTimeDilation {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([LeF64Codec
            .field(field![SetTimeDilation, time_dilation])
            .named("time_dilation")])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct SetUpdateRate {
    pub updates_per_second: u32,
}

impl Packet for SetUpdateRate {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "SetUpdateRate",
        id: 29,
        is_compressed: false,
        max_size: 4,
        category: PacketCategory::SETUP,
    };
}

impl Serde for SetUpdateRate {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([LeU32Codec
            .field(field![SetUpdateRate, updates_per_second])
            .named("updates_per_second")])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct UpdateFeatures {
    pub features: Option<HashMap<ClientFeature, bool>>,
}

impl Packet for UpdateFeatures {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "UpdateFeatures",
        id: 31,
        is_compressed: false,
        max_size: 8192006,
        category: PacketCategory::SETUP,
    };
}

impl Serde for UpdateFeatures {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([VarDictionaryCodec::new(
            ClientFeature::codec(),
            ByteBoolCodec.erase(),
            4096000,
        )
        .nullable_variable()
        .field(field![UpdateFeatures, features])
        .named("features")])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct ViewRadius {
    pub value: u32,
}

impl Packet for ViewRadius {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "ViewRadius",
        id: 32,
        is_compressed: false,
        max_size: 4,
        category: PacketCategory::SETUP,
    };
}

impl Serde for ViewRadius {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([LeU32Codec.field(field![ViewRadius, value]).named("value")]).erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct WorldLoadProgress {
    pub status: Option<String>,
    pub percent_complete: u32,
    pub percent_complete_subitem: u32,
}

impl Packet for WorldLoadProgress {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "WorldLoadProgress",
        id: 21,
        is_compressed: false,
        max_size: 16384014,
        category: PacketCategory::SETUP,
    };
}

impl Serde for WorldLoadProgress {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![WorldLoadProgress, status])
                .named("status"),
            LeU32Codec
                .field(field![WorldLoadProgress, percent_complete])
                .named("percent_complete"),
            LeU32Codec
                .field(field![WorldLoadProgress, percent_complete_subitem])
                .named("percent_complete_subitem"),
        ])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct WorldLoadFinished;

impl Packet for WorldLoadFinished {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "WorldLoadFinished",
        id: 22,
        is_compressed: false,
        max_size: 0,
        category: PacketCategory::SETUP,
    };
}

impl Serde for WorldLoadFinished {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([]).erase()
    }
}

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
            LeU32Codec
                .field(field![WorldSettings, world_height])
                .named("world_height"),
            VarArrayCodec::new(Asset::codec(), 4096000)
                .nullable_variable()
                .field(field![WorldSettings, required_assets])
                .named("required_assets"),
        ])
        .erase()
    }
}

// === Data types === //

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ordinalize, Default)]
#[repr(u8)]
pub enum ClientFeature {
    #[default]
    SplitVelocity,
    Mantling,
    SprintForce,
    CrouchSlide,
    SafetyRoll,
    DisplayHealthBars,
    DisplayCombatText,
}

impl Serde for ClientFeature {
    fn build_codec() -> ErasedCodec<Self> {
        EnumCodec::new().erase()
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
                .field(field![Asset, hash])
                .named("hash"),
            VarStringCodec::new(512)
                .field(field![Asset, name])
                .named("name"),
        ])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct PlayerSkin {
    pub body_characteristic: Option<String>,
    pub underwear: Option<String>,
    pub face: Option<String>,
    pub eyes: Option<String>,
    pub ears: Option<String>,
    pub mouth: Option<String>,
    pub facial_hair: Option<String>,
    pub haircut: Option<String>,
    pub eyebrows: Option<String>,
    pub pants: Option<String>,
    pub overpants: Option<String>,
    pub undertop: Option<String>,
    pub overtop: Option<String>,
    pub shoes: Option<String>,
    pub head_accessory: Option<String>,
    pub face_accessory: Option<String>,
    pub ear_accessory: Option<String>,
    pub skin_feature: Option<String>,
    pub gloves: Option<String>,
    pub cape: Option<String>,
}

impl Serde for PlayerSkin {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![PlayerSkin, body_characteristic])
                .named("body_characteristic"),
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![PlayerSkin, underwear])
                .named("underwear"),
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![PlayerSkin, face])
                .named("face"),
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![PlayerSkin, eyes])
                .named("eyes"),
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![PlayerSkin, ears])
                .named("ears"),
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![PlayerSkin, mouth])
                .named("mouth"),
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![PlayerSkin, facial_hair])
                .named("facial_hair"),
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![PlayerSkin, haircut])
                .named("haircut"),
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![PlayerSkin, eyebrows])
                .named("eyebrows"),
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![PlayerSkin, pants])
                .named("pants"),
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![PlayerSkin, overpants])
                .named("overpants"),
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![PlayerSkin, undertop])
                .named("undertop"),
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![PlayerSkin, overtop])
                .named("overtop"),
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![PlayerSkin, shoes])
                .named("shoes"),
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![PlayerSkin, head_accessory])
                .named("head_accessory"),
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![PlayerSkin, face_accessory])
                .named("face_accessory"),
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![PlayerSkin, ear_accessory])
                .named("ear_accessory"),
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![PlayerSkin, skin_feature])
                .named("skin_feature"),
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![PlayerSkin, gloves])
                .named("gloves"),
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![PlayerSkin, cape])
                .named("cape"),
        ])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct AssetPart {
    pub part: Option<Bytes>,
}

impl Serde for AssetPart {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([VarByteArrayCodec::new(4096000)
            .nullable_variable()
            .field(field![AssetPart, part])
            .named("part")])
        .erase()
    }
}
