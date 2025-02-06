use protocol::prelude::*;

#[derive(Debug, PacketOut)]
#[packet_id(0x03, "configuration/client/minecraft:finish_configuration")]
pub struct FinishConfigurationPacket {}
