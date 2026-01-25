use std::collections::HashMap;

use enum_ordinalize::Ordinalize;
use uuid::Uuid;

use crate::{
    codec,
    data::{
        Color, ColorLight, Direction, FloatRange, NearFar, Range, RangeVector2f, Rangeb, Rangef,
        Vector2f, Vector3f, Vector3i,
    },
    field,
    packets::{Packet, PacketCategory, PacketDescriptor},
    serde::{
        ByteBoolCodec, Codec, EnumCodec, ErasedCodec, LeF32Codec, LeU32Codec, Serde, StructCodec,
        VarArrayCodec, VarDictionaryCodec, VarStringCodec,
    },
};

// === Packets === //

#[derive(Debug, Clone, Default)]
pub struct TrackOrUpdateObjective {
    pub objective: Option<Objective>,
}

impl Packet for TrackOrUpdateObjective {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "TrackOrUpdateObjective",
        id: 69,
        is_compressed: false,
        max_size: 1677721600,
        category: PacketCategory::ASSETS,
    };
}

impl Serde for TrackOrUpdateObjective {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([Objective::codec()
            .nullable_variable()
            .field(field![TrackOrUpdateObjective, objective])
            .named("objective")])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct UntrackObjective {
    pub objective_uuid: Uuid,
}

impl Packet for UntrackObjective {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "UntrackObjective",
        id: 70,
        is_compressed: false,
        max_size: 16,
        category: PacketCategory::ASSETS,
    };
}

impl Serde for UntrackObjective {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([Uuid::codec()
            .field(field![UntrackObjective, objective_uuid])
            .named("objective_uuid")])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct UpdateAmbienceFX {
    pub type_: UpdateType,
    pub max_id: u32,
    pub ambience_fx: Option<HashMap<u32, AmbienceFx>>,
}

impl Packet for UpdateAmbienceFX {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "UpdateAmbienceFX",
        id: 62,
        is_compressed: true,
        max_size: 1677721600,
        category: PacketCategory::ASSETS,
    };
}

impl Serde for UpdateAmbienceFX {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            UpdateType::codec()
                .field(field![UpdateAmbienceFX, type_])
                .named("type_"),
            LeU32Codec
                .field(field![UpdateAmbienceFX, max_id])
                .named("max_id"),
            VarDictionaryCodec::new(LeU32Codec.erase(), AmbienceFx::codec(), 4096000)
                .nullable_variable()
                .field(field![UpdateAmbienceFX, ambience_fx])
                .named("ambience_fx"),
        ])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct UpdateAudioCategories {
    pub type_: UpdateType,
    pub max_id: u32,
    pub categories: Option<HashMap<u32, AudioCategory>>,
}

impl Packet for UpdateAudioCategories {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "UpdateAudioCategories",
        id: 80,
        is_compressed: true,
        max_size: 1677721600,
        category: PacketCategory::ASSETS,
    };
}

impl Serde for UpdateAudioCategories {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            UpdateType::codec()
                .field(field![UpdateAudioCategories, type_])
                .named("type_"),
            LeU32Codec
                .field(field![UpdateAudioCategories, max_id])
                .named("max_id"),
            VarDictionaryCodec::new(LeU32Codec.erase(), AudioCategory::codec(), 4096000)
                .nullable_variable()
                .field(field![UpdateAudioCategories, categories])
                .named("categories"),
        ])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct UpdateBlockBreakingDecals {
    pub type_: UpdateType,
    pub block_breaking_decals: Option<HashMap<String, BlockBreakingDecal>>,
}

impl Packet for UpdateBlockBreakingDecals {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "UpdateBlockBreakingDecals",
        id: 45,
        is_compressed: true,
        max_size: 1677721600,
        category: PacketCategory::ASSETS,
    };
}

impl Serde for UpdateBlockBreakingDecals {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            UpdateType::codec()
                .field(field![UpdateBlockBreakingDecals, type_])
                .named("type_"),
            VarDictionaryCodec::new(
                VarStringCodec::new(4096000).erase(),
                BlockBreakingDecal::codec(),
                4096000,
            )
            .nullable_variable()
            .field(field![UpdateBlockBreakingDecals, block_breaking_decals])
            .named("block_breaking_decals"),
        ])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct UpdateBlockGroups {
    pub type_: UpdateType,
    pub groups: Option<HashMap<String, BlockGroup>>,
}

impl Packet for UpdateBlockGroups {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "UpdateBlockGroups",
        id: 78,
        is_compressed: true,
        max_size: 1677721600,
        category: PacketCategory::ASSETS,
    };
}

impl Serde for UpdateBlockGroups {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            UpdateType::codec()
                .field(field![UpdateBlockGroups, type_])
                .named("type_"),
            VarDictionaryCodec::new(
                VarStringCodec::new(4096000).erase(),
                BlockGroup::codec(),
                4096000,
            )
            .nullable_variable()
            .field(field![UpdateBlockGroups, groups])
            .named("groups"),
        ])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct UpdateBlockHitboxes {
    pub type_: UpdateType,
    pub max_id: u32,
    pub block_base_hitboxes: Option<HashMap<u32, Vec<Hitbox>>>,
}

impl Packet for UpdateBlockHitboxes {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "UpdateBlockHitboxes",
        id: 41,
        is_compressed: true,
        max_size: 1677721600,
        category: PacketCategory::ASSETS,
    };
}

impl Serde for UpdateBlockHitboxes {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            UpdateType::codec()
                .field(field![UpdateBlockHitboxes, type_])
                .named("type_"),
            LeU32Codec
                .field(field![UpdateBlockHitboxes, max_id])
                .named("max_id"),
            VarDictionaryCodec::new(
                LeU32Codec.erase(),
                VarArrayCodec::new(Hitbox::codec(), 4096000).erase(),
                4096000,
            )
            .nullable_variable()
            .field(field![UpdateBlockHitboxes, block_base_hitboxes])
            .named("categories"),
        ])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct UpdateBlockParticleSets {
    pub type_: UpdateType,
    pub block_particle_sets: Option<HashMap<String, BlockParticleSet>>,
}

impl Packet for UpdateBlockParticleSets {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "UpdateBlockParticleSets",
        id: 44,
        is_compressed: true,
        max_size: 1677721600,
        category: PacketCategory::ASSETS,
    };
}

impl Serde for UpdateBlockParticleSets {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            UpdateType::codec()
                .field(field![UpdateBlockParticleSets, type_])
                .named("type_"),
            VarDictionaryCodec::new(
                VarStringCodec::new(4096000).erase(),
                BlockParticleSet::codec(),
                4096000,
            )
            .nullable_variable()
            .field(field![UpdateBlockParticleSets, block_particle_sets])
            .named("block_particle_sets"),
        ])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct UpdateBlockSets {
    pub type_: UpdateType,
    pub block_sets: Option<HashMap<String, BlockSet>>,
}

impl Packet for UpdateBlockSets {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "UpdateBlockSets",
        id: 46,
        is_compressed: true,
        max_size: 1677721600,
        category: PacketCategory::ASSETS,
    };
}

impl Serde for UpdateBlockSets {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            UpdateType::codec()
                .field(field![UpdateBlockSets, type_])
                .named("type_"),
            VarDictionaryCodec::new(
                VarStringCodec::new(4096000).erase(),
                BlockSet::codec(),
                4096000,
            )
            .nullable_variable()
            .field(field![UpdateBlockSets, block_sets])
            .named("block_sets"),
        ])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct UpdateBlockSoundSets {
    pub type_: UpdateType,
    pub max_id: u32,
    pub block_sound_sets: Option<HashMap<u32, BlockSoundSet>>,
}

