use crate::{
    field,
    serde::{ByteCodec, Codec, ErasedCodec, LeF32Codec, LeI32Codec, Serde, StructCodec},
};

#[derive(Debug, Copy, Clone, Default)]
pub struct Range {
    pub min: i32,
    pub max: i32,
}

impl Serde for Range {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            LeI32Codec.field(field![Range, min]).named("min"),
            LeI32Codec.field(field![Range, max]).named("max"),
        ])
        .erase()
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Rangef {
    pub min: f32,
    pub max: f32,
}

impl Serde for Rangef {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            LeF32Codec.field(field![Rangef, min]).named("min"),
            LeF32Codec.field(field![Rangef, max]).named("max"),
        ])
        .erase()
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Rangeb {
    pub min: u8,
    pub max: u8,
}

impl Serde for Rangeb {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            ByteCodec.field(field![Rangeb, min]).named("min"),
            ByteCodec.field(field![Rangeb, max]).named("max"),
        ])
        .erase()
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct FloatRange {
    pub inclusive_min: f32,
    pub inclusive_max: f32,
}

impl Serde for FloatRange {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            LeF32Codec
                .field(field![FloatRange, inclusive_min])
                .named("inclusive_min"),
            LeF32Codec
                .field(field![FloatRange, inclusive_max])
                .named("inclusive_max"),
        ])
        .erase()
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Serde for Color {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            ByteCodec.field(field![Color, red]).named("red"),
            ByteCodec.field(field![Color, green]).named("green"),
            ByteCodec.field(field![Color, blue]).named("blue"),
        ])
        .erase()
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Vector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Serde for Vector3f {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            LeF32Codec.field(field![Vector3f, x]).named("x"),
            LeF32Codec.field(field![Vector3f, y]).named("y"),
            LeF32Codec.field(field![Vector3f, z]).named("z"),
        ])
        .erase()
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Vector3i {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Serde for Vector3i {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            LeI32Codec.field(field![Vector3i, x]).named("x"),
            LeI32Codec.field(field![Vector3i, y]).named("y"),
            LeI32Codec.field(field![Vector3i, z]).named("z"),
        ])
        .erase()
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Direction {
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}

impl Serde for Direction {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            LeF32Codec.field(field![Direction, yaw]).named("yaw"),
            LeF32Codec.field(field![Direction, pitch]).named("pitch"),
            LeF32Codec.field(field![Direction, roll]).named("roll"),
        ])
        .erase()
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct ColorLight {
    pub radius: u8,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Serde for ColorLight {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            ByteCodec.field(field![ColorLight, radius]).named("radius"),
            ByteCodec.field(field![ColorLight, red]).named("red"),
            ByteCodec.field(field![ColorLight, green]).named("green"),
            ByteCodec.field(field![ColorLight, blue]).named("blue"),
        ])
        .erase()
    }
}
