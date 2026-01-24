#[doc(hidden)]
pub mod codec_internals {
    pub use {
        super::codec,
        crate::serde::{Codec, EnumCodec, ErasedCodec, Serde, StructCodec, field},
        enum_ordinalize::Ordinalize,
        std::{
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