impl Packet for UpdateBlockSoundSets {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "UpdateBlockSoundSets",
        id: 42,
        is_compressed: true,
        max_size: 1677721600,
        category: PacketCategory::ASSETS,
    };
}

impl Serde for UpdateBlockSoundSets {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            UpdateType::codec()
                .field(field![UpdateBlockSoundSets, type_])
                .named("type_"),
            LeU32Codec
                .field(field![UpdateBlockSoundSets, max_id])
                .named("max_id"),
            VarDictionaryCodec::new(LeU32Codec.erase(), BlockSoundSet::codec(), 4096000)
                .nullable_variable()
                .field(field![UpdateBlockSoundSets, block_sound_sets])
                .named("block_sound_sets"),
        ])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct UpdateBlockTypes {
    pub type_: UpdateType,
    pub max_id: u32,
    pub block_types: Option<HashMap<u32, BlockType>>,
    pub update_block_textures: bool,
    pub update_model_textures: bool,
    pub update_models: bool,
    pub update_map_geometry: bool,
}

impl Packet for UpdateBlockTypes {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "UpdateBlockTypes",
        id: 40,
        is_compressed: true,
        max_size: 1677721600,
        category: PacketCategory::ASSETS,
    };
}

impl Serde for UpdateBlockTypes {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            UpdateType::codec()
                .field(field![UpdateBlockTypes, type_])
                .named("type_"),
            LeU32Codec
                .field(field![UpdateBlockTypes, max_id])
                .named("max_id"),
            VarDictionaryCodec::new(LeU32Codec.erase(), BlockType::codec(), 4096000)
                .nullable_variable()
                .field(field![UpdateBlockTypes, block_types])
                .named("block_sound_sets"),
            ByteBoolCodec
                .field(field![UpdateBlockTypes, update_block_textures])
                .named("update_block_textures"),
            ByteBoolCodec
                .field(field![UpdateBlockTypes, update_model_textures])
                .named("update_model_textures"),
            ByteBoolCodec
                .field(field![UpdateBlockTypes, update_models])
                .named("update_models"),
            ByteBoolCodec
                .field(field![UpdateBlockTypes, update_map_geometry])
                .named("update_map_geometry"),
        ])
        .erase()
    }
}

codec! {
    pub struct UpdateCameraShake {
        pub type_: UpdateType,
        pub profiles: Option<HashMap<u32, CameraShake>>,
    }
}

impl Packet for UpdateCameraShake {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "UpdateCameraShake",
        id: 77,
        is_compressed: true,
        max_size: 1677721600,
        category: PacketCategory::ASSETS,
    };
}

codec! {
    pub struct UpdateEntityEffects {
        pub type_: UpdateType,
        pub max_id: u32,
        pub entity_effects: Option<HashMap<u32, EntityEffect>>,
    }
}

impl Packet for UpdateEntityEffects {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "UpdateEntityEffects",
        id: 51,
        is_compressed: true,
        max_size: 1677721600,
        category: PacketCategory::ASSETS,
    };
}

codec! {
    pub struct UpdateEntityStatTypes {
        pub type_: UpdateType,
        pub max_id: u32,
        pub types: Option<HashMap<u32, EntityStateType>>,
    }
}

impl Packet for UpdateEntityStatTypes {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "UpdateEntityStatTypes",
        id: 72,
        is_compressed: true,
        max_size: 1677721600,
        category: PacketCategory::ASSETS,
    };
}

codec! {
    pub struct UpdateEntityUiComponents {
        pub type_: UpdateType,
        pub max_id: u32,
        pub components: Option<HashMap<u32, EntityUiComponent>>,
    }
}

impl Packet for UpdateEntityUiComponents {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "UpdateEntityUiComponents",
        id: 73,
        is_compressed: true,
        max_size: 1677721600,
        category: PacketCategory::ASSETS,
    };
}

codec! {
    pub struct UpdateEnvironments {
        pub type_: UpdateType,
        pub max_id: u32,
        pub environments: Option<HashMap<u32, WorldEnvironment>>,
        pub rebuild_map_geometry: bool,
    }
}

impl Packet for UpdateEnvironments {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "UpdateEnvironments",
        id: 61,
        is_compressed: true,
        max_size: 1677721600,
        category: PacketCategory::ASSETS,
    };
}

codec! {
    pub struct UpdateEqualizerEffects {
        pub type_: UpdateType,
        pub max_id: u32,
        pub effects: Option<HashMap<u32, EqualizerEffect>>,
    }
}

impl Packet for UpdateEqualizerEffects {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "UpdateEqualizerEffects",
        id: 82,
        is_compressed: true,
        max_size: 1677721600,
        category: PacketCategory::ASSETS,
    };
}

codec! {
    pub struct UpdateFieldcraftCategories {
        pub type_: UpdateType,
        pub item_categories: Option<Vec<ItemCategory>>,
    }
}

impl Packet for UpdateFieldcraftCategories {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "UpdateFieldcraftCategories",
        id: 58,
        is_compressed: true,
        max_size: 1677721600,
        category: PacketCategory::ASSETS,
    };
}

codec! {
    pub struct UpdateFluidFx {
        pub type_: UpdateType,
        pub max_id: u32,
        pub fluid_fx: Option<HashMap<u32, FluidFx>>,
    }
}

impl Packet for UpdateFluidFx {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "UpdateFluidFx",
        id: 63,
        is_compressed: true,
        max_size: 1677721600,
        category: PacketCategory::ASSETS,
    };
}

codec! {
    pub struct UpdateFluids {
        pub type_: UpdateType,
        pub max_id: u32,
        pub fluids: Option<HashMap<u32, Fluid>>,
    }
}

impl Packet for UpdateFluids {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "UpdateFluids",
        id: 83,
        is_compressed: true,
        max_size: 1677721600,
        category: PacketCategory::ASSETS,
    };
}

codec! {
    pub struct UpdateHitboxCollisionConfig {
        pub type_: UpdateType,
        pub max_id: u32,
        pub hitbox_collision_configs: Option<HashMap<u32, HitboxCollisionConfig>>,
    }
}

impl Packet for UpdateHitboxCollisionConfig {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "UpdateHitboxCollisionConfig",
        id: 74,
        is_compressed: true,
        max_size: 36864011,
        category: PacketCategory::ASSETS,
    };
}

codec! {
    pub struct UpdateInteractions {
        pub type_: UpdateType,
        pub max_id: u32,
        pub interactions: Option<HashMap<u32, Interaction>>,
    }
}

impl Packet for UpdateInteractions {
    const DESCRIPTOR: &'static PacketDescriptor = &PacketDescriptor {
        name: "UpdateInteractions",
        id: 66,
        is_compressed: true,
        max_size: 1677721600,
        category: PacketCategory::ASSETS,
    };
}

// === Data types === //

#[derive(Debug, Clone, Default)]
pub struct Objective {
    pub objective_uuid: Uuid,
    pub objective_title_key: Option<String>,
    pub objective_description_key: Option<String>,
    pub objective_line_id: Option<String>,
    pub tasks: Option<Vec<ObjectiveTask>>,
}

