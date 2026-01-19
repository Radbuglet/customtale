use bytes::Bytes;

use crate::{
    field,
    serde::{Codec as _, ErasedCodec, Serde, StructCodec, VarByteArrayCodec},
};

#[derive(Debug, Clone, Default)]
pub struct ConnectAccept {
    pub password_challenge: Option<Bytes>,
}

impl Serde for ConnectAccept {
    fn build_codec() -> ErasedCodec<Self> {
        StructCodec::new([VarByteArrayCodec::new(64)
            .nullable_variable()
            .map(field![ConnectAccept, password_challenge])
            .named("password_challenge")])
        .erase()
    }
}
