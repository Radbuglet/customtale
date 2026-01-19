use anyhow::Context;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use bytes_varint::{VarIntSupport, VarIntSupportMut};

use crate::Codec;

#[derive(Clone)]
pub struct LeU64;

impl Codec for LeU64 {
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
pub struct LeU32;

impl Codec for LeU32 {
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
pub struct LeU16;

impl Codec for LeU16 {
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
pub struct VarInt;

impl Codec for VarInt {
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
