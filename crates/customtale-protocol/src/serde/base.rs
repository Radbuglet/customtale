use std::{
    any::{Any, TypeId, type_name},
    collections::hash_map,
    fmt,
    marker::PhantomData,
    sync::{Arc, RwLock},
};

use anyhow::Context;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use derive_where::derive_where;
use enum_ordinalize::Ordinalize;
use rustc_hash::{FxBuildHasher, FxHashMap};

// === Field === //

#[derive_where(Copy, Clone, Hash, Eq, PartialEq)]
pub struct Field<I: ?Sized, O: ?Sized> {
    #[expect(clippy::type_complexity)]
    _ty: PhantomData<(fn(I) -> I, fn(O) -> O)>,
    offset: usize,
}

impl<I: ?Sized, O: ?Sized> fmt::Debug for Field<I, O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Offset({} -> {}, {})",
            type_name::<I>(),
            type_name::<O>(),
            self.offset
        )
    }
}

impl<I: ?Sized, O: ?Sized> Field<I, O> {
    #[expect(clippy::missing_safety_doc)]
    pub const unsafe fn new_unchecked(offset: usize) -> Self {
        Self {
            _ty: PhantomData,
            offset,
        }
    }

    pub const fn offset(self) -> usize {
        self.offset
    }

    pub const fn chain<P: ?Sized>(self, other: Field<O, P>) -> Field<I, P> {
        unsafe { Field::new_unchecked(self.offset + other.offset) }
    }

    pub const fn get_raw(self, input: *const I) -> *const O
    where
        O: Sized,
    {
        unsafe { input.cast::<u8>().add(self.offset).cast::<O>() }
    }

    pub const fn get_raw_mut(self, input: *mut I) -> *mut O
    where
        O: Sized,
    {
        self.get_raw(input).cast_mut()
    }

    pub const fn get(self, input: &I) -> &O
    where
        O: Sized,
    {
        unsafe { &*self.get_raw(input) }
    }

    pub const fn get_mut(self, input: &mut I) -> &mut O
    where
        O: Sized,
    {
        unsafe { &mut *self.get_raw_mut(input) }
    }
}

#[doc(hidden)]
pub mod field_internals {
    pub use std::mem::offset_of;

    use crate::serde::Field;

    pub const unsafe fn get_field<T: ?Sized, V: ?Sized>(
        _proof: fn(&T) -> &V,
        offset: usize,
    ) -> Field<T, V> {
        unsafe { Field::new_unchecked(offset) }
    }
}

#[macro_export]
macro_rules! field {
    ($Container:ty, $($fields:ident).+ $(,)?) => {
        unsafe {
            $crate::serde::field_internals::get_field::<$Container, _>(
                |v| &v.$($fields)*,
                $crate::serde::field_internals::offset_of!($Container, $($fields)*),
            )
        }
    };
}

pub use field;

// === Serde === //

static CODEC_CACHE: RwLock<FxHashMap<TypeId, Box<dyn Any + Send + Sync>>> =
    RwLock::new(FxHashMap::with_hasher(FxBuildHasher));

pub trait Serde: CodecValue {
    fn build_codec() -> ErasedCodec<Self>;

    fn codec() -> ErasedCodec<Self> {
        if let Some(codec) = CODEC_CACHE.read().unwrap().get(&TypeId::of::<Self>()) {
            return codec.downcast_ref::<ErasedCodec<Self>>().unwrap().clone();
        }

        let codec = Self::build_codec();

        match CODEC_CACHE.write().unwrap().entry(TypeId::of::<Self>()) {
            hash_map::Entry::Occupied(entry) => entry
                .get()
                .downcast_ref::<ErasedCodec<Self>>()
                .unwrap()
                .clone(),
            hash_map::Entry::Vacant(entry) => {
                entry.insert(Box::new(codec.clone()));
                codec
            }
        }
    }

    fn encode(&self, buf: &mut BytesMut) -> anyhow::Result<()> {
        Self::codec().encode(self, buf)
    }

    fn decode(mut data: Bytes) -> anyhow::Result<Self> {
        let mut target = Self::default();
        Self::codec()
            .decode(&mut target, &mut data, false)
            .with_context(|| format!("failed to parse packet\npacket thus far: {target:#?}"))?;
        Ok(target)
    }
}

// === Codec === //

pub trait CodecValue: 'static + Default + fmt::Debug + Clone {}

impl<T: 'static + Default + fmt::Debug + Clone> CodecValue for T {}