impl Serde for Objective {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            Uuid::codec()
                .field(field![Objective, objective_uuid])
                .named("objective_uuid"),
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![Objective, objective_title_key])
                .named("objective_title_key"),
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![Objective, objective_description_key])
                .named("objective_description_key"),
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![Objective, objective_line_id])
                .named("objective_line_id"),
            VarArrayCodec::new(ObjectiveTask::codec(), 4096000)
                .nullable_variable()
                .field(field![Objective, tasks])
                .named("tasks"),
        ])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct ObjectiveTask {
    pub task_description_key: Option<String>,
    pub current_completion: u32,
    pub completion_needed: u32,
}

impl Serde for ObjectiveTask {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![ObjectiveTask, task_description_key])
                .named("task_description_key"),
            LeU32Codec
                .field(field![ObjectiveTask, current_completion])
                .named("current_completion"),
            LeU32Codec
                .field(field![ObjectiveTask, completion_needed])
                .named("completion_needed"),
        ])
        .erase()
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ordinalize, Default)]
#[repr(u8)]
pub enum UpdateType {
    #[default]
    Init,
    AddOrUpdate,
    Remove,
}

impl Serde for UpdateType {
    fn build_codec() -> ErasedCodec<Self> {
        EnumCodec::new().erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct AmbienceFx {
    pub id: Option<String>,
    pub conditions: Option<AmbienceFxConditions>,
    pub sounds: Option<Vec<AmbienceFxSound>>,
    pub music: Option<AmbienceFxMusic>,
    pub ambient_bed: Option<AmbienceFxAmbientBed>,
    pub sound_effect: Option<AmbienceFxSoundEffect>,
    pub priority: u32,
    pub blocked_ambience_fx_indices: Option<Vec<u32>>,
    pub audio_category_index: u32,
}

impl Serde for AmbienceFx {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![AmbienceFx, id])
                .named("id"),
            AmbienceFxConditions::codec()
                .nullable_variable()
                .field(field![AmbienceFx, conditions])
                .named("conditions"),
            VarArrayCodec::new(AmbienceFxSound::codec(), 4096000)
                .nullable_variable()
                .field(field![AmbienceFx, sounds])
                .named("sounds"),
            AmbienceFxMusic::codec()
                .nullable_variable()
                .field(field![AmbienceFx, music])
                .named("music"),
            AmbienceFxAmbientBed::codec()
                .nullable_variable()
                .field(field![AmbienceFx, ambient_bed])
                .named("ambient_bed"),
            AmbienceFxAmbientBed::codec()
                .nullable_variable()
                .field(field![AmbienceFx, ambient_bed])
                .named("ambient_bed"),
            AmbienceFxSoundEffect::codec()
                .nullable_fixed()
                .field(field![AmbienceFx, sound_effect])
                .named("sound_effect"),
            LeU32Codec
                .field(field![AmbienceFx, priority])
                .named("priority"),
            VarArrayCodec::new(LeU32Codec.erase(), 4096000)
                .nullable_variable()
                .field(field![AmbienceFx, blocked_ambience_fx_indices])
                .named("blocked_ambience_fx_indices"),
            LeU32Codec
                .field(field![AmbienceFx, audio_category_index])
                .named("audio_category_index"),
        ])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct AmbienceFxConditions {
    pub never: bool,
    pub environment_indices: Option<Vec<u32>>,
    pub weather_indices: Option<Vec<u32>>,
    pub fluid_fx_indices: Option<Vec<u32>>,
    pub environment_tag_pattern_index: u32,
    pub weather_tag_pattern_index: u32,
    pub surrounding_block_sound_sets: Option<Vec<AmbienceFxBlockSoundSet>>,
    pub altitude: Option<Range>,
    pub walls: Option<Rangeb>,
    pub roof: bool,
    pub roof_material_tag_pattern_index: u32,
    pub floor: bool,
    pub sun_light_level: Option<Rangeb>,
    pub torch_light_level: Option<Rangeb>,
    pub global_light_level: Option<Rangeb>,
    pub day_time: Option<Rangef>,
}

impl Serde for AmbienceFxConditions {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            ByteBoolCodec
                .field(field![AmbienceFxConditions, never])
                .named("never"),
            VarArrayCodec::new(LeU32Codec.erase(), 4096000)
                .nullable_variable()
                .field(field![AmbienceFxConditions, environment_indices])
                .named("environment_indices"),
            VarArrayCodec::new(LeU32Codec.erase(), 4096000)
                .nullable_variable()
                .field(field![AmbienceFxConditions, weather_indices])
                .named("weather_indices"),
            VarArrayCodec::new(LeU32Codec.erase(), 4096000)
                .nullable_variable()
                .field(field![AmbienceFxConditions, fluid_fx_indices])
                .named("fluid_fx_indices"),
            VarArrayCodec::new(AmbienceFxBlockSoundSet::codec(), 4096000)
                .nullable_variable()
                .field(field![AmbienceFxConditions, surrounding_block_sound_sets])
                .named("surrounding_block_sound_sets"),
            Range::codec()
                .nullable_fixed()
                .field(field![AmbienceFxConditions, altitude])
                .named("altitude"),
            Rangeb::codec()
                .nullable_fixed()
                .field(field![AmbienceFxConditions, walls])
                .named("walls"),
            ByteBoolCodec
                .field(field![AmbienceFxConditions, roof])
                .named("roof"),
            LeU32Codec
                .field(field![
                    AmbienceFxConditions,
                    roof_material_tag_pattern_index
                ])
                .named("roof_material_tag_pattern_index"),
            ByteBoolCodec
                .field(field![AmbienceFxConditions, floor])
                .named("floor"),
            Rangeb::codec()
                .nullable_fixed()
                .field(field![AmbienceFxConditions, sun_light_level])
                .named("sun_light_level"),
            Rangeb::codec()
                .nullable_fixed()
                .field(field![AmbienceFxConditions, torch_light_level])
                .named("torch_light_level"),
            Rangeb::codec()
                .nullable_fixed()
                .field(field![AmbienceFxConditions, global_light_level])
                .named("global_light_level"),
            Rangef::codec()
                .nullable_fixed()
                .field(field![AmbienceFxConditions, day_time])
                .named("day_time"),
        ])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct AmbienceFxSound {
    pub sound_event_index: u32,
    pub play_3d: AmbienceFxSoundPlay3d,
    pub block_sound_set_index: u32,
    pub altitude: AmbienceFxAltitude,
    pub frequency: Option<Rangef>,
    pub radius: Option<Range>,
}

impl Serde for AmbienceFxSound {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            LeU32Codec
                .field(field![AmbienceFxSound, sound_event_index])
                .named("sound_event_index"),
            AmbienceFxSoundPlay3d::codec()
                .field(field![AmbienceFxSound, play_3d])
                .named("play_3d"),
            LeU32Codec
                .field(field![AmbienceFxSound, block_sound_set_index])
                .named("block_sound_set_index"),
            AmbienceFxAltitude::codec()
                .field(field![AmbienceFxSound, altitude])
                .named("altitude"),
            Rangef::codec()
                .nullable_fixed()
                .field(field![AmbienceFxSound, frequency])
                .named("frequency"),
            Range::codec()
                .nullable_fixed()
                .field(field![AmbienceFxSound, radius])
                .named("radius"),
        ])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct AmbienceFxMusic {
    pub tracks: Option<Vec<String>>,
    pub volume: f32,
}

impl Serde for AmbienceFxMusic {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            VarArrayCodec::new(VarStringCodec::new(4096000).erase(), 4096000)
                .nullable_variable()
                .field(field![AmbienceFxMusic, tracks])
                .named("tracks"),
            LeF32Codec
                .field(field![AmbienceFxMusic, volume])
                .named("volume"),
        ])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct AmbienceFxAmbientBed {
    pub track: Option<String>,
    pub volume: f32,
}

