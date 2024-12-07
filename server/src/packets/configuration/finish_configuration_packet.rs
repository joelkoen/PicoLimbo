use protocol::prelude::*;

#[derive(Debug, PacketOut)]
#[packet_id(0x03)]
pub struct FinishConfigurationPacket {}