pub trait Codec: 'static + Send + Sync {
    type Target: CodecValue;

    fn fixed_size(&self) -> Option<usize>;

    fn wants_non_null_bit(&self) -> bool;

    fn is_non_null_bit_set(&self, target: &Self::Target) -> bool;

    fn decode(
        &self,
        target: &mut Self::Target,
        buf: &mut Bytes,
        non_null_bit_set: bool,
    ) -> anyhow::Result<()>;

    fn encode(&self, target: &Self::Target, buf: &mut BytesMut) -> anyhow::Result<()>;

    fn map<I: CodecValue>(self, field: Field<I, Self::Target>) -> ErasedCodec<I>
    where
        Self: Sized,
    {
        MapCodec::new(field, self.erase()).erase()
    }

    fn nullable_fixed(self) -> ErasedCodec<Option<Self::Target>>
    where
        Self: Sized,
    {
        FixedNullableCodec::new(self.erase()).erase()
    }

    fn nullable_variable(self) -> ErasedCodec<Option<Self::Target>>
    where
        Self: Sized,
    {
        VariableNullableCodec::new(self.erase()).erase()
    }

    fn erase(self) -> ErasedCodec<Self::Target>
    where
        Self: Sized,
    {
        ErasedCodec::new(self)
    }

    fn named(self, name: &'static str) -> NamedCodec<Self::Target>
    where
        Self: Sized,
    {
        NamedCodec {
            name,
            codec: self.erase(),
        }
    }
}

#[derive_where(Clone)]
pub struct NamedCodec<T: CodecValue> {
    pub name: &'static str,
    pub codec: ErasedCodec<T>,
}

#[derive_where(Clone)]
pub struct ErasedCodec<T: CodecValue>(Arc<dyn Codec<Target = T>>);

impl<T: CodecValue> ErasedCodec<T> {
    pub fn new<C: Codec<Target = T>>(codec: C) -> Self {
        Self(Arc::new(codec))
    }
}

impl<T: CodecValue> Codec for ErasedCodec<T> {
    type Target = T;

    fn fixed_size(&self) -> Option<usize> {
        self.0.fixed_size()
    }

    fn wants_non_null_bit(&self) -> bool {
        self.0.wants_non_null_bit()
    }

    fn is_non_null_bit_set(&self, target: &Self::Target) -> bool {
        self.0.is_non_null_bit_set(target)
    }

    fn decode(
        &self,
        target: &mut Self::Target,
        buf: &mut Bytes,
        non_null_bit_set: bool,
    ) -> anyhow::Result<()> {
        self.0.decode(target, buf, non_null_bit_set)
    }

    fn encode(&self, target: &Self::Target, buf: &mut BytesMut) -> anyhow::Result<()> {
        self.0.encode(target, buf)
    }

    fn erase(self) -> ErasedCodec<Self::Target>
    where
        Self: Sized,
    {
        self
    }
}

// === MapCodec === //

#[derive_where(Clone)]
pub struct MapCodec<I: CodecValue, O: CodecValue> {
    field: Field<I, O>,
    inner: ErasedCodec<O>,
}

impl<I: CodecValue, O: CodecValue> MapCodec<I, O> {
    pub fn new(field: Field<I, O>, inner: ErasedCodec<O>) -> Self {
        Self { field, inner }
    }
}

impl<I: CodecValue, O: CodecValue> Codec for MapCodec<I, O> {
    type Target = I;

    fn fixed_size(&self) -> Option<usize> {
        self.inner.fixed_size()
    }

    fn wants_non_null_bit(&self) -> bool {
        self.inner.wants_non_null_bit()
    }

    fn is_non_null_bit_set(&self, target: &Self::Target) -> bool {
        self.inner.is_non_null_bit_set(self.field.get(target))
    }

    fn decode(
        &self,
        target: &mut Self::Target,
        buf: &mut Bytes,
        non_null_bit_set: bool,
    ) -> anyhow::Result<()> {
        self.inner
            .decode(self.field.get_mut(target), buf, non_null_bit_set)
    }

    fn encode(&self, target: &Self::Target, buf: &mut BytesMut) -> anyhow::Result<()> {
        self.inner.encode(self.field.get(target), buf)
    }
}

// === FixedNullableCodec === //

#[derive_where(Clone)]
pub struct FixedNullableCodec<T: CodecValue> {
    inner: ErasedCodec<T>,
}

impl<T: CodecValue> FixedNullableCodec<T> {
    pub fn new(inner: ErasedCodec<T>) -> Self {
        assert!(inner.fixed_size().is_some());
        assert!(!inner.wants_non_null_bit());

        Self { inner }
    }
}