impl Serde for AmbienceFxAmbientBed {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![AmbienceFxAmbientBed, track])
                .named("track"),
            LeF32Codec
                .field(field![AmbienceFxAmbientBed, volume])
                .named("volume"),
        ])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct AmbienceFxSoundEffect {
    pub reverb_effect_index: u32,
    pub equalizer_effect_index: u32,
    pub is_instant: bool,
}

impl Serde for AmbienceFxSoundEffect {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            LeU32Codec
                .field(field![AmbienceFxSoundEffect, reverb_effect_index])
                .named("reverb_effect_index"),
            LeU32Codec
                .field(field![AmbienceFxSoundEffect, equalizer_effect_index])
                .named("equalizer_effect_index"),
            ByteBoolCodec
                .field(field![AmbienceFxSoundEffect, is_instant])
                .named("is_instant"),
        ])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct AmbienceFxBlockSoundSet {
    pub block_sound_set_index: u32,
    pub percent: Option<Rangef>,
}

impl Serde for AmbienceFxBlockSoundSet {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            LeU32Codec
                .field(field![AmbienceFxBlockSoundSet, block_sound_set_index])
                .named("block_sound_set_index"),
            Rangef::codec()
                .nullable_variable()
                .field(field![AmbienceFxBlockSoundSet, percent])
                .named("percent"),
        ])
        .erase()
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ordinalize, Default)]
#[repr(u8)]
pub enum AmbienceFxSoundPlay3d {
    #[default]
    Random,
    LocationName,
    No,
}

impl Serde for AmbienceFxSoundPlay3d {
    fn build_codec() -> ErasedCodec<Self> {
        EnumCodec::new().erase()
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ordinalize, Default)]
#[repr(u8)]
pub enum AmbienceFxAltitude {
    #[default]
    Normal,
    Lowest,
    Highest,
    Random,
}

impl Serde for AmbienceFxAltitude {
    fn build_codec() -> ErasedCodec<Self> {
        EnumCodec::new().erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct AudioCategory {
    pub id: Option<String>,
    pub volume: f32,
}

impl Serde for AudioCategory {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![AudioCategory, id])
                .named("id"),
            LeF32Codec
                .field(field![AudioCategory, volume])
                .named("volume"),
        ])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct BlockBreakingDecal {
    pub stage_textures: Option<Vec<String>>,
}

impl Serde for BlockBreakingDecal {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            VarArrayCodec::new(VarStringCodec::new(4096000).erase(), 4096000)
                .nullable_variable()
                .field(field![BlockBreakingDecal, stage_textures])
                .named("stage_textures"),
        ])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct BlockGroup {
    pub names: Option<Vec<String>>,
}

impl Serde for BlockGroup {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            VarArrayCodec::new(VarStringCodec::new(4096000).erase(), 4096000)
                .nullable_variable()
                .field(field![BlockGroup, names])
                .named("names"),
        ])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Hitbox {
    pub min_x: f32,
    pub min_y: f32,
    pub min_z: f32,
    pub max_x: f32,
    pub max_y: f32,
    pub max_z: f32,
}

impl Serde for Hitbox {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            LeF32Codec.field(field![Hitbox, min_x]).named("min_x"),
            LeF32Codec.field(field![Hitbox, min_y]).named("min_y"),
            LeF32Codec.field(field![Hitbox, min_z]).named("min_z"),
            LeF32Codec.field(field![Hitbox, max_x]).named("max_x"),
            LeF32Codec.field(field![Hitbox, max_y]).named("max_y"),
            LeF32Codec.field(field![Hitbox, max_z]).named("max_z"),
        ])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct BlockParticleSet {
    pub id: Option<String>,
    pub color: Option<Color>,
    pub scale: f32,
    pub position_offset: Option<Vector3f>,
    pub rotation_offset: Option<Direction>,
    pub particle_system_ids: Option<HashMap<BlockParticleEvent, String>>,
}

impl Serde for BlockParticleSet {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![BlockParticleSet, id])
                .named("id"),
            Color::codec()
                .nullable_fixed()
                .field(field![BlockParticleSet, color])
                .named("color"),
            LeF32Codec
                .field(field![BlockParticleSet, scale])
                .named("scale"),
            Vector3f::codec()
                .nullable_fixed()
                .field(field![BlockParticleSet, position_offset])
                .named("position_offset"),
            Direction::codec()
                .nullable_fixed()
                .field(field![BlockParticleSet, rotation_offset])
                .named("rotation_offset"),
            VarDictionaryCodec::new(
                BlockParticleEvent::codec(),
                VarStringCodec::new(4096000).erase(),
                4096000,
            )
            .nullable_variable()
            .field(field![BlockParticleSet, particle_system_ids])
            .named("particle_system_ids"),
        ])
        .erase()
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ordinalize, Default)]
#[repr(u8)]
pub enum BlockParticleEvent {
    #[default]
    Walk,
    Run,
    Sprint,
    SoftLand,
    HardLand,
    MoveOut,
    Hit,
    Break,
    Build,
    Physics,
}

impl Serde for BlockParticleEvent {
    fn build_codec() -> ErasedCodec<Self> {
        EnumCodec::new().erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct BlockSet {
    pub name: Option<String>,
    pub blocks: Option<Vec<u32>>,
}

impl Serde for BlockSet {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![BlockSet, name])
                .named("name"),
            VarArrayCodec::new(LeU32Codec.erase(), 4096000)
                .nullable_variable()
                .field(field![BlockSet, blocks])
                .named("blocks"),
        ])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct BlockSoundSet {
    pub id: Option<String>,
    pub sound_event_indices: Option<HashMap<BlockSoundEvent, u32>>,
    pub move_in_repeat_range: Option<FloatRange>,
}

impl Serde for BlockSoundSet {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![BlockSoundSet, id])
                .named("id"),
            VarDictionaryCodec::new(BlockSoundEvent::codec(), LeU32Codec.erase(), 4096000)
                .nullable_variable()
                .field(field![BlockSoundSet, sound_event_indices])
                .named("sound_event_indices"),
            FloatRange::codec()
                .nullable_fixed()
                .field(field![BlockSoundSet, move_in_repeat_range])
                .named("move_in_repeat_range"),
        ])
        .erase()
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ordinalize, Default)]
#[repr(u8)]
pub enum BlockSoundEvent {
    #[default]
    Walk,
    Land,
    MoveIn,
    MoveOut,
    Hit,
    Break,
    Build,
    Clone,
    Harvest,
}

impl Serde for BlockSoundEvent {
    fn build_codec() -> ErasedCodec<Self> {
        EnumCodec::new().erase()
    }
}

