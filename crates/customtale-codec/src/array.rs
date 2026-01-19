use anyhow::Context;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use bytes_varint::{VarIntSupport, VarIntSupportMut};

use crate::{Codec, ErasedCodec, Serde};

#[derive(Debug, Clone)]
pub struct FixedByteArray<const N: usize>(pub Box<[u8; N]>);

impl<const N: usize> Default for FixedByteArray<N> {
    fn default() -> Self {
        Self(Box::new([0; N]))
    }
}

impl<const N: usize> Serde for FixedByteArray<N> {
    fn build_codec() -> ErasedCodec<Self> {
        FixedByteArrayCodec.erase()
    }
}

pub struct FixedByteArrayCodec<const N: usize>;

impl<const N: usize> Codec for FixedByteArrayCodec<N> {
    type Target = FixedByteArray<N>;

    fn fixed_size(&self) -> Option<usize> {
        Some(N)
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
        if buf.remaining() < N {
            anyhow::bail!("need {N} byte(s) for array but got {}", buf.remaining());
        }

        target.0.copy_from_slice(&buf[0..N]);
        buf.advance(N);

        Ok(())
    }

    fn encode(&self, target: &Self::Target, buf: &mut BytesMut) -> anyhow::Result<()> {
        buf.put_slice(&target.0[..]);

        Ok(())
    }
}

#[derive(Clone)]
pub struct NulTerminatedStringCodec {
    max_len: u32,
}

impl NulTerminatedStringCodec {
    pub fn new(max_len: u32) -> Self {
        Self { max_len }
    }
}

impl Codec for NulTerminatedStringCodec {
    type Target = String;

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
        let mut accum = Vec::new();

        for _ in 0..self.max_len {
            let ch = buf
                .try_get_u8()
                .with_context(|| format!("unterminated string, got {:?}", accum))?;

            if ch == 0 {
                break;
            }

            accum.push(ch);
        }

        *target = String::from_utf8(accum).context("string was not valid UTF-8")?;

        Ok(())
    }

    fn encode(&self, target: &Self::Target, buf: &mut BytesMut) -> anyhow::Result<()> {
        if target.len() > self.max_len as usize {
            anyhow::bail!("string too long");
        }

        for &ch in target.as_bytes() {
            if ch == 0 {
                anyhow::bail!("interior NUL byte");
            }

            buf.put_u8(ch);
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct VarStringCodec {
    max_len: u32,
}

impl VarStringCodec {
    pub fn new(max_len: u32) -> Self {
        Self { max_len }
    }
}

impl Codec for VarStringCodec {
    type Target = String;

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
        let len = buf.try_get_u32_varint().context("missing string length")?;

        if len > self.max_len {
            anyhow::bail!("string too long");
        }

        if buf.remaining() < len as usize {
            anyhow::bail!("buffer not long enough for string");
        }

        *target = String::from_utf8(buf[..len as usize].to_vec()).context("invalid UTF-8")?;

        buf.advance(len as usize);

        Ok(())
    }

    fn encode(&self, target: &Self::Target, buf: &mut BytesMut) -> anyhow::Result<()> {
        if target.len() > self.max_len as usize {
            anyhow::bail!("string too long");
        }

        buf.put_u32_varint(target.len() as u32);
        buf.put_slice(target.as_bytes());

        Ok(())
    }
}

#[derive(Clone)]
pub struct VarByteArrayCodec {
    max_len: u32,
}

impl VarByteArrayCodec {
    pub fn new(max_len: u32) -> Self {
        Self { max_len }
    }
}

impl Codec for VarByteArrayCodec {
    type Target = Bytes;

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
        let len = buf
            .try_get_u32_varint()
            .context("failed to read byte array length")?;

        if len > self.max_len {
            anyhow::bail!("byte array is beyond maximum allowed length");
        }

        if buf.remaining() < len as usize {
            anyhow::bail!("not enough bytes for byte array");
        }

        *target = buf.split_off(len as usize);

        Ok(())
    }

    fn encode(&self, target: &Self::Target, buf: &mut BytesMut) -> anyhow::Result<()> {
        if target.len() > self.max_len as usize {
            anyhow::bail!("byte array too long");
        }

        buf.put_u32_varint(target.len() as u32);
        buf.put_slice(target);

        Ok(())
    }
}
