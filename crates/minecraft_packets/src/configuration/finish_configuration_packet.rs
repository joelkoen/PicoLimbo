use minecraft_protocol::prelude::*;

#[derive(Debug, PacketOut)]
#[packet_id("configuration/clientbound/minecraft:finish_configuration")]
pub struct FinishConfigurationPacket {}
