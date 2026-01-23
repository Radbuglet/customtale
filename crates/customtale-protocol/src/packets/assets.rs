use std::collections::HashMap;

use enum_ordinalize::Ordinalize;
use uuid::Uuid;

use crate::{
    data::{Range, Rangeb, Rangef},
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
            .named("categories"),
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
