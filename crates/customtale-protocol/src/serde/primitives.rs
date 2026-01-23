use anyhow::Context;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use bytes_varint::{VarIntSupport, VarIntSupportMut};

use crate::serde::Codec;

#[derive(Clone)]
pub struct ByteBoolCodec;

impl Codec for ByteBoolCodec {
    type Target = bool;

    fn fixed_size(&self) -> Option<usize> {
        Some(1)
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

#[derive(Clone)]
pub struct LeU64Codec;

impl Codec for LeU64Codec {
    type Target = u64;

    fn fixed_size(&self) -> Option<usize> {
        Some(8)
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

#[derive(Clone)]
pub struct LeU32Codec;

impl Codec for LeU32Codec {
    type Target = u32;

    fn fixed_size(&self) -> Option<usize> {
        Some(4)
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

#[derive(Clone)]
pub struct LeU16Codec;

impl Codec for LeU16Codec {
    type Target = u16;

    fn fixed_size(&self) -> Option<usize> {
        Some(2)
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

#[derive(Clone)]
pub struct LeF64Codec;

impl Codec for LeF64Codec {
    type Target = f64;

    fn fixed_size(&self) -> Option<usize> {
        Some(8)
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

#[derive(Clone)]
pub struct LeF32Codec;

impl Codec for LeF32Codec {
    type Target = f32;

    fn fixed_size(&self) -> Option<usize> {
        Some(4)
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

#[derive(Clone)]
pub struct VarIntCodec;

impl Codec for VarIntCodec {
    type Target = u32;

    fn fixed_size(&self) -> Option<usize> {
        None
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
