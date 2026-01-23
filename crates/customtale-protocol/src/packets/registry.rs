use bytes::{Bytes, BytesMut};
use std::fmt;

use crate::serde::Serde;

bitflags::bitflags! {
    #[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
    pub struct PacketCategory: u32 {
        const ASSETS = 1 << 0;
        const AUTH = 1 << 1;
        const CONNECTION = 1 << 2;
        const SETUP = 1 << 3;
    }
}

pub trait Packet: Into<AnyPacket> + fmt::Debug + Clone + Serde {
    const DESCRIPTOR: &'static PacketDescriptor;
}

#[derive(Debug, Clone)]
pub struct PacketDescriptor {
    pub name: &'static str,
    pub id: u32,
    pub is_compressed: bool,
    pub max_size: u32,
    pub category: PacketCategory,
}

impl fmt::Display for PacketDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.name, self.id)
    }
}

macro_rules! define_packets {
    (
        $(
            $mod:ident [
                $($name:ident),*$(,)?
            ]
        ),*
        $(,)?
    ) => {
        #[derive(Debug, Clone)]
        pub enum AnyPacket {
            $($(
                $name (super::$mod::$name),
            )*)*
        }

        impl AnyPacket {
            pub const fn descriptor_for(id: u32) -> Option<&'static PacketDescriptor> {
                $($(
                    if id == <super::$mod::$name as Packet>::DESCRIPTOR.id {
                        return Some(<super::$mod::$name as Packet>::DESCRIPTOR);
                    }
                )*)*

                None
            }

            pub fn decode(id: u32, contents: Bytes) -> anyhow::Result<Self> {
                $($(
                    if id == <super::$mod::$name as Packet>::DESCRIPTOR.id {
                        return <super::$mod::$name as Serde>::decode(contents).map(Into::into);
                    }
                )*)*

                anyhow::bail!("unknown packet id {id:?}")
            }

            pub fn descriptor(&self) -> &'static PacketDescriptor {
                match self {
                    $($(Self::$name(_) => <super::$mod::$name as Packet>::DESCRIPTOR,)*)*
                }
            }

            pub fn encode(&self, target: &mut BytesMut) -> anyhow::Result<()> {
                match self {
                    $($(Self::$name(v) => Serde::encode(v, target),)*)*
                }
            }
        }

        $($(
            impl From<super::$mod::$name> for AnyPacket {
                fn from(packet: super::$mod::$name) -> Self {
                    AnyPacket::$name(packet)
                }
            }
        )*)*
    };
}

define_packets! {
    assets [
        TrackOrUpdateObjective,
        UntrackObjective,
        UpdateAmbienceFX,
        UpdateAudioCategories,
        UpdateBlockBreakingDecals,
        UpdateBlockGroups,
        UpdateBlockHitboxes,
    ],
    auth [
        AuthGrant,
        AuthToken,
        ServerAuthToken,
    ],
    connection [
        Connect,
        Disconnect,
    ],
    setup [
        AssetFinalize,
        AssetInitialize,
        PlayerOptions,
        RemoveAssets,
        RequestAssets,
        RequestCommonAssetsRebuild,
        ServerTags,
        SetTimeDilation,
        SetUpdateRate,
        UpdateFeatures,
        ViewRadius,
        WorldLoadProgress,
        WorldLoadFinished,
        WorldSettings,
    ]
}
