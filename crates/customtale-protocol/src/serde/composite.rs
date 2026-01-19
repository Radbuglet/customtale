use crate::serde::{Codec as _, ErasedCodec, LeU64, Serde, StructCodec, field};

#[derive(Debug, Copy, Clone, Default)]
pub struct Uuid {
    pub lo: u64,
    pub hi: u64,
}

impl Serde for Uuid {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([
            LeU64.map(field![Uuid, lo]).named("lo"),
            LeU64.map(field![Uuid, hi]).named("hi"),
        ])
        .erase()
    }
}
