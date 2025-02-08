use protocol::prelude::*;

#[derive(Debug, PacketOut)]
#[packet_id(0x03, "configuration/clientbound/minecraft:finish_configuration")]
pub struct FinishConfigurationPacket {}
