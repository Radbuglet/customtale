use anyhow::Context;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use bytes_varint::{VarIntSupport, VarIntSupportMut};
use derive_where::derive_where;

use crate::serde::{Codec, CodecValue, ErasedCodec, Serde};

// === Dictionary === //

#[derive(Debug, Clone)]
#[derive_where(Default)]
pub struct Dictionary<K, V> {
    pub entries: Vec<DictionaryEntry<K, V>>,
}

#[derive(Debug, Clone)]
pub struct DictionaryEntry<K, V> {
    pub key: K,
    pub value: V,
}

// === Containers === //

#[derive_where(Clone)]
pub struct VarDictionaryCodec<K: CodecValue, V: CodecValue> {
    key_codec: ErasedCodec<K>,
    value_codec: ErasedCodec<V>,
    max_len: u32,
}

impl<K: CodecValue, V: CodecValue> VarDictionaryCodec<K, V> {
    pub fn new(key_codec: ErasedCodec<K>, value_codec: ErasedCodec<V>, max_len: u32) -> Self {
        Self {
            key_codec,
            value_codec,
            max_len,
        }
    }
}

impl<K: CodecValue, V: CodecValue> Codec for VarDictionaryCodec<K, V> {
    type Target = Dictionary<K, V>;

    fn fixed_size(&self) -> Option<usize> {
        None
    }

    fn decode(
        &self,
        target: &mut Self::Target,
        buf: &mut Bytes,
        _non_null_bit_set: bool,
    ) -> anyhow::Result<()> {
        let len = buf
            .try_get_u32_varint()
            .context("failed to get dictionary length")?;

        if len > self.max_len {
            anyhow::bail!(
                "map of length {len} exceeds maximum length of {}",
                self.max_len
            );
        }

        for _ in 0..len {
            let mut key = K::default();
            self.key_codec
                .decode(&mut key, buf, false)
                .context("failed to read map key")?;

            let mut value = V::default();
            self.value_codec
                .decode(&mut value, buf, false)
                .context("failed to read map key")?;

            target.entries.push(DictionaryEntry { key, value });
        }

        Ok(())
    }

    fn encode(&self, target: &Self::Target, buf: &mut BytesMut) -> anyhow::Result<()> {
        if target.entries.len() > self.max_len as usize {
            anyhow::bail!(
                "map of length {} exceeds maximum length of {}",
                target.entries.len(),
                self.max_len
            );
        }

        buf.put_u32_varint(target.entries.len() as u32);

        for DictionaryEntry { key, value } in &target.entries {
            self.key_codec.encode(key, buf)?;
            self.value_codec.encode(value, buf)?;
        }

        Ok(())
    }
}

impl<K: Serde, V: Serde> Serde for Dictionary<K, V> {
    fn build_codec() -> ErasedCodec<Self> {
        VarDictionaryCodec::new(K::codec(), V::codec(), 4096000).erase()
    }
}

#[derive_where(Clone)]
pub struct VarArrayCodec<T: CodecValue> {
    codec: ErasedCodec<T>,
    max_len: u32,
}

impl<T: CodecValue> VarArrayCodec<T> {
    pub fn new(codec: ErasedCodec<T>, max_len: u32) -> Self {
        Self { codec, max_len }
    }
}

impl<T: CodecValue> Codec for VarArrayCodec<T> {
    type Target = Vec<T>;

    fn fixed_size(&self) -> Option<usize> {
        None
    }

    fn decode(
        &self,
        target: &mut Self::Target,
        buf: &mut Bytes,
        _non_null_bit_set: bool,
    ) -> anyhow::Result<()> {
        let len = buf
            .try_get_u32_varint()
            .context("failed to read array length")?;

        if len > self.max_len {
            anyhow::bail!(
                "array had length {len} but the maximum allowed array length was {}",
                self.max_len
            );
        }

        target.resize_with(len as usize, T::default);

        for target in target {
            self.codec.decode(target, buf, false)?;
        }

        Ok(())
    }

    fn encode(&self, target: &Self::Target, buf: &mut BytesMut) -> anyhow::Result<()> {
        if target.len() > self.max_len as usize {
            anyhow::bail!(
                "array had length {} but the maximum allowed array length was {}",
                target.len(),
                self.max_len
            );
        }

        buf.put_u32_varint(target.len() as u32);

        for target in target {
            self.codec.encode(target, buf)?;
        }

        Ok(())
    }
}

impl<T: Serde> Serde for Vec<T> {
    fn build_codec() -> ErasedCodec<Self> {
        VarArrayCodec::new(T::codec(), 4096000).erase()
    }
}

// === Byte Arrays === //

#[derive(Clone)]
pub struct ExactByteArrayCodec {
    size: u32,
}

impl ExactByteArrayCodec {
    pub fn new(size: u32) -> Self {
        Self { size }
    }
}

impl Codec for ExactByteArrayCodec {
    type Target = Bytes;

    fn fixed_size(&self) -> Option<usize> {
        Some(self.size as usize)
    }

    fn decode(
        &self,
        target: &mut Self::Target,
        buf: &mut Bytes,
        _non_null_bit_set: bool,
    ) -> anyhow::Result<()> {
        if buf.remaining() < self.size as usize {
            anyhow::bail!(
                "need {} byte(s) for array but got {}",
                self.size,
                buf.remaining()
            );
        }

        *target = buf.split_to(self.size as usize);

        Ok(())
    }

    fn encode(&self, target: &Self::Target, buf: &mut BytesMut) -> anyhow::Result<()> {
        if target.len() != self.size as usize {
            anyhow::bail!(
                "expected an array of size {} but got {}",
                self.size,
                target.len()
            );
        }

        buf.put_slice(&target[..]);

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

// === Strings === //

#[derive(Clone)]
pub struct FixedSizeStringCodec {
    size: u32,
}

impl FixedSizeStringCodec {
    pub fn new(size: u32) -> Self {
        Self { size }
    }
}

impl Codec for FixedSizeStringCodec {
    type Target = String;

    fn fixed_size(&self) -> Option<usize> {
        Some(self.size as usize)
    }

    fn decode(
        &self,
        target: &mut Self::Target,
        buf: &mut Bytes,
        _non_null_bit_set: bool,
    ) -> anyhow::Result<()> {
        if buf.len() < self.size as usize {
            anyhow::bail!(
                "need {} byte(s) for string but got {}",
                self.size,
                buf.remaining()
            );
        }

        let buf = buf.split_to(self.size as usize);
        let len = buf.iter().position(|&v| v == 0).unwrap_or(buf.len());

        *target = String::from_utf8(buf[..len].to_vec()).context("string was not valid UTF-8")?;

        Ok(())
    }

    fn encode(&self, target: &Self::Target, buf: &mut BytesMut) -> anyhow::Result<()> {
        if target.as_bytes().contains(&0) {
            anyhow::bail!("interior NUL byte");
        }

        if target.len() > self.size as usize {
            anyhow::bail!("string too long");
        }

        buf.put_slice(target.as_bytes());
        buf.put_bytes(0, self.size as usize - target.len());

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

impl Serde for String {
    fn build_codec() -> ErasedCodec<Self> {
        VarStringCodec::new(4096000).erase()
    }
}