impl<T: CodecValue> Codec for FixedNullableCodec<T> {
    type Target = Option<T>;

    fn fixed_size(&self) -> Option<usize> {
        self.inner.fixed_size()
    }

    fn wants_non_null_bit(&self) -> bool {
        true
    }

    fn is_non_null_bit_set(&self, target: &Self::Target) -> bool {
        target.is_some()
    }

    fn decode(
        &self,
        target: &mut Self::Target,
        buf: &mut Bytes,
        non_null_bit_set: bool,
    ) -> anyhow::Result<()> {
        if non_null_bit_set {
            self.inner
                .decode(target.insert(T::default()), buf, non_null_bit_set)
        } else {
            buf.advance(self.fixed_size().unwrap());
            Ok(())
        }
    }

    fn encode(&self, target: &Self::Target, buf: &mut BytesMut) -> anyhow::Result<()> {
        if let Some(target) = target {
            self.inner.encode(target, buf)
        } else {
            buf.put_bytes(0, self.fixed_size().unwrap());
            Ok(())
        }
    }
}

// === VariableNullableCodec === //

#[derive_where(Clone)]
pub struct VariableNullableCodec<T: CodecValue> {
    inner: ErasedCodec<T>,
}

impl<T: CodecValue> VariableNullableCodec<T> {
    pub fn new(inner: ErasedCodec<T>) -> Self {
        assert!(!inner.wants_non_null_bit());

        Self { inner }
    }
}

impl<T: CodecValue> Codec for VariableNullableCodec<T> {
    type Target = Option<T>;

    fn fixed_size(&self) -> Option<usize> {
        self.inner.fixed_size()
    }

    fn wants_non_null_bit(&self) -> bool {
        true
    }

    fn is_non_null_bit_set(&self, target: &Self::Target) -> bool {
        target.is_some()
    }

    fn decode(
        &self,
        target: &mut Self::Target,
        buf: &mut Bytes,
        non_null_bit_set: bool,
    ) -> anyhow::Result<()> {
        if non_null_bit_set {
            self.inner
                .decode(target.insert(T::default()), buf, non_null_bit_set)
        } else {
            Ok(())
        }
    }

    fn encode(&self, target: &Self::Target, buf: &mut BytesMut) -> anyhow::Result<()> {
        if let Some(target) = target {
            self.inner.encode(target, buf)
        } else {
            Ok(())
        }
    }
}

// === StructCodec === //

#[derive_where(Clone)]
pub struct StructCodec<T: CodecValue> {
    fixed_total_size: Option<usize>,
    null_bytes: usize,
    fixed_fields: Vec<StructField<T>>,
    variable_fields: Vec<StructField<T>>,
}

#[derive_where(Clone)]
struct StructField<T: CodecValue> {
    name: &'static str,
    non_null_bit_idx: Option<usize>,
    codec: ErasedCodec<T>,
}

impl<T: CodecValue> StructField<T> {
    fn is_non_null_bit_set(&self, non_null_bits: &[u8]) -> bool {
        self.non_null_bit_idx
            .map(|non_null_bit_idx| {
                non_null_bits[non_null_bit_idx / 8] >> (non_null_bit_idx % 8) != 0
            })
            .unwrap_or(true)
    }
}

impl<T: CodecValue> StructCodec<T> {
    pub fn new(fields: impl IntoIterator<Item = NamedCodec<T>>) -> Self {
        let mut fixed_total_size = Some(0);
        let mut non_null_bits = 0;
        let mut fixed_fields = Vec::new();
        let mut variable_fields = Vec::new();

        for NamedCodec { name, codec: field } in fields {
            let non_null_bit_idx = if field.wants_non_null_bit() {
                let idx = non_null_bits;
                non_null_bits += 1;
                Some(idx)
            } else {
                None
            };

            if let Some(size) = field.fixed_size() {
                fixed_fields.push(StructField {
                    name,
                    non_null_bit_idx,
                    codec: field,
                });

                if let Some(fixed_total_size) = &mut fixed_total_size {
                    *fixed_total_size += size;
                }
            } else {
                variable_fields.push(StructField {
                    name,
                    non_null_bit_idx,
                    codec: field,
                });

                fixed_total_size = None;
            }
        }

        Self {
            fixed_total_size,
            null_bytes: non_null_bits.div_ceil(8),
            fixed_fields,
            variable_fields,
        }
    }
}

impl<T: CodecValue> Codec for StructCodec<T> {
    type Target = T;

