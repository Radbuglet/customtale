use std::io;

use bytes::{Buf, BufMut, BytesMut};
use customtale_protocol::packets::{AnyPacket, PacketCategory, PacketDescriptor};
use thiserror::Error;
use tokio_util::codec::{Decoder, Encoder};

#[derive(Debug, Error)]
pub enum HytaleEncodeError {
    #[error("an underlying IO error occurred")]
    Io(#[from] io::Error),
    #[error("compression failed with Zstd error {0}")]
    Compress(usize),
    #[error("encoding failed")]
    Encode(#[from] anyhow::Error),
}

#[derive(Debug, Clone, Default)]
pub struct HytaleEncoder;

impl Encoder<AnyPacket> for HytaleEncoder {
    type Error = HytaleEncodeError;

    fn encode(&mut self, item: AnyPacket, dst: &mut bytes::BytesMut) -> Result<(), Self::Error> {
        // Write header
        let header_len_offset = dst.len();
        dst.put_u32_le(u32::MAX);
        dst.put_u32_le(item.descriptor().id);

        // Write payload
        let start = dst.len();

        if item.descriptor().is_compressed {
            let mut uncompressed = BytesMut::new();
            item.encode(&mut uncompressed)?;

            dst.put_bytes(0xFF, zstd_safe::compress_bound(uncompressed.len()));

            let compressed_len =
                zstd_safe::compress(&mut dst[..], &uncompressed, zstd_safe::CLEVEL_DEFAULT)
                    .map_err(HytaleEncodeError::Compress)?;

            dst.truncate(start + compressed_len);
        } else {
            item.encode(dst)?;
        }

        let packet_len = dst.len() - start;

        // Adjust header
        dst[header_len_offset..][..4].copy_from_slice(&(packet_len as u32).to_le_bytes());

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
    #[error("failed to get decompressed content size for {descriptor}")]
    DecompressContentSize {
        descriptor: &'static PacketDescriptor,
    },
    #[error("failed to get decompress contents of {descriptor}: {code}")]
    DecompressContents {
        descriptor: &'static PacketDescriptor,
        code: zstd_safe::ErrorCode,
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

        let packet_len = src.get_u32_le();
        let packet_id = src.get_u32_le();

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

        let packet = src.split_to(packet_len as usize).freeze();

        let packet = if descriptor.is_compressed {
            let size = zstd_safe::get_frame_content_size(&packet)
                .and_then(|v| v.ok_or(zstd_safe::ContentSizeError))
                .map_err(|_| HytaleDecodeError::DecompressContentSize { descriptor })?;

            if size > descriptor.max_size as u64 {
                return Err(HytaleDecodeError::TooLong {
                    descriptor,
                    received: packet_len,
                });
            }

            let mut target = BytesMut::new();
            target.put_bytes(0, size as usize);

            zstd_safe::decompress(&mut target[..], &packet)
                .map_err(|code| HytaleDecodeError::DecompressContents { descriptor, code })?;

            target.freeze()
        } else {
            packet
        };

        let packet = AnyPacket::decode(packet_id, packet)
            .map_err(|error| HytaleDecodeError::Decode { descriptor, error })?;

        Ok(Some(packet))
    }
}
