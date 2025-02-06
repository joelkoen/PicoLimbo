use crate::packets::configuration::data::known_pack::KnownPack;
use protocol::prelude::*;

#[derive(Debug, PacketIn)]
#[packet_id(0x07, "configuration/server/minecraft:select_known_packs")]
pub struct ClientKnownPacksPacket {
    known_packs: LengthPaddedVec<KnownPack>,
}

impl Default for ClientKnownPacksPacket {
    fn default() -> Self {
        Self {
            known_packs: vec![KnownPack::default()].into(),
        }
    }
}
