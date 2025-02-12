use minecraft_protocol::prelude::*;

#[derive(Debug, PacketIn)]
#[packet_id("configuration/serverbound/minecraft:finish_configuration")]
pub struct AcknowledgeConfigurationPacket {}
