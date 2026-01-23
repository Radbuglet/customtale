use bytes::{Buf, BufMut};
use uuid::Uuid;

use crate::serde::{Codec, ErasedCodec, Serde};

struct UuidCodec;

impl Codec for UuidCodec {
    type Target = Uuid;

    fn fixed_size(&self) -> Option<usize> {
        Some(16)
    }

    fn wants_non_null_bit(&self) -> bool {
        false
    }

    fn is_non_null_bit_set(&self, _target: &Self::Target) -> bool {
        true
    }

    fn decode(
        &self,
        target: &mut Self::Target,
        buf: &mut bytes::Bytes,
        _non_null_bit_set: bool,
    ) -> anyhow::Result<()> {
        let high_bits = buf.try_get_u64_le()?;
        let low_bits = buf.try_get_u64_le()?;

        *target = Uuid::from_u64_pair(high_bits, low_bits);

        Ok(())
    }

    fn encode(&self, target: &Self::Target, buf: &mut bytes::BytesMut) -> anyhow::Result<()> {
        let (high_bits, low_bits) = target.as_u64_pair();

        buf.put_u64_le(high_bits);
        buf.put_u64_le(low_bits);

        Ok(())
    }
}

impl Serde for Uuid {
    fn build_codec() -> ErasedCodec<Self> {
        UuidCodec.erase()
    }
}
