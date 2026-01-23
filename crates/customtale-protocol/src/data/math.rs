use crate::{
    field,
    serde::{ByteCodec, Codec, LeF32Codec, LeI32Codec, Serde, StructCodec},
};

#[derive(Debug, Copy, Clone, Default)]
pub struct Range {
    pub min: i32,
    pub max: i32,
}

impl Serde for Range {
    fn build_codec() -> crate::serde::ErasedCodec<Self> {
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
    fn build_codec() -> crate::serde::ErasedCodec<Self> {
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
    fn build_codec() -> crate::serde::ErasedCodec<Self> {
        StructCodec::new([
            ByteCodec.field(field![Rangeb, min]).named("min"),
            ByteCodec.field(field![Rangeb, max]).named("max"),
        ])
        .erase()
    }
}
