use crate::packets::configuration::data::known_pack::KnownPack;
use protocol::prelude::*;

#[derive(Debug, PacketIn)]
#[packet_id(0x07)]
pub struct ClientKnownPacksPacket {
    known_packs: Vec<KnownPack>,
}

impl Default for ClientKnownPacksPacket {
    fn default() -> Self {
        Self {
            known_packs: vec![KnownPack::default()],
        }
    }
}
