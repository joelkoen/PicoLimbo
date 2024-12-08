use crate::packets::configuration::data::known_pack::KnownPack;
use protocol::prelude::*;

#[derive(Debug, PacketOut)]
#[packet_id(0x0E)]
pub struct ClientBoundKnownPacksPacket {
    pub known_packs: Vec<KnownPack>,
}

impl Default for ClientBoundKnownPacksPacket {
    fn default() -> Self {
        Self {
            known_packs: vec![KnownPack::default()],
        }
    }
}