codec! {
    pub struct BlockType {
        pub item: Option<String>,
        pub name: Option<String>,
        pub unknown: bool,
        pub draw_type: DrawType,
        pub material: BlockMaterial,
        pub opacity: Opacity,
        pub shader_effect: Option<Vec<ShaderType>>,
        pub hitbox: u32,
        pub interaction_hitbox: u32,
        pub model: Option<String>,
        pub model_texture: Option<Vec<ModelTexture>>,
        pub model_scale: f32,
        pub model_animation: Option<String>,
        pub looping: bool,
        pub max_support_distance: u32,
        pub block_supports_required_for: BlockSupportsRequiredForType,
        pub support: Option<HashMap<BlockNeighbor, Vec<RequiredBlockFaceSupport>>>,
        pub supporting: Option<HashMap<BlockNeighbor, Vec<BlockFaceSupport>>>,
        pub requires_alpha_blending: bool,
        pub cube_textures: Option<Vec<BlockTextures>>,
        pub cube_side_mesh_texture: Option<String>,
        pub cube_shading_mode: ShadingMode,
        pub random_rotation: RandomRotation,
        pub variant_rotation: VariantRotation,
        pub rotation_yaw_placement_offset: Rotation,
        pub block_sound_set_index: u32,
        pub ambient_sound_event_index: u32,
        pub particles: Option<Vec<String>>,
        pub block_particle_set_id: Option<String>,
        pub block_breaking_decal_id: Option<String>,
        pub particle_color: Option<Color>,
        pub light: Option<ColorLight>,
        pub tint: Option<Tint>,
        pub biome_tint: Option<Tint>,
        pub group: u32,
        pub transition_texture: Option<String>,
        pub transition_to_groups: Option<Vec<u32>>,
        pub movement_settings: Option<BlockMovementSettings>,
        pub flags: Option<BlockFlags>,
        pub interaction_hint: Option<String>,
        pub gathering: Option<BlockGathering>,
        pub placement_settings: Option<BlockPlacementSettings>,
        pub display: Option<ModelDisplay>,
        pub rail: Option<RailConfig>,
        pub ignore_support_when_placed: bool,
        pub interactions: Option<HashMap<InteractionType, u32>>,
        pub states: Option<HashMap<String, u32>>,
        pub transition_to_tag: u32,
        pub tag_indexes: Option<u32>,
        pub bench: Option<Bench>,
        pub connected_block_rule_set: Option<ConnectedBlockRuleSet>,
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ordinalize, Default)]
#[repr(u8)]
pub enum DrawType {
    #[default]
    Empty,
    GizmoCube,
    Cube,
    Model,
    CubeWithModel,
}

impl Serde for DrawType {
    fn build_codec() -> ErasedCodec<Self> {
        EnumCodec::new().erase()
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ordinalize, Default)]
#[repr(u8)]
pub enum BlockMaterial {
    #[default]
    Empty,
    Solid,
}

impl Serde for BlockMaterial {
    fn build_codec() -> ErasedCodec<Self> {
        EnumCodec::new().erase()
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ordinalize, Default)]
#[repr(u8)]
pub enum Opacity {
    #[default]
    Solid,
    Semitransparent,
    Cutout,
    Transparent,
}

impl Serde for Opacity {
    fn build_codec() -> ErasedCodec<Self> {
        EnumCodec::new().erase()
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ordinalize, Default)]
#[repr(u8)]
pub enum ShaderType {
    #[default]
    None,
    Wind,
    WindAttached,
    WindRandom,
    WindFractal,
    Ice,
    Water,
    Lava,
    Slime,
    Ripple,
}

impl Serde for ShaderType {
    fn build_codec() -> ErasedCodec<Self> {
        EnumCodec::new().erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct ModelTexture {
    pub texture: Option<String>,
    pub weight: f32,
}

impl Serde for ModelTexture {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![ModelTexture, texture])
                .named("texture"),
            LeF32Codec
                .field(field![ModelTexture, weight])
                .named("weight"),
        ])
        .erase()
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ordinalize, Default)]
#[repr(u8)]
pub enum BlockSupportsRequiredForType {
    #[default]
    Any,
    All,
}

impl Serde for BlockSupportsRequiredForType {
    fn build_codec() -> ErasedCodec<Self> {
        EnumCodec::new().erase()
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ordinalize, Default)]
#[repr(u8)]
pub enum BlockNeighbor {
    #[default]
    Up,
    Down,
    North,
    East,
    South,
    West,
    UpNorth,
    UpSouth,
    UpEast,
    UpWest,
    DownNorth,
    DownSouth,
    DownEast,
    DownWest,
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest,
    UpNorthEast,
    UpSouthEast,
    UpSouthWest,
    UpNorthWest,
    DownNorthEast,
    DownSouthEast,
    DownSouthWest,
    DownNorthWest,
}

impl Serde for BlockNeighbor {
    fn build_codec() -> ErasedCodec<Self> {
        EnumCodec::new().erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct RequiredBlockFaceSupport {
    pub face_type: Option<String>,
    pub self_face_type: Option<String>,
    pub block_set_id: Option<String>,
    pub block_type_id: u32,
    pub tag_index: u32,
    pub fluid_id: u32,
    pub support: SupportMatch,
    pub match_self: SupportMatch,
    pub allow_support_propagation: bool,
    pub rotate: bool,
    pub filler: Option<Vec<Vector3i>>,
}

impl Serde for RequiredBlockFaceSupport {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![RequiredBlockFaceSupport, face_type])
                .named("face_type"),
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![RequiredBlockFaceSupport, self_face_type])
                .named("self_face_type"),
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![RequiredBlockFaceSupport, block_set_id])
                .named("block_set_id"),
            LeU32Codec
                .field(field![RequiredBlockFaceSupport, block_type_id])
                .named("block_type_id"),
            LeU32Codec
                .field(field![RequiredBlockFaceSupport, tag_index])
                .named("tag_index"),
            LeU32Codec
                .field(field![RequiredBlockFaceSupport, fluid_id])
                .named("fluid_id"),
            SupportMatch::codec()
                .field(field![RequiredBlockFaceSupport, support])
                .named("support"),
            SupportMatch::codec()
                .field(field![RequiredBlockFaceSupport, match_self])
                .named("match_self"),
            ByteBoolCodec
                .field(field![RequiredBlockFaceSupport, allow_support_propagation])
                .named("allow_support_propagation"),
            ByteBoolCodec
                .field(field![RequiredBlockFaceSupport, rotate])
                .named("rotate"),
            VarArrayCodec::new(Vector3i::codec(), 4096000)
                .nullable_variable()
                .field(field![RequiredBlockFaceSupport, filler])
                .named("filler"),
        ])
        .erase()
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ordinalize, Default)]
#[repr(u8)]
pub enum SupportMatch {
    #[default]
    Ignored,
    Required,
    Disallowed,
}

impl Serde for SupportMatch {
    fn build_codec() -> ErasedCodec<Self> {
        EnumCodec::new().erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct BlockFaceSupport {
    pub face_type: Option<String>,
    pub filler: Option<Vec<Vector3i>>,
}

impl Serde for BlockFaceSupport {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![BlockFaceSupport, face_type])
                .named("face_type"),
            VarArrayCodec::new(Vector3i::codec(), 4096000)
                .nullable_variable()
                .field(field![BlockFaceSupport, filler])
                .named("filler"),
        ])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct BlockTextures {
    pub top: Option<String>,
    pub bottom: Option<String>,
    pub front: Option<String>,
    pub back: Option<String>,
    pub left: Option<String>,
    pub right: Option<String>,
    pub weight: f32,
}

impl Serde for BlockTextures {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![BlockTextures, top])
                .named("top"),
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![BlockTextures, bottom])
                .named("bottom"),
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![BlockTextures, front])
                .named("front"),
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![BlockTextures, back])
                .named("back"),
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![BlockTextures, left])
                .named("left"),
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![BlockTextures, right])
                .named("right"),
            LeF32Codec
                .field(field![BlockTextures, weight])
                .named("weight"),
        ])
        .erase()
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ordinalize, Default)]
#[repr(u8)]
pub enum ShadingMode {
    #[default]
    Standard,
    Flat,
    Fullbright,
    Reflective,
}

