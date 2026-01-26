use std::collections::HashMap;

use bytes::Bytes;

use crate::{
    codec,
    packets::{Packet, PacketCategory, PacketDescriptor},
    serde::{Codec, FixedSizeStringCodec, VarByteArrayCodec, VarStringCodec},
};

// === Packets === //

codec! {
    pub struct AssetFinalize {}
}

impl Packet for AssetFinalize {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "AssetFinalize",
        id: 26,
        is_compressed: false,
        max_size: 0,
        category: PacketCategory::SETUP,
    };
}

codec! {
    pub struct AssetInitialize {
        pub asset: Asset,
        pub size: u32,
    }
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

codec! {
    pub struct PlayerOptions {
        pub skin: Option<Box<PlayerSkin>>
        => PlayerSkin::codec()
            .boxed()
            .nullable_variable(),
    }
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

codec! {
    pub struct RemoveAssets {
        pub assets: Option<Vec<Asset>>,
    }
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

codec! {
    pub struct RequestAssets {
        pub assets: Option<Vec<Asset>>,
    }
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

codec! {
    pub struct RequestCommonAssetsRebuild {}
}

impl Packet for RequestCommonAssetsRebuild {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "RequestCommonAssetsRebuild",
        id: 28,
        is_compressed: false,
        max_size: 0,
        category: PacketCategory::SETUP,
    };
}

codec! {
    pub struct ServerTags {
        pub tags: Option<HashMap<String, u32>>,
    }
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

codec! {
    pub struct SetTimeDilation {
        pub time_dilation: f32,
    }
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

codec! {
    pub struct SetUpdateRate {
        pub updates_per_second: u32,
    }
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

codec! {
    pub struct UpdateFeatures {
        pub features: Option<HashMap<ClientFeature, bool>>,
    }
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

codec! {
    pub struct ViewRadius {
        pub value: u32,
    }
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

codec! {
    pub struct WorldLoadProgress {
        pub status: Option<String>,
        pub percent_complete: u32,
        pub percent_complete_subitem: u32,
    }
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

codec! {
    pub struct WorldLoadFinished {}
}

impl Packet for WorldLoadFinished {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "WorldLoadFinished",
        id: 22,
        is_compressed: false,
        max_size: 0,
        category: PacketCategory::SETUP,
    };
}

codec! {
    pub struct WorldSettings {
        pub world_height: u32,
        pub required_assets: Option<Vec<Asset>>,
    }
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

// === Data types === //

codec! {
    pub enum ClientFeature {
        SplitVelocity,
        Mantling,
        SprintForce,
        CrouchSlide,
        SafetyRoll,
        DisplayHealthBars,
        DisplayCombatText,
        CanHideHelmet,
        CanHideCuirass,
        CanHideGauntlets,
        CanHidePants,
    }

    pub struct Asset {
        pub hash: String => FixedSizeStringCodec::new(64),
        pub name: String => VarStringCodec::new(512),
    }

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

    pub struct AssetPart {
        pub part: Option<Bytes> => VarByteArrayCodec::new(4096000).nullable_variable(),
    }
}
