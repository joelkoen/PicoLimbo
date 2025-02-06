use protocol::prelude::*;

#[derive(Debug, PacketIn)]
#[packet_id(0x03, "configuration/server/minecraft:finish_configuration")]
pub struct AcknowledgeConfigurationPacket {}
