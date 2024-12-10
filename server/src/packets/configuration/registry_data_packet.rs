use crate::packets::configuration::data::registry_entry::RegistryEntry;
use protocol::prelude::*;

#[derive(Debug, PacketOut)]
#[packet_id(0x07)]
pub struct RegistryDataPacket {
    pub registry_id: Identifier,
    pub entries: LengthPaddedVec<RegistryEntry>,
}