impl Serde for ShadingMode {
    fn build_codec() -> ErasedCodec<Self> {
        EnumCodec::new().erase()
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ordinalize, Default)]
#[repr(u8)]
pub enum RandomRotation {
    #[default]
    None,
    YawPitchRollStep1,
    YawStep1,
    YawStep1XZ,
    YawStep90,
}

impl Serde for RandomRotation {
    fn build_codec() -> ErasedCodec<Self> {
        EnumCodec::new().erase()
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ordinalize, Default)]
#[repr(u8)]
pub enum VariantRotation {
    #[default]
    None,
    Wall,
    UpDown,
    Pipe,
    DoublePipe,
    NESW,
    UpDownNESW,
    All,
}

impl Serde for VariantRotation {
    fn build_codec() -> ErasedCodec<Self> {
        EnumCodec::new().erase()
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ordinalize, Default)]
#[repr(u8)]
pub enum Rotation {
    #[default]
    None,
    Ninety,
    OneEighty,
    TwoSeventy,
}

impl Serde for Rotation {
    fn build_codec() -> ErasedCodec<Self> {
        EnumCodec::new().erase()
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Tint {
    pub top: u32,
    pub bottom: u32,
    pub front: u32,
    pub back: u32,
    pub left: u32,
    pub right: u32,
}

impl Serde for Tint {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            LeU32Codec.field(field![Tint, top]).named("top"),
            LeU32Codec.field(field![Tint, bottom]).named("bottom"),
            LeU32Codec.field(field![Tint, front]).named("front"),
            LeU32Codec.field(field![Tint, back]).named("back"),
            LeU32Codec.field(field![Tint, left]).named("left"),
            LeU32Codec.field(field![Tint, right]).named("right"),
        ])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct BlockMovementSettings {
    pub is_climbable: bool,
    pub climb_up_speed_multiplier: f32,
    pub climb_down_speed_multiplier: f32,
    pub climb_lateral_speed_multiplier: f32,
    pub is_bouncy: bool,
    pub bounce_velocity: f32,
    pub drag: f32,
    pub friction: f32,
    pub terminal_velocity_modifier: f32,
    pub horizontal_speed_multiplier: f32,
    pub acceleration: f32,
    pub force_jump_multiplier: f32,
}

impl Serde for BlockMovementSettings {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            ByteBoolCodec
                .field(field![BlockMovementSettings, is_climbable])
                .named("is_climbable"),
            LeF32Codec
                .field(field![BlockMovementSettings, climb_up_speed_multiplier])
                .named("climb_up_speed_multiplier"),
            LeF32Codec
                .field(field![BlockMovementSettings, climb_down_speed_multiplier])
                .named("climb_down_speed_multiplier"),
            LeF32Codec
                .field(field![
                    BlockMovementSettings,
                    climb_lateral_speed_multiplier
                ])
                .named("climb_lateral_speed_multiplier"),
            ByteBoolCodec
                .field(field![BlockMovementSettings, is_bouncy])
                .named("is_bouncy"),
            LeF32Codec
                .field(field![BlockMovementSettings, bounce_velocity])
                .named("bounce_velocity"),
            LeF32Codec
                .field(field![BlockMovementSettings, drag])
                .named("drag"),
            LeF32Codec
                .field(field![BlockMovementSettings, friction])
                .named("friction"),
            LeF32Codec
                .field(field![BlockMovementSettings, terminal_velocity_modifier])
                .named("terminal_velocity_modifier"),
            LeF32Codec
                .field(field![BlockMovementSettings, horizontal_speed_multiplier])
                .named("horizontal_speed_multiplier"),
            LeF32Codec
                .field(field![BlockMovementSettings, acceleration])
                .named("acceleration"),
            LeF32Codec
                .field(field![BlockMovementSettings, force_jump_multiplier])
                .named("force_jump_multiplier"),
        ])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct BlockFlags {
    pub is_usable: bool,
    pub is_stackable: bool,
}

impl Serde for BlockFlags {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            ByteBoolCodec
                .field(field![BlockFlags, is_usable])
                .named("is_usable"),
            ByteBoolCodec
                .field(field![BlockFlags, is_stackable])
                .named("is_stackable"),
        ])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct BlockGathering {
    pub breaking: Option<BlockBreaking>,
    pub harvest: Option<Harvesting>,
    pub soft: Option<SoftBlock>,
}

impl Serde for BlockGathering {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            BlockBreaking::codec()
                .nullable_variable()
                .field(field![BlockGathering, breaking])
                .named("breaking"),
            Harvesting::codec()
                .nullable_variable()
                .field(field![BlockGathering, harvest])
                .named("harvest"),
            SoftBlock::codec()
                .nullable_variable()
                .field(field![BlockGathering, soft])
                .named("soft"),
        ])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct BlockBreaking {
    pub gather_type: Option<String>,
    pub health: f32,
    pub quantity: u32,
    pub quality: u32,
    pub item_id: Option<String>,
    pub drop_list_id: Option<String>,
}

impl Serde for BlockBreaking {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![BlockBreaking, gather_type])
                .named("gather_type"),
            LeF32Codec
                .field(field![BlockBreaking, health])
                .named("health"),
            LeU32Codec
                .field(field![BlockBreaking, quantity])
                .named("quantity"),
            LeU32Codec
                .field(field![BlockBreaking, quality])
                .named("quality"),
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![BlockBreaking, item_id])
                .named("item_id"),
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![BlockBreaking, drop_list_id])
                .named("drop_list_id"),
        ])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Harvesting {
    pub item_id: Option<String>,
    pub drop_list_id: Option<String>,
}

impl Serde for Harvesting {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![Harvesting, item_id])
                .named("item_id"),
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![Harvesting, drop_list_id])
                .named("drop_list_id"),
        ])
        .erase()
    }
}

#[derive(Debug, Clone, Default)]
pub struct SoftBlock {
    pub item_id: Option<String>,
    pub drop_list_id: Option<String>,
    pub is_weapon_breakable: bool,
}

impl Serde for SoftBlock {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![SoftBlock, item_id])
                .named("item_id"),
            VarStringCodec::new(4096000)
                .nullable_variable()
                .field(field![SoftBlock, drop_list_id])
                .named("drop_list_id"),
            ByteBoolCodec
                .field(field![SoftBlock, is_weapon_breakable])
                .named("is_weapon_breakable"),
        ])
        .erase()
    }
}

