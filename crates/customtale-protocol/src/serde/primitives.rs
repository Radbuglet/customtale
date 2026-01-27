use anyhow::Context;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use bytes_varint::{VarIntSupport, VarIntSupportMut};
use uuid::Uuid;

use crate::serde::{Codec, ErasedCodec, Serde};

#[derive(Clone)]
pub struct ByteBoolCodec;

impl Codec for ByteBoolCodec {
    type Target = bool;

    fn fixed_size(&self) -> Option<usize> {
        Some(1)
    }

    fn decode(
        &self,
        target: &mut Self::Target,
        buf: &mut Bytes,
        _non_null_bit_set: bool,
    ) -> anyhow::Result<()> {
        match buf
            .try_get_u8()
            .context("failed to read byte for boolean")?
        {
            0 => *target = false,
            1 => *target = true,
            v => anyhow::bail!("unknown boolean variant {v}"),
        }

        Ok(())
    }

    fn encode(&self, target: &Self::Target, buf: &mut BytesMut) -> anyhow::Result<()> {
        buf.put_u8(*target as u8);

        Ok(())
    }
}

impl Serde for bool {
    const OPTION_IS_FIXED: bool = true;

    fn build_codec() -> ErasedCodec<Self> {
        ByteBoolCodec.erase()
    }
}

#[derive(Clone)]
pub struct ByteCodec;

impl Codec for ByteCodec {
    type Target = u8;

    fn fixed_size(&self) -> Option<usize> {
        Some(1)
    }

    fn decode(
        &self,
        target: &mut Self::Target,
        buf: &mut Bytes,
        _non_null_bit_set: bool,
    ) -> anyhow::Result<()> {
        *target = buf.try_get_u8().context("failed to read byte")?;

        Ok(())
    }

    fn encode(&self, target: &Self::Target, buf: &mut BytesMut) -> anyhow::Result<()> {
        buf.put_u8(*target);

        Ok(())
    }
}

impl Serde for u8 {
    const OPTION_IS_FIXED: bool = true;

    fn build_codec() -> ErasedCodec<Self> {
        ByteCodec.erase()
    }
}

#[derive(Clone)]
pub struct LeU64Codec;

impl Codec for LeU64Codec {
    type Target = u64;

    fn fixed_size(&self) -> Option<usize> {
        Some(8)
    }

    fn decode(
        &self,
        target: &mut Self::Target,
        buf: &mut Bytes,
        _non_null_bit_set: bool,
    ) -> anyhow::Result<()> {
        *target = buf.try_get_u64_le().context("failed to read LeU64")?;

        Ok(())
    }

    fn encode(&self, target: &Self::Target, buf: &mut BytesMut) -> anyhow::Result<()> {
        buf.put_u64_le(*target);

        Ok(())
    }
}

impl Serde for u64 {
    const OPTION_IS_FIXED: bool = true;

    fn build_codec() -> ErasedCodec<Self> {
        LeU64Codec.erase()
    }
}

#[derive(Clone)]
pub struct LeU32Codec;

impl Codec for LeU32Codec {
    type Target = u32;

    fn fixed_size(&self) -> Option<usize> {
        Some(4)
    }

    fn decode(
        &self,
        target: &mut Self::Target,
        buf: &mut Bytes,
        _non_null_bit_set: bool,
    ) -> anyhow::Result<()> {
        *target = buf.try_get_u32_le().context("failed to read LeU32")?;

        Ok(())
    }

    fn encode(&self, target: &Self::Target, buf: &mut BytesMut) -> anyhow::Result<()> {
        buf.put_u32_le(*target);

        Ok(())
    }
}

impl Serde for u32 {
    const OPTION_IS_FIXED: bool = true;

    fn build_codec() -> ErasedCodec<Self> {
        LeU32Codec.erase()
    }
}

#[derive(Clone)]
pub struct LeI32Codec;

impl Codec for LeI32Codec {
    type Target = i32;

    fn fixed_size(&self) -> Option<usize> {
        Some(4)
    }

