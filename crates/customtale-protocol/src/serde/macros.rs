#[doc(hidden)]
pub mod codec_internals {
    pub use {
        super::codec,
        crate::serde::{Codec, EnumCodec, ErasedCodec, Serde, StructCodec, field},
        anyhow,
        bytes::{Bytes, BytesMut},
        bytes_varint::{VarIntSupport, VarIntSupportMut},
        enum_ordinalize::Ordinalize,
        std::{
            boxed::Box,
            clone::Clone,
            cmp::{Eq, PartialEq},
            default::Default,
            fmt::Debug,
            hash::Hash,
            marker::{Copy, Sized},
            mem::transmute,
            option::Option,
            primitive::{bool, u8, usize},
            stringify,
        },
    };
}

#[macro_export]
macro_rules! codec {
    (
        $(
            $(#[$($meta:tt)*])*
            $vis:vis $kind:ident $name:ident {
                $($body:tt)*
            }
        )*
    ) => {$(
        $crate::serde::codec_internals::codec! {
            @internal

            $(#[$($meta)*])*
            $vis $kind $name {
                $($body)*
            }
        }
    )*};
    (
        @internal

        $(#[$($meta:tt)*])*
        $vis:vis struct $name:ident {
            $(@small = $small:literal;)?
            $(
                $(#[$($field_meta:tt)*])*
                $field_vis:vis $field:ident: $ty:ty $(=> $custom:expr)?
            ),*
            $(,)?
        }
    ) => {
        #[derive(
            $crate::serde::codec_internals::Debug,
            $crate::serde::codec_internals::Clone,
            $crate::serde::codec_internals::Default,
        )]
        $(#[$($meta)*])*
        $vis struct $name {
            $(
                $(#[$($field_meta)*])*
                $field_vis $field: $ty,
            )*
        }

        impl $crate::serde::codec_internals::Serde for $name {
            $(const OPTION_IS_FIXED: $crate::serde::codec_internals::bool = $small;)?

            fn build_codec() -> $crate::serde::codec_internals::ErasedCodec<Self> {
                $crate::serde::codec_internals::Codec::erase(
                    $crate::serde::codec_internals::StructCodec::<Self>::new([
                        $(
                            $crate::serde::codec_internals::Codec::named(
                                $crate::serde::codec_internals::Codec::field(
                                    $crate::serde::codec_internals::codec!(
                                        @pick_first
                                        $({ $custom })?
                                        { <$ty as $crate::serde::codec_internals::Serde>::codec() }
                                    ),
                                    $crate::serde::codec_internals::field![$name, $field],
                                ),
                                $crate::serde::codec_internals::stringify!($field),
                            ),
                        )*
                    ]),
                )
            }
        }
    };
    (
        @internal

        $(#[$($meta:tt)*])*
        $vis:vis union $name:ident {
            $(
                $(#[$($variant_meta:tt)*])*
                $variant:ident($ty:ty) $(=> $custom:expr)?
            ),*
            $(,)?
        }
    ) => {
        #[derive(
            $crate::serde::codec_internals::Debug,
            $crate::serde::codec_internals::Clone,
        )]
        $(#[$($meta)*])*
        $vis enum $name {
            $(
                $(#[$($variant_meta)*])*
                $variant($crate::serde::codec_internals::Box<$ty>),
            )*
        }

        #[allow(unreachable_code)]
        impl $crate::serde::codec_internals::Default for $name {
            fn default() -> Self {
                $(
                    return Self::$variant(
                        <$crate::serde::codec_internals::Box<$ty> as $crate::serde::codec_internals::Default>::default()
                    );
                )*
            }
        }

        const _: () = {
            struct UnionCodec;

            impl $crate::serde::codec_internals::Codec for UnionCodec {
                type Target = $name;

                fn fixed_size(&self) -> $crate::serde::codec_internals::Option<$crate::serde::codec_internals::usize> {
                    $crate::serde::codec_internals::Option::None
                }

                fn decode(
                    &self,
                    target: &mut $name,
                    buf: &mut $crate::serde::codec_internals::Bytes,
                    _non_null_bit_set: $crate::serde::codec_internals::bool,
                ) -> $crate::serde::codec_internals::anyhow::Result<()> {
                    let mut type_id = $crate::serde::codec_internals::VarIntSupport::try_get_u32_varint(buf)?;

                    $(
                        if type_id == 0 {
                            let mut value =
                                <$crate::serde::codec_internals::Box<$ty> as $crate::serde::codec_internals::Default>::default();

                            $crate::serde::codec_internals::Codec::decode(
                                &$crate::serde::codec_internals::codec!(
                                    @pick_first
                                    $({ $custom })?
                                    { <$ty as $crate::serde::codec_internals::Serde>::codec() }
                                ),
                                &mut value,
                                buf,
                                true,
                            )?;

                            *target = $name::$variant(value);

                            return $crate::serde::codec_internals::anyhow::Result::Ok(());
                        }
                        type_id -= 1;
                    )*

                    $crate::serde::codec_internals::anyhow::bail!(
                        "unknown union variant {type_id}",
                    )
                }

                fn encode(
                    &self,
                    target: &$name,
                    buf: &mut $crate::serde::codec_internals::BytesMut,
                ) -> $crate::serde::codec_internals::anyhow::Result<()> {
                    match target {
                        $($name::$variant(target) => {
                            $crate::serde::codec_internals::Codec::encode(
                                &$crate::serde::codec_internals::codec!(
                                    @pick_first
                                    $({ $custom })?
                                    { <$ty as $crate::serde::codec_internals::Serde>::codec() }
                                ),
                                target,
                                buf,
                            )
                        })*
                    }
                }
            }

            impl $crate::serde::codec_internals::Serde for $name {
                fn build_codec() -> $crate::serde::codec_internals::ErasedCodec<Self> {
                    $crate::serde::codec_internals::ErasedCodec::new(UnionCodec)
                }
            }
        };
    };
    (
        @internal

        $(#[$($meta:tt)*])*
        $vis:vis enum $name:ident {
            $(
                $(#[$($variant_meta:tt)*])*
                $variant:ident
            ),*
            $(,)?
        }
    ) => {
        #[derive(
            $crate::serde::codec_internals::Debug,
            $crate::serde::codec_internals::Copy,
            $crate::serde::codec_internals::Clone,
            $crate::serde::codec_internals::Hash,
            $crate::serde::codec_internals::Eq,
            $crate::serde::codec_internals::PartialEq,
        )]
        $(#[$($meta)*])*
        #[repr(u8)]
        $vis enum $name {
            $(
                $(#[$($variant_meta)*])*
                $variant,
            )*
        }

        impl $crate::serde::codec_internals::Default for $name {
            fn default() -> Self {
                <Self as $crate::serde::codec_internals::Ordinalize>::from_ordinal(0).unwrap()
            }
        }

        impl $crate::serde::codec_internals::Ordinalize for $name {
            type VariantType = $crate::serde::codec_internals::u8;

            const VARIANT_COUNT: $crate::serde::codec_internals::usize = 0
                $(+ { $crate::serde::codec_internals::codec!(@ignore $variant); 1 })*;

            const VARIANTS: &'static [Self] = &[
                $(Self::$variant),*
            ];

            const VALUES: &'static [Self::VariantType] = &[
                $(Self::$variant as $crate::serde::codec_internals::u8),*
            ];

            unsafe fn from_ordinal_unsafe(number: Self::VariantType) -> Self {
                unsafe {
                    $crate::serde::codec_internals::transmute(number)
                }
            }

            fn from_ordinal(number: Self::VariantType) -> $crate::serde::codec_internals::Option<Self>
                where Self: $crate::serde::codec_internals::Sized,
            {
                if (number as $crate::serde::codec_internals::usize) < Self::VARIANT_COUNT {
                    $crate::serde::codec_internals::Option::Some(unsafe {
                        Self::from_ordinal_unsafe(number)
                    })
                } else {
                    $crate::serde::codec_internals::Option::None
                }
            }

            fn ordinal(&self) -> Self::VariantType {
                *self as $crate::serde::codec_internals::u8
            }
        }

        impl $crate::serde::codec_internals::Serde for $name {
            const OPTION_IS_FIXED: $crate::serde::codec_internals::bool = true;

            fn build_codec() -> $crate::serde::codec_internals::ErasedCodec<Self> {
                $crate::serde::codec_internals::Codec::erase(
                    $crate::serde::codec_internals::EnumCodec::<Self>::new(),
                )
            }
        }
    };
    (@ignore $($stuff:tt)*) => {};
    (@pick_first { $($first:tt)* } $({ $($remainder:tt)* })*) => { $($first)* };
}

pub use codec;