codec! {
    pub struct BlockPlacementSettings {
        pub allow_rotation_key: bool,
        pub place_in_empty_blocks: bool,
        pub preview_visibility: BlockPreviewVisibility,
        pub rotation_mode: BlockPlacementRotationMode,
        pub wall_placement_override_block_id: u32,
        pub floor_placement_override_block_id: u32,
        pub ceiling_placement_override_block_id: u32,
    }

    pub enum BlockPreviewVisibility {
        AlwaysVisible,
        AlwaysHidden,
        Default,
    }

    pub enum BlockPlacementRotationMode {
        FacingPlayer,
        StairFacingPlayer,
        BlockNormal,
        Default,
    }

    pub struct ModelDisplay {
        pub node: Option<String>,
        pub attach_to: Option<String>,
        pub translation: Option<Vector3f>,
        pub rotation: Option<Vector3f>,
        pub scale: Option<Vector3f>,
    }

    pub struct RailConfig {
        pub points: Option<Vec<RailPoint>>,
    }

    pub struct RailPoint {
        pub point: Option<Vector3f>,
        pub normal: Option<Vector3f>,
    }

    pub enum InteractionType {
        Primary,
        Secondary,
        Ability1,
        Ability2,
        Ability3,
        Use,
        Pick,
        Pickup,
        CollisionEnter,
        CollisionLeave,
        Collision,
        EntityStatEffect,
        SwapTo,
        SwapFrom,
        Death,
        Wielding,
        ProjectileSpawn,
        ProjectileHit,
        ProjectileMiss,
        ProjectileBounce,
        Held,
        HeldOffhand,
        Equipped,
        Dodge,
        GameModeSwap,
    }

    pub struct Bench {
        pub bench_tier_levels: Option<Vec<BenchTierLevel>>,
    }

    pub struct BenchTierLevel {
        pub bench_upgrade_requirement: Option<BenchUpgradeRequirement>,
        pub crafting_time_reduction_modifier: f64,
        pub extra_input_slot: u32,
        pub extra_output_slot: u32,
    }

    pub struct BenchUpgradeRequirement {
        pub material: Option<Vec<MaterialQuantity>>,
        pub time_seconds: f64,
    }

    pub struct MaterialQuantity {
        pub item_id: Option<String>,
        pub item_tag: u32,
        pub resource_type_id: Option<String>,
        pub quantity: u32,
    }

    pub struct ConnectedBlockRuleSet {
        pub type_: ConnectedBlockRuleSetType,
        pub stair: Option<StairConnectedBlockRuleSet>,
        pub roof: Option<RoofConnectedBlockRuleSet>,
    }

    pub enum ConnectedBlockRuleSetType {
        Stair,
        Roof,
    }

    pub struct StairConnectedBlockRuleSet {
        pub straight_block_id: u32,
        pub corner_left_block_id: u32,
        pub corner_right_block_id: u32,
        pub inverted_corner_left_block_id: u32,
        pub inverted_corner_right_block_id: u32,
    }

    pub struct RoofConnectedBlockRuleSet {
        pub regular: Option<StairConnectedBlockRuleSet>,
        pub hollow: Option<StairConnectedBlockRuleSet>,
        pub topper_block_id: u32,
        pub width: u32,
        pub material_name: Option<String>,
    }
}

codec! {
    pub struct CameraShake {
        pub first_person: Option<CameraShakeConfig>,
        pub third_person: Option<CameraShakeConfig>,
    }

    pub struct CameraShakeConfig {
        pub duration: f32,
        pub start_time: f32,
        pub continuous: bool,
        pub ease_in: Option<EasingConfig>,
        pub ease_out: Option<EasingConfig>,
        pub offset: Option<OffsetNoise>,
        pub rotation: Option<RotationNoise>,
    }

    pub struct EasingConfig {
        @small = true;

        pub time: f32,
        pub type_: EasingType,
    }

    pub enum EasingType {
        Linear,
        QuadIn,
        QuadOut,
        QuadInOut,
        CubicIn,
        CubicOut,
        CubicInOut,
        QuartIn,
        QuartOut,
        QuartInOut,
        QuintIn,
        QuintOut,
        QuintInOut,
        SineIn,
        SineOut,
        SineInOut,
        ExpoIn,
        ExpoOut,
        ExpoInOut,
        CircIn,
        CircOut,
        CircInOut,
        ElasticIn,
        ElasticOut,
        ElasticInOut,
        BackIn,
        BackOut,
        BackInOut,
        BounceIn,
        BounceOut,
        BounceInOut,
    }

    pub struct OffsetNoise {
        pub x: Option<Vec<NoiseConfig>>,
        pub y: Option<Vec<NoiseConfig>>,
        pub z: Option<Vec<NoiseConfig>>,
    }

    pub struct NoiseConfig {
        pub seed: u32,
        pub type_: NoiseType,
        pub frequency: f32,
        pub amplitude: f32,
        pub clamp: Option<ClampConfig>,
    }

    pub enum NoiseType {
        Sin,
        Cos,
        PerlinLinear,
        PerlinHermite,
        PerlinQuintic,
        Random,
    }

    pub struct ClampConfig {
        pub min: f32,
        pub max: f32,
        pub normalize: bool,
    }

    pub struct RotationNoise {
        pub pitch: Option<Vec<NoiseConfig>>,
        pub yaw: Option<Vec<NoiseConfig>>,
        pub roll: Option<Vec<NoiseConfig>>,
    }
}

codec! {
    pub struct EntityEffect {
        pub id: Option<String>,
        pub name: Option<String>,
        pub application_effects: Option<ApplicationEffects>,
        pub world_removal_sound_event_index: u32,
        pub local_removal_sound_event_index: u32,
        pub model_override: Option<ModelOverride>,
        pub duration: f32,
        pub infinite: bool,
        pub debuff: bool,
        pub status_effect_icon: Option<String>,
        pub overlap_behavior: OverlapBehavior,
        pub damage_calculator_cooldown: f32,
        pub stat_modifiers: Option<HashMap<u32, f32>>,
        pub value_type: ValueType,
    }

    pub struct ApplicationEffects {
        pub entity_bottom_tint: Option<Color>,
        pub entity_top_tint: Option<Color>,
        pub entity_animation_id: Option<String>,
        pub particles: Option<Vec<ModelParticle>>,
        pub first_person_particles: Option<Vec<ModelParticle>>,
        pub screen_effect: Option<String>,
        pub horizontal_speed_multiplier: f32,
        pub sound_event_index_local: u32,
        pub sound_event_index_world: u32,
        pub world_vfx_id: Option<String>,
        pub movement_effects: Option<MovementEffects>,
        pub mouse_sensitivity_adjustment_target: f32,
        pub mouse_sensitivity_adjustment_duration: f32,
        pub ability_effects: AbilityEffects,
    }

    pub struct ModelParticle {
        pub system_id: Option<String>,
        pub scale: f32,
        pub color: Option<Color>,
        pub target_entity_part: EntityPart,
        pub target_node_name: Option<String>,
        pub position_offset: Option<Vector3f>,
        pub rotation_offset: Option<Direction>,
        pub detached_from_model: bool,
    }

    pub enum EntityPart {
        Self_,
        Entity,
        PrimaryItem,
        SecondaryItem,
    }

    pub struct MovementEffects {
        pub disable_forward: bool,
        pub disable_backward: bool,
        pub disable_left: bool,
        pub disable_right: bool,
        pub disable_sprint: bool,
        pub disable_jump: bool,
        pub disable_crouch: bool,
    }

    pub struct AbilityEffects {
        pub disabled: Option<Vec<InteractionType>>,
    }

    pub struct ModelOverride {
        pub model: Option<String>,
        pub texture: Option<String>,
        pub animation_sets: Option<HashMap<String, AnimationSet>>,
    }

    pub struct AnimationSet {
        pub id:Option<String>,
        pub animations: Option<Vec<Animation>>,
        pub next_animation_delay: Option<Rangef>,
    }

    pub struct Animation {
        pub name: Option<String>,
        pub speed: f32,
        pub blending_duration: f32,
        pub looping: bool,
        pub weight: f32,
        pub footstep_intervals: Option<Vec<u32>>,
        pub sound_event_index: u32,
        pub passive_loop_count: u32,
    }

    pub enum OverlapBehavior {
        Extend,
        Overwrite,
        Ignore,
    }

    pub enum ValueType {
        Percent,
        Absolute,
    }
}