    fn decode(
        &self,
        target: &mut Self::Target,
        buf: &mut Bytes,
        _non_null_bit_set: bool,
    ) -> anyhow::Result<()> {
        *target = buf.try_get_i32_le().context("failed to read LeU32")?;

        Ok(())
    }

    fn encode(&self, target: &Self::Target, buf: &mut BytesMut) -> anyhow::Result<()> {
        buf.put_i32_le(*target);

        Ok(())
    }
}

impl Serde for i32 {
    const OPTION_IS_FIXED: bool = true;

    fn build_codec() -> ErasedCodec<Self> {
        LeI32Codec.erase()
    }
}

#[derive(Clone)]
pub struct LeU16Codec;

impl Codec for LeU16Codec {
    type Target = u16;

    fn fixed_size(&self) -> Option<usize> {
        Some(2)
    }

    fn decode(
        &self,
        target: &mut Self::Target,
        buf: &mut Bytes,
        _non_null_bit_set: bool,
    ) -> anyhow::Result<()> {
        *target = buf.try_get_u16_le().context("failed to read LeU16")?;

        Ok(())
    }

    fn encode(&self, target: &Self::Target, buf: &mut BytesMut) -> anyhow::Result<()> {
        buf.put_u16_le(*target);

        Ok(())
    }
}

impl Serde for u16 {
    const OPTION_IS_FIXED: bool = true;

    fn build_codec() -> ErasedCodec<Self> {
        LeU16Codec.erase()
    }
}

#[derive(Clone)]
pub struct LeF64Codec;

impl Codec for LeF64Codec {
    type Target = f64;

    fn fixed_size(&self) -> Option<usize> {
        Some(8)
    }

    fn decode(
        &self,
        target: &mut Self::Target,
        buf: &mut Bytes,
        _non_null_bit_set: bool,
    ) -> anyhow::Result<()> {
        *target = buf.try_get_f64_le().context("failed to read LeF32")?;

        Ok(())
    }

    fn encode(&self, target: &Self::Target, buf: &mut BytesMut) -> anyhow::Result<()> {
        buf.put_f64_le(*target);
        Ok(())
    }
}

impl Serde for f64 {
    const OPTION_IS_FIXED: bool = true;

    fn build_codec() -> ErasedCodec<Self> {
        LeF64Codec.erase()
    }
}

#[derive(Clone)]
pub struct LeF32Codec;

impl Codec for LeF32Codec {
    type Target = f32;

    fn fixed_size(&self) -> Option<usize> {
        Some(4)
    }

    fn decode(
        &self,
        target: &mut Self::Target,
        buf: &mut Bytes,
        _non_null_bit_set: bool,
    ) -> anyhow::Result<()> {
        *target = buf.try_get_f32_le().context("failed to read LeF32")?;

        Ok(())
    }

    fn encode(&self, target: &Self::Target, buf: &mut BytesMut) -> anyhow::Result<()> {
        buf.put_f32_le(*target);
        Ok(())
    }
}

impl Serde for f32 {
    const OPTION_IS_FIXED: bool = true;

    fn build_codec() -> ErasedCodec<Self> {
        LeF32Codec.erase()
    }
}

#[derive(Clone)]
pub struct VarIntCodec;

impl Codec for VarIntCodec {
    type Target = u32;

    fn fixed_size(&self) -> Option<usize> {
        None
    }

    fn decode(
        &self,
        target: &mut Self::Target,
        buf: &mut Bytes,
        _non_null_bit_set: bool,
    ) -> anyhow::Result<()> {
        *target = buf.try_get_u32_varint().context("failed to read VarInt")?;

        Ok(())
    }

    fn encode(&self, target: &Self::Target, buf: &mut BytesMut) -> anyhow::Result<()> {
        buf.put_u32_varint(*target);

        Ok(())
    }
}

// === UUID === //

struct UuidCodec;

impl Codec for UuidCodec {
    type Target = Uuid;

    fn fixed_size(&self) -> Option<usize> {
        Some(16)
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
    const OPTION_IS_FIXED: bool = true;

    fn build_codec() -> ErasedCodec<Self> {
        UuidCodec.erase()
    }
}
