use std::io;

use bytes::{Buf, BufMut};
use customtale_protocol::packets::{AnyPacket, PacketCategory, PacketDescriptor};
use thiserror::Error;
use tokio_util::codec::{Decoder, Encoder};

#[derive(Debug, Error)]
pub enum HytaleEncodeError {
    #[error("an underlying IO error occurred")]
    Io(#[from] io::Error),
    #[error("encoding failed")]
    Encode(#[from] anyhow::Error),
}

#[derive(Debug, Clone, Default)]
pub struct HytaleEncoder;

impl Encoder<AnyPacket> for HytaleEncoder {
    type Error = HytaleEncodeError;

    fn encode(&mut self, item: AnyPacket, dst: &mut bytes::BytesMut) -> Result<(), Self::Error> {
        dst.put_u32(item.descriptor().id);

        let len_offset = dst.len();
        dst.put_u32(u32::MAX);

        let start = dst.len();
        item.encode(dst)?;

        let data_len = dst.len() - start;
        dst[len_offset..][..4].copy_from_slice(&(data_len as u32).to_le_bytes());

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum HytaleDecodeError {
    #[error("an underlying IO error occurred")]
    Io(
        #[from]
        #[source]
        io::Error,
    ),
    #[error("unknown packet ID {0}")]
    UnknownId(u32),
    #[error(
        "packet {descriptor} in category {:?} is not yet accepted by filter {allowed:?}",
        descriptor.category,
    )]
    DeniedCategory {
        descriptor: &'static PacketDescriptor,
        allowed: PacketCategory,
    },
    #[error(
        "packet is {received} byte{} in length but {descriptor} only permits packets up to {} byte{} long",
        if *received == 1 { "" } else { "s" },
        descriptor.max_size,
        if descriptor.max_size == 1 { "" } else { "s" },
    )]
    TooLong {
        descriptor: &'static PacketDescriptor,
        received: u32,
    },
    #[error("decoding of {descriptor} failed")]
    Decode {
        descriptor: &'static PacketDescriptor,
        #[source]
        error: anyhow::Error,
    },
}

#[derive(Debug, Clone)]
pub struct HytaleDecoder {
    pub allowed_categories: PacketCategory,
}

impl Decoder for HytaleDecoder {
    type Item = AnyPacket;
    type Error = HytaleDecodeError;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 8 {
            return Ok(None);
        }

        let packet_id = src.get_u32_le();
        let packet_len = src.get_u32_le();

        let descriptor =
            AnyPacket::descriptor_for(packet_id).ok_or(HytaleDecodeError::UnknownId(packet_id))?;

        if !self.allowed_categories.contains(descriptor.category) {
            return Err(HytaleDecodeError::DeniedCategory {
                descriptor,
                allowed: self.allowed_categories,
            });
        }

        if packet_len > descriptor.max_size {
            return Err(HytaleDecodeError::TooLong {
                descriptor,
                received: packet_len,
            });
        }

        if src.remaining() < packet_len as usize {
            return Ok(None);
        }

        let packet = src.split_off(packet_len as usize).freeze();
        let packet = AnyPacket::decode(packet_id, packet)
            .map_err(|error| HytaleDecodeError::Decode { descriptor, error })?;

        Ok(Some(packet))
    }
}