codec! {
    pub struct EntityStateType {
        pub id: Option<String>,
        pub value: f32,
        pub min: f32,
        pub max: f32,
        pub min_value_effects: Option<EntityStatEffects>,
        pub max_value_effects: Option<EntityStatEffects>,
        pub reset_behavior: EntityStatResetBehavior,
    }

    pub struct EntityStatEffects {
        pub trigger_at_zero: bool,
        pub sound_event_index: u32,
        pub particles: Option<Vec<ModelParticle>>,
    }

    pub enum EntityStatResetBehavior {
        InitialValue,
        MaxValue,
    }
}

codec! {
    pub struct EntityUiComponent {
        pub type_: EntityUiType,
        pub hitbox_offset: Option<Vector2f>,
        pub unknown: bool,
        pub entity_stat_index: u32,
        pub combat_text_random_position_offset_range: Option<RangeVector2f>,
        pub combat_text_viewport_margin: f32,
        pub combat_text_duration: f32,
        pub combat_text_hit_angle_modifier_strength: f32,
        pub combat_text_font_size: f32,
        pub combat_text_color: Option<Color>,
        pub combat_text_animation_events: Option<Vec<CombatTextEntityUiComponentAnimationEvent>>,
    }

    pub enum EntityUiType {
        EntityStat,
        CombatText,
    }

    pub struct CombatTextEntityUiComponentAnimationEvent {
        pub type_: CombatTextEntityUiAnimationEventType,
        pub start_at: f32,
        pub end_at: f32,
        pub start_scale: f32,
        pub end_scale: f32,
        pub position_offset: Option<Vector2f>,
        pub start_opacity: f32,
        pub end_opacity: f32,
    }

    pub enum CombatTextEntityUiAnimationEventType {
        Scale,
        Position,
        Opacity,
    }
}

codec! {
    pub struct WorldEnvironment {
        pub id: Option<String>,
        pub water_tint: Option<Color>,
        pub fluid_particles: Option<HashMap<u32, FluidParticle>>,
        pub tag_indices: Option<Vec<u32>>,
    }

    pub struct FluidParticle {
        pub system_id: Option<String>,
        pub color: Option<u32>,
        pub scale: f32,
    }
}

codec! {
    pub struct EqualizerEffect {
        pub id: Option<String>,
        pub low_gain: f32,
        pub low_cut_off: f32,
        pub low_mid_gain: f32,
        pub low_mid_center: f32,
        pub low_mid_width: f32,
        pub high_mid_gain: f32,
        pub high_mid_center: f32,
        pub high_mid_width: f32,
        pub high_gain: f32,
        pub high_cut_off: f32,
    }
}

codec! {
    pub struct ItemCategory {
        pub id: Option<String>,
        pub name: Option<String>,
        pub icon: Option<String>,
        pub order: u32,
        pub info_display_mode: ItemGridInfoDisplayMode,
        pub children: Option<Vec<ItemCategory>>,
    }

    pub enum ItemGridInfoDisplayMode {
        Tooltip,
        Adjacent,
        None,
    }
}

codec! {
    pub struct FluidFx {
        pub id: Option<String>,
        pub shader: ShaderType,
        pub fog_mode: FluidFog,
        pub fog_color: Option<String>,
        pub fog_distance: Option<NearFar>,
        pub fog_depth_start: f32,
        pub fog_depth_falloff: f32,
        pub color_filter: Option<String>,
        pub color_saturation: f32,
        pub distortion_amplitude: f32,
        pub distortion_frequency: f32,
        pub particle: Option<FluidParticle>,
        pub movement_settings: Option<FluidFxMovementSettings>,
    }

    pub enum FluidFog {
        Color,
        ColorLight,
        EnvironmentTint,
    }

    pub struct FluidFxMovementSettings {
        @small = true;
        pub swim_up_speed: f32,
        pub swim_down_speed: f32,
        pub sink_speed: f32,
        pub horizontal_speed_multiplier: f32,
        pub field_of_view_multiplier: f32,
        pub entry_velocity_multiplier: f32,
    }
}

codec! {
    pub struct Fluid {
        pub id: Option<String>,
        pub max_fluid_level: u32,
        pub cube_textures: Option<Vec<BlockTextures>>,
        pub requires_alpha_blending: bool,
        pub opacity: Opacity,
        pub shader_effect: Option<Vec<ShaderType>>,
        pub light: Option<ColorLight>,
        pub fluid_fx_index: u32,
        pub block_sound_set_index: u32,
        pub block_particle_set_id: Option<String>,
        pub particle_color: Option<Color>,
        pub tag_indexes: Option<Vec<u32>>,
    }
}

codec! {
    pub struct HitboxCollisionConfig {
        pub collision_type: CollisionType,
        pub soft_collision_offset_ratio: f32,
    }

    pub enum CollisionType {
        Hard,
        Soft,
    }
}

codec! {
    pub struct Interaction {
        pub wait_for_data_from: WaitForDataFrom,
        pub effects: Option<InteractionEffects>,
        pub horizontal_speed_multiplier: f32,
        pub run_time: f32,
        pub cancel_on_item_change: bool,
        pub settings: Option<HashMap<GameMode, InteractionSettings>>,
        pub rules: Option<InteractionRules>,
        pub tags: Option<Vec<u32>>,
        pub camera: Option<InteractionCameraSettings>,
    }

    pub enum WaitForDataFrom {
        Client,
        Server,
        None,
    }

    pub struct InteractionEffects {
        pub particles: Option<Vec<ModelParticle>>,
        pub first_person_particles: Option<Vec<ModelParticle>>,
        pub world_sound_event_index: u32,
        pub local_sound_event_index: u32,
        pub trails: Option<Vec<ModelTrail>>,
        pub wait_for_animation_to_finish: bool,
        pub item_player_animations_id: Option<String>,
        pub item_animation_id: Option<String>,
        pub clear_animation_on_finish: bool,
        pub clear_sound_event_on_finish: bool,
        pub camera_shake: Option<CameraShakeEffect>,
        pub movement_effects: Option<MovementEffects>,
        pub start_delay: f32,
    }

    pub struct ModelTrail {
        pub trail_id: Option<String>,
        pub target_entity_part: EntityPart,
        pub target_node_name: Option<String>,
        pub position_offset: Option<Vector3f>,
        pub rotation_offset: Option<Direction>,
        pub fixed_rotation: bool,
    }

    pub struct CameraShakeEffect {
        @small = true;
        pub camera_shake_id:  u32,
        pub intensity: f32,
        pub mode: AccumulationMode,
    }

    pub enum AccumulationMode {
        Set,
        Sum,
        Average,
    }

    pub enum GameMode {
        Adventure,
        Creative,
    }

    pub struct InteractionSettings {
        pub allow_skip_on_click: bool
    }

    pub struct InteractionRules {
        pub blocked_by: Option<Vec<InteractionType>>,
        pub blocking: Option<Vec<InteractionType>>,
        pub interrupted_by: Option<Vec<InteractionType>>,
        pub interrupting: Option<Vec<InteractionType>>,
        pub blocked_by_bypass_index: u32,
        pub blocking_bypass_index: u32,
        pub interrupted_by_bypass_index: u32,
        pub interrupting_bypass_index: u32,
    }

    pub struct InteractionCameraSettings {
        pub first_person: Option<Vec<InteractionCamera>>,
        pub third_person: Option<Vec<InteractionCamera>>,
    }

    pub struct InteractionCamera {
        pub time: f32,
        pub position: Option<Vector3f>,
        pub rotation: Option<Direction>,
    }
}