    fn fixed_size(&self) -> Option<usize> {
        self.fixed_total_size
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
        if buf.remaining() < self.null_bytes {
            anyhow::bail!("not enough null bytes for `{}`", type_name::<T>());
        }

        let null_bits = buf.split_to(self.null_bytes);

        for fixed in &self.fixed_fields {
            fixed
                .codec
                .decode(target, buf, fixed.is_non_null_bit_set(&null_bits))
                .with_context(|| {
                    format!(
                        "failed to decode fixed field `{}.{}`",
                        type_name::<T>(),
                        fixed.name,
                    )
                })?;
        }

        if let [unique] = &self.variable_fields[..] {
            unique
                .codec
                .decode(target, buf, unique.is_non_null_bit_set(&null_bits))
                .with_context(|| {
                    format!(
                        "failed to decode unique variable field `{}.{}`",
                        type_name::<T>(),
                        unique.name,
                    )
                })?
        } else {
            let mut max_variable_end = 0;

            let mut fixed_buf = buf.clone();

            if buf.remaining() < self.variable_fields.len() * 4 {
                anyhow::bail!(
                    "variable offset section for `{}` not long enough",
                    type_name::<T>()
                );
            }

            buf.advance(self.variable_fields.len() * 4);

            for field in &self.variable_fields {
                let offset = fixed_buf.get_u32_le();

                if !field.is_non_null_bit_set(&null_bits) {
                    continue;
                }

                max_variable_end = max_variable_end.max(offset);

                let mut buf = buf.clone();

                if buf.remaining() < offset as usize {
                    anyhow::bail!(
                        "variable data section for `{}.{}` not long enough (section has size {} but offset was {})",
                        type_name::<T>(),
                        field.name,
                        buf.remaining(),
                        offset as i32,
                    );
                }

                buf.advance(offset as usize);

                field
                    .codec
                    .decode(target, &mut buf, field.is_non_null_bit_set(&null_bits))
                    .with_context(|| {
                        format!(
                            "failed to decode variable field `{}.{}`",
                            type_name::<T>(),
                            field.name,
                        )
                    })?;
            }

            buf.advance(max_variable_end as usize);
        }

        Ok(())
    }

    fn encode(&self, target: &Self::Target, buf: &mut BytesMut) -> anyhow::Result<()> {
        let null_bytes_len = buf.len();
        buf.put_bytes(0, self.null_bytes);

        for field in self.fixed_fields.iter().chain(&self.variable_fields) {
            let Some(non_null_bit_idx) = field.non_null_bit_idx else {
                continue;
            };

            if field.codec.is_non_null_bit_set(target) {
                buf[null_bytes_len..][non_null_bit_idx / 8] |= 1 << (non_null_bit_idx % 8);
            }
        }

        for field in &self.fixed_fields {
            field.codec.encode(target, buf)?;
        }

        if let [unique] = &self.variable_fields[..] {
            unique.codec.encode(target, buf)?;
        } else {
            let offsets_start_len = buf.len();
            buf.put_bytes(0, 4 * self.variable_fields.len());

            let variable_start_len = buf.len();

            for (idx, field) in self.variable_fields.iter().enumerate() {
                if !field.codec.is_non_null_bit_set(target) {
                    buf[offsets_start_len..][(idx * 4)..][..4]
                        .copy_from_slice(&(-1i32).to_le_bytes());

                    continue;
                }

                let own_offset = (buf.len() - variable_start_len) as u32;

                buf[offsets_start_len..][(idx * 4)..][..4]
                    .copy_from_slice(&own_offset.to_le_bytes());

                field.codec.encode(target, buf)?;
            }
        }

        Ok(())
    }
}

// === EnumCodec === //

pub trait SimpleEnum: Ordinalize<VariantType = u8> + Default + Copy + fmt::Debug {}

impl<T: Ordinalize<VariantType = u8> + Default + Copy + fmt::Debug> SimpleEnum for T {}

#[derive_where(Clone, Default)]
pub struct EnumCodec<T: SimpleEnum> {
    _ty: PhantomData<fn(T) -> T>,
}

impl<T: SimpleEnum> EnumCodec<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T: SimpleEnum> Codec for EnumCodec<T> {
    type Target = T;

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
        let id = buf
            .try_get_u8()
            .with_context(|| format!("missing variant data for `{}`", type_name::<T>()))?;
        *target = *T::VARIANTS
            .get(id as usize)
            .with_context(|| format!("no such variant {id} for `{}`", type_name::<T>()))?;

        Ok(())
    }

    fn encode(&self, target: &Self::Target, buf: &mut BytesMut) -> anyhow::Result<()> {
        buf.put_u8(target.ordinal());

        Ok(())
    }
}
